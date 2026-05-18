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

/// オーディオエンジンの将来のエントリポイント（Phase 2 で実装予定）
pub struct AudioEngine;
