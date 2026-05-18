//! MIDI処理モジュール
//!
//! midir によるMIDIデバイスの接続、
//! MIDIメッセージのパース、シーケンスデータの管理を担当する。

// Phase 4 で実装予定
pub mod device;
pub mod message;
// pub mod sequence;

/// MIDI処理の将来のエントリポイント（Phase 4 で実装予定）
pub struct MidiManager;
