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

use crate::midi::sequence::Sequence;

/// DAW のコア状態を管理する構造体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DawState {
    #[serde(skip, default)]
    pub is_playing: bool,
    #[serde(skip, default)]
    pub is_recording: bool,
    pub is_looping: bool,
    pub is_metronome_enabled: bool,
    #[serde(skip, default)]
    pub playhead_pos: f32,
    pub master_volume: f32,
    pub is_muted: bool,
    pub bpm: f32,
    pub tracks: Vec<track::Track>,
    pub next_track_id: usize,
    pub active_sequence: Sequence,
    pub is_grid_enabled: bool,
    pub grid_resolution: u32,
}

impl Default for DawState {
    fn default() -> Self {
        Self {
            is_playing: false,
            is_recording: false,
            is_looping: true,
            is_metronome_enabled: false,
            playhead_pos: 0.0,
            master_volume: 0.8,
            is_muted: false,
            bpm: 120.0,
            tracks: Vec::new(),
            next_track_id: 1,
            active_sequence: Sequence::new(),
            is_grid_enabled: true,
            grid_resolution: 4,
        }
    }
}

impl DawState {
    /// マスターミュートの状態を切り替えます。
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    /// 再生・一時停止の状態を切り替えます。
    pub fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
    }

    /// 録音状態を切り替えます。
    pub fn toggle_recording(&mut self) {
        self.is_recording = !self.is_recording;
    }

    /// 再生を停止し、プレイヘッドの位置を初期化（0.0）します。
    pub fn stop_playback(&mut self) {
        self.is_playing = false;
        self.playhead_pos = 0.0;
    }

    /// ループ再生の有効・無効を切り替えます。
    pub fn toggle_loop(&mut self) {
        self.is_looping = !self.is_looping;
    }

    /// メトロノームの有効・無効を切り替えます。
    pub fn toggle_metronome(&mut self) {
        self.is_metronome_enabled = !self.is_metronome_enabled;
    }

    /// プレイヘッドを指定された位置に移動させます。
    /// 指定位置は `0.0` から `100.0` の範囲にクランプされます。
    pub fn seek_to(&mut self, pos: f32) {
        self.playhead_pos = pos.clamp(0.0, 100.0);
    }

    /// 毎フレーム呼び出され、再生中の場合はプレイヘッドを進めます。
    /// BPMに依存した速度でプレイヘッドの進行を調整します。
    pub fn tick_playback(&mut self) {
        if self.is_playing {
            // BPMに基づいて進行速度を調整 (120 BPM を基準 (1.0) とする)
            self.playhead_pos += 1.0 * (self.bpm / 120.0);
            // 画面端まで行ったらループさせるか停止する処理
            if self.playhead_pos > 100.0 {
                self.playhead_pos = 0.0;
                if !self.is_looping {
                    self.is_playing = false;
                }
            }
        }
    }

    /// 新しいトラックを追加します。
    pub fn add_track(&mut self, name: impl Into<String>) {
        let id = self.next_track_id;
        self.next_track_id += 1;
        self.tracks.push(track::Track::new(id, name));
    }

    /// 指定されたIDのトラックを削除します。
    pub fn remove_track(&mut self, id: usize) {
        self.tracks.retain(|t| t.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seek_to() {
        let mut state = DawState::default();

        state.seek_to(-10.0);
        assert_eq!(state.playhead_pos, 0.0);

        state.seek_to(150.0);
        assert_eq!(state.playhead_pos, 100.0);

        state.seek_to(50.0);
        assert_eq!(state.playhead_pos, 50.0);
    }

    #[test]
    fn test_toggle_playback() {
        let mut state = DawState::default();
        assert!(!state.is_playing);

        state.toggle_playback();
        assert!(state.is_playing);

        state.toggle_playback();
        assert!(!state.is_playing);
    }


    #[test]
    fn test_toggle_recording() {
        let mut state = DawState::default();
        assert!(!state.is_recording);

        state.toggle_recording();
        assert!(state.is_recording);

        state.toggle_recording();
        assert!(!state.is_recording);
    }

    #[test]
    fn test_stop_playback() {

        let mut state = DawState::default();
        state.is_playing = true;
        state.playhead_pos = 50.0;

        state.stop_playback();
        assert!(!state.is_playing);
        assert_eq!(state.playhead_pos, 0.0);
    }

    #[test]
    fn test_toggle_loop() {
        let mut state = DawState::default();
        assert!(state.is_looping);

        state.toggle_loop();
        assert!(!state.is_looping);

        state.toggle_loop();
        assert!(state.is_looping);
    }

    #[test]
    fn test_toggle_metronome() {
        let mut state = DawState::default();
        assert!(!state.is_metronome_enabled);

        state.toggle_metronome();
        assert!(state.is_metronome_enabled);

        state.toggle_metronome();
        assert!(!state.is_metronome_enabled);
    }

    #[test]
    fn test_playback_end_behavior_with_loop() {
        let mut state = DawState::default();
        state.is_playing = true;
        state.is_looping = true;
        state.playhead_pos = 100.0; // 次の tick で 100.0 を超える

        state.tick_playback();

        assert!(state.is_playing);
        assert_eq!(state.playhead_pos, 0.0);
    }

    #[test]
    fn test_playback_end_behavior_without_loop() {
        let mut state = DawState::default();
        state.is_playing = true;
        state.is_looping = false;
        state.playhead_pos = 100.0; // 次の tick で 100.0 を超える

        state.tick_playback();

        assert!(!state.is_playing);
        assert_eq!(state.playhead_pos, 0.0);
    }

    #[test]
    fn test_toggle_mute() {
        let mut state = DawState::default();
        assert!(!state.is_muted);

        state.toggle_mute();
        assert!(state.is_muted);

        state.toggle_mute();
        assert!(!state.is_muted);
    }

    #[test]
    fn test_bpm_affects_playback_speed() {
        let mut state120 = DawState::default();
        state120.is_playing = true;
        state120.bpm = 120.0;
        state120.tick_playback();

        let mut state240 = DawState::default();
        state240.is_playing = true;
        state240.bpm = 240.0;
        state240.tick_playback();

        // 240 BPMの場合は120 BPMの2倍進むはず
        assert_eq!(state120.playhead_pos * 2.0, state240.playhead_pos);
    }

    #[test]
    fn test_add_remove_track() {
        let mut state = DawState::default();
        assert_eq!(state.tracks.len(), 0);

        state.add_track("Track 1");
        assert_eq!(state.tracks.len(), 1);
        assert_eq!(state.tracks[0].name, "Track 1");
        assert_eq!(state.tracks[0].id, 1);

        state.add_track("Track 2");
        assert_eq!(state.tracks.len(), 2);
        assert_eq!(state.tracks[1].name, "Track 2");
        assert_eq!(state.tracks[1].id, 2);

        state.remove_track(1);
        assert_eq!(state.tracks.len(), 1);
        assert_eq!(state.tracks[0].name, "Track 2");
        assert_eq!(state.tracks[0].id, 2);

        // トラック削除後に追加してもIDが重複しないことを確認
        state.add_track("Track 3");
        assert_eq!(state.tracks.len(), 2);
        assert_eq!(state.tracks[1].name, "Track 3");
        assert_eq!(state.tracks[1].id, 3); // len()+1のロジックだと2になってしまう

        state.remove_track(999); // 存在しないID
        assert_eq!(state.tracks.len(), 2);
    }

    #[test]
    fn test_dawstate_sequence() -> Result<(), Box<dyn std::error::Error>> {
        let mut state = DawState::default();
        assert_eq!(state.active_sequence.notes.len(), 0);

        let id = state.active_sequence.add_note(60, 100, 0.0, 1.0);
        assert_eq!(state.active_sequence.notes.len(), 1);
        assert_eq!(id, 0);

        let note = state.active_sequence.get_note(id).ok_or("Note not found")?;
        assert_eq!(note.pitch, 60);
        assert_eq!(note.velocity, 100);
        assert_eq!(note.start_beat, 0.0);
        assert_eq!(note.duration_beats, 1.0);
        Ok(())
    }
}

pub mod freeze;
