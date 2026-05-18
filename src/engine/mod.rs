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

// Phase 2 で実装予定
// pub mod device;
// pub mod channel;
// pub mod audio_file;
// pub mod stream;
// pub mod mixer;
// pub mod effects;
// pub mod export;

/// オーディオエンジンのエントリポイント
#[derive(Default)]
pub struct AudioEngine {
    /// 現在選択されているオーディオデバイスの名前
    device_name: Option<String>,
}

impl AudioEngine {
    /// 新しいAudioEngineインスタンスを作成する
    pub fn new() -> Self {
        Self {
            device_name: None,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_engine_device_selection() {
        let mut engine = AudioEngine::new();

        // 初期状態ではデバイスはNone
        assert_eq!(engine.device_name(), None);

        // デバイス名を設定
        engine.set_device(Some("MacBook Pro Speakers".to_string()));
        assert_eq!(engine.device_name(), Some("MacBook Pro Speakers"));

        // デバイス名をクリア
        engine.set_device(None);
        assert_eq!(engine.device_name(), None);
    }
}
