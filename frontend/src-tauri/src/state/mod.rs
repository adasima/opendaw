pub mod clip;
pub mod history;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// プロジェクトの全体状態を保持する構造体
pub struct ProjectState {
    pub is_playing: bool,
    pub bpm: f64,
    pub master_volume: f64,
    pub tracks: Vec<Track>,
}

impl Default for ProjectState {
    fn default() -> Self {
        Self {
            is_playing: false,
            bpm: 120.0,
            master_volume: 0.8,
            tracks: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// トラックの情報を保持する構造体
pub struct Track {
    pub id: usize,
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    pub is_muted: bool,
    pub is_solo: bool,
    pub is_record_armed: bool,
    pub clips: Vec<clip::AudioClip>,
    pub midi_clips: Vec<clip::MidiClip>,
}

impl Track {
    /// 新しいトラックを作成する
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            is_record_armed: false,
            clips: Vec::new(),
            midi_clips: Vec::new(),
        }
    }
}
