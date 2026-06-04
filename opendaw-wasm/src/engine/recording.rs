//! オーディオ録音モジュール
//!
//! マイクなどからのオーディオ入力をキャプチャし、
//! リングバッファを通じて安全にUIスレッドへ引き渡す役割を担う。

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Observer, Producer, Split};
use ringbuf::wrap::caching::CachingCons;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// 録音バッファの初期容量（サンプル数）
const RECORDING_BUFFER_CAPACITY: usize = 960_000;

/// 録音セッションを管理し、キャプチャしたオーディオデータを保持・提供する構造体
pub struct Recorder {
    /// 録音ストリーム
    stream: Option<cpal::Stream>,
    /// オーディオスレッドからデータを受け取るコンシューマー
    consumer: Option<CachingCons<Arc<HeapRb<f32>>>>,
    /// 録音中かどうかを示すフラグ
    is_recording: Arc<AtomicBool>,
}

impl Default for Recorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Recorder {
    /// 新しい `Recorder` インスタンスを作成する
    pub fn new() -> Self {
        Self {
            stream: None,
            consumer: None,
            is_recording: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 録音を開始する
    ///
    /// 指定されたデバイス名が見つからない場合はデフォルトデバイスを使用する
    pub fn start_recording(&mut self, device_name: Option<&str>) -> Result<(), String> {
        let host = cpal::default_host();
        let device = if let Some(name) = device_name {
            host.input_devices().map_err(|e| e.to_string())?.find(|d| {
                #[allow(deprecated)]
                let dev_name = d.name();
                dev_name.ok().as_deref() == Some(name)
            })
        } else {
            host.default_input_device()
        };

        let device = device.ok_or_else(|| "入力デバイスが見つかりません".to_string())?;
        let config = device.default_input_config().map_err(|e| e.to_string())?;

        // 余裕を持った容量を確保 (例: 480_000 * 2 = 960_000 サンプル)
        let capacity = RECORDING_BUFFER_CAPACITY;
        let rb = HeapRb::<f32>::new(capacity);
        let (mut prod, cons) = rb.split();

        self.consumer = Some(cons);
        self.is_recording.store(true, Ordering::Relaxed);
        let is_recording = Arc::clone(&self.is_recording);

        let err_fn = move |_err| {
            // エラーハンドリング (リアルタイムスレッド外だが、安全のためログ等は避けるか最小限にする)
        };

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    if is_recording.load(Ordering::Relaxed) {
                        let _ = prod.push_slice(data);
                    }
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &_| {
                    if is_recording.load(Ordering::Relaxed) {
                        for &sample in data {
                            let f_sample = sample as f32 / i16::MAX as f32;
                            let _ = prod.try_push(f_sample);
                        }
                    }
                },
                err_fn,
                None,
            ),
            _ => return Err("サポートされていないサンプルフォーマットです".to_string()),
        }
        .map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(stream);

        Ok(())
    }

    /// 録音を停止し、ストリームを破棄する
    pub fn stop_recording(&mut self) {
        if let Some(stream) = &self.stream {
            let _ = stream.pause();
        }
        self.is_recording.store(false, Ordering::Relaxed);
        self.stream = None;
    }

    /// バッファに蓄積されたオーディオデータを取得する
    pub fn collect_recorded_data(&mut self) -> Vec<f32> {
        if let Some(cons) = &mut self.consumer {
            let mut data = Vec::with_capacity(cons.occupied_len());
            while let Some(sample) = cons.try_pop() {
                data.push(sample);
            }
            data
        } else {
            Vec::new()
        }
    }

    /// 現在録音中かどうかを返す
    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_initial_state() {
        let recorder = Recorder::new();
        assert!(!recorder.is_recording());
        assert!(recorder.stream.is_none());
    }

    #[test]
    fn test_recorder_stop_without_start() {
        let mut recorder = Recorder::new();
        recorder.stop_recording();
        assert!(!recorder.is_recording());
    }
}
