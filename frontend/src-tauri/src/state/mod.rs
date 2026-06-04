pub mod clip;
pub mod history;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// グリッドスナップの設定を保持する構造体
pub struct GridSettings {
    pub is_enabled: bool,
    pub resolution: u32,
}

impl Default for GridSettings {
    fn default() -> Self {
        Self {
            is_enabled: true,
            resolution: 4, // 1/4 (四分音符) をデフォルト
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// プロジェクトの全体状態を保持する構造体
pub struct ProjectState {
    pub is_playing: bool,
    pub bpm: f64,
    pub master_volume: f64,
    pub grid_settings: GridSettings,
    pub tracks: Vec<Track>,
}

impl Default for ProjectState {
    fn default() -> Self {
        Self {
            is_playing: false,
            bpm: 120.0,
            master_volume: 0.8,
            grid_settings: GridSettings::default(),
            tracks: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// オートメーションポイントの情報を保持する構造体
pub struct AutomationPoint {
    pub id: usize,
    pub time: f64, // タイムライン上の位置(パーセンテージまたは拍)
    pub value: f32, // パラメータの値 (0.0 ~ 1.0 など)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// オートメーショントラックの情報を保持する構造体
pub struct AutomationTrack {
    pub parameter_name: String, // "Volume", "Pan" など
    pub points: Vec<AutomationPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// センドルーティングの情報を保持する構造体
pub struct SendRouting {
    pub target_track_id: usize,
    pub amount: f32,
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
    pub plugins: Vec<String>,
    pub output_routing: Option<usize>,
    pub sends: Vec<SendRouting>,
    pub automations: Vec<AutomationTrack>,
    pub automation_visible: bool,
    pub selected_automation: Option<String>,
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
            plugins: Vec::new(),
            output_routing: None,
            sends: Vec::new(),
            automations: Vec::new(),
            automation_visible: false,
            selected_automation: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_settings_default() {
        let settings = GridSettings::default();
        assert_eq!(settings.is_enabled, true);
        assert_eq!(settings.resolution, 4);
    }

    #[test]
    fn test_project_state_default() {
        let state = ProjectState::default();
        assert_eq!(state.grid_settings.is_enabled, true);
        assert_eq!(state.grid_settings.resolution, 4);
    }
}
