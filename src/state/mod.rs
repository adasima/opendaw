//! アプリケーション状態管理モジュール
//!
//! プロジェクト全体の状態（トラック構成、再生位置、設定など）を管理する。
//! UIスレッドとオーディオスレッドの両方から参照される共有状態。

// Phase 3 で実装予定
// pub mod project;   // プロジェクトの保存/読み込み
pub mod track;        // Track 構造体（名前、ボリューム、パン、ミュート、ソロ）

/// プロジェクト状態の将来のエントリポイント（Phase 3 で実装予定）
pub struct ProjectState;

/// DAW のコア状態を管理する構造体
pub struct DawState {
    pub is_playing: bool,
    pub is_looping: bool,
    pub playhead_pos: f32,
    pub master_volume: f32,
    pub is_muted: bool,
    pub bpm: f32,
    pub tracks: Vec<track::Track>,
}

impl Default for DawState {
    fn default() -> Self {
        Self {
            is_playing: false,
            is_looping: true,
            playhead_pos: 0.0,
            master_volume: 0.8,
            is_muted: false,
            bpm: 120.0,
            tracks: Vec::new(),
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

    /// 再生を停止し、プレイヘッドの位置を初期化（0.0）します。
    pub fn stop_playback(&mut self) {
        self.is_playing = false;
        self.playhead_pos = 0.0;
    }

    /// ループ再生の有効・無効を切り替えます。
    pub fn toggle_loop(&mut self) {
        self.is_looping = !self.is_looping;
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
}
