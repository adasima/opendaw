//! アプリケーション状態管理モジュール
//!
//! プロジェクト全体の状態（トラック構成、再生位置、設定など）を管理する。
//! UIスレッドとオーディオスレッドの両方から参照される共有状態。

pub mod clip;
pub mod daw;
pub mod freeze;
pub mod project;
pub mod sync;
pub mod track;
pub mod track_clip;
pub mod track_plugin;

// 再エクスポート
pub use clip::MidiClip;
pub use daw::DawState;
pub use project::ProjectState;
pub use track_clip::*;
pub use track_plugin::*;
