//! アプリケーション状態管理モジュール
//!
//! プロジェクト全体の状態（トラック構成、再生位置、設定など）を管理する。
//! UIスレッドとオーディオスレッドの両方から参照される共有状態。

// Phase 3 で実装予定
// pub mod project;
pub mod clip;
pub use clip::MidiClip;
pub mod project;
pub use project::ProjectState;
pub mod track; // Track 構造体（名前、ボリューム、パン、ミュート、ソロ）

pub mod daw;
pub use daw::*;

pub mod freeze;

pub mod sync;
pub mod track_clip;
pub use track_clip::*;
pub mod track_plugin;
pub use track_plugin::*;
