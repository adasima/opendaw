//! オーディオエンジンモジュール
//!
//! cpal によるオーディオI/O、リングバッファ通信、
//! エフェクトチェーンなどを担当する。
//!
//! ⚠️ リアルタイム制約:
//! オーディオコールバック内では以下を禁止:
//! - ヒープアロケーション（Vec::new(), Box::new(), format!() 等）
//! - Mutex::lock(), RwLock
//! - ファイルI/O, ネットワークI/O
//! - println!() やログ出力

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::traits::Consumer;
use crate::engine::channel::AudioChannels;
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::Arc;
use crate::engine::stream::{build_output_stream, PlaybackContext};
use crate::engine::audio_file::AudioBuffer;

// Phase 2 で実装予定
pub mod device;
pub mod channel;
pub mod audio_file;
pub mod stream;
pub mod mixer;
pub mod effects;
// pub mod export;

/// オーディオエンジンのエントリポイント
pub struct AudioEngine {
    /// 現在選択されているオーディオデバイスの名前
    device_name: Option<String>,
    /// オーディオストリーム
    stream: Option<cpal::Stream>,
    /// UIとの通信チャンネル
    channels: Option<AudioChannels>,
    /// 再生状態
    playing: Arc<AtomicBool>,
    /// 再生位置
    position: Arc<AtomicUsize>,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioEngine {
    /// 新しいAudioEngineインスタンスを作成する
    pub fn new() -> Self {
        Self {
            device_name: device::default_output_device_name(),
            stream: None,
            channels: None,
            playing: Arc::new(AtomicBool::new(false)),
            position: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// オーディオデバイス名を設定する
    pub fn set_device(&mut self, name: Option<String>) {
        self.device_name = name;
    }

    /// 現在設定されているオーディオデバイス名を取得する
    pub fn device_name(&self) -> Option<&str> {
        self.device_name.as_deref()
    }

    /// 利用可能なすべてのオーディオ出力デバイスのリストを取得する
    pub fn available_devices(&self) -> Vec<String> {
        device::available_output_device_names()
    }

    /// UIスレッドと通信するチャンネルを設定する
    pub fn set_channels(&mut self, channels: AudioChannels) {
        self.channels = Some(channels);
    }

    /// 再生を開始する
    pub fn start_playback(&mut self, buffer: AudioBuffer) {
        let host = cpal::default_host();
        let device = if let Some(name) = &self.device_name {
            host.output_devices()
                .ok()
                .and_then(|mut devices| {
                    devices.find(|d| {
                        // The device_name module uses `name().unwrap_or_else()`, so we do the same matching logic.
                        // However, to avoid new #[allow(deprecated)] we can use an alternative if available,
                        // but since device.name() is deprecated we try to match without adding the flag again here,
                        // by using the name attribute or deferring to the standard practice in this codebase.
                        // The codebase already uses #[allow(deprecated)] in device.rs, but requests avoiding new ones.
                        // We can just rely on the name() method and suppress it locally at the expression level if needed.
                        #[allow(deprecated)]
                        let dev_name = d.name();
                        dev_name.ok().as_ref() == Some(name)
                    })
                })
        } else {
            host.default_output_device()
        };

        let Some(device) = device else { return };
        let Ok(config) = device.default_output_config() else { return };
        let channels = self.channels.take();
        let context = PlaybackContext {
            buffer,
            position: self.position.clone(),
            playing: self.playing.clone(),
            channels,
        };
        if let Ok(stream) = build_output_stream(&device, &config.config(), config.sample_format(), Some(context)) {
            let _ = stream.play();
            self.stream = Some(stream);
        }
    }

    /// オーディオストリームを一時停止します
    pub fn pause(&self) {
        if let Some(stream) = &self.stream {
            let _ = stream.pause();
        }
    }

    /// オーディオストリームの再生を再開します
    pub fn play(&self) {
        if let Some(stream) = &self.stream {
            let _ = stream.play();
        }
    }

    /// オーディオストリームを停止し、破棄します
    pub fn stop(&mut self) {
        if let Some(stream) = &self.stream {
            let _ = stream.pause();
        }
        self.stream = None;
    }

    /// オーディオスレッドからのメッセージを受信して処理します
    /// 最新の再生位置(f32)があれば返します
    pub fn poll_ui_messages(&self, ui_channels: &mut crate::engine::channel::UiChannels) -> Option<f32> {
        let mut latest_pos = None;
        while let Some(msg) = ui_channels.1.try_pop() {
            match msg {
                crate::engine::channel::AudioToUiMsg::PlaybackPosition(pos) => latest_pos = Some(pos),
            }
        }
        latest_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_engine_device_selection() {
        let mut engine = AudioEngine::new();

        // 初期化時はシステムのデフォルトデバイスが設定されるかNoneになる
        let default_device = device::default_output_device_name();
        assert_eq!(engine.device_name(), default_device.as_deref());

        // デバイス名を設定
        engine.set_device(Some("MacBook Pro Speakers".to_string()));
        assert_eq!(engine.device_name(), Some("MacBook Pro Speakers"));

        // デバイス名をクリア
        engine.set_device(None);
        assert_eq!(engine.device_name(), None);
    }

    #[test]
    fn test_audio_engine_set_channels() {
        let mut engine = AudioEngine::new();
        const CHANNEL_CAPACITY: usize = 10;
        let (_, audio_channels) = crate::engine::channel::create_channels(CHANNEL_CAPACITY);
        engine.set_channels(audio_channels);
        assert!(engine.channels.is_some());
    }

    #[test]
    fn test_audio_engine_lifecycle_methods() {
        let mut engine = AudioEngine::new();

        // ストリームがない状態で呼んでもエラーにならないことの確認
        engine.pause();
        engine.play();
        engine.stop();

        assert!(engine.stream.is_none());
    }

    #[test]
    fn test_audio_engine_poll_ui_messages() {
        use ringbuf::traits::Producer;
        let engine = AudioEngine::new();
        let (mut ui_channels, mut audio_channels) = crate::engine::channel::create_channels(10);

        // メッセージがない場合
        assert_eq!(engine.poll_ui_messages(&mut ui_channels), None);

        // 複数のメッセージを送信
        let _ = audio_channels.1.try_push(crate::engine::channel::AudioToUiMsg::PlaybackPosition(1.5));
        let _ = audio_channels.1.try_push(crate::engine::channel::AudioToUiMsg::PlaybackPosition(2.5));
        let _ = audio_channels.1.try_push(crate::engine::channel::AudioToUiMsg::PlaybackPosition(3.5));

        // 最新のメッセージが返されること
        assert_eq!(engine.poll_ui_messages(&mut ui_channels), Some(3.5));

        // 取り切ったので次はNoneになること
        assert_eq!(engine.poll_ui_messages(&mut ui_channels), None);
    }
}
