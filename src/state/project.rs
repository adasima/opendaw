use serde::{Deserialize, Serialize};
use crate::state::DawState;

/// プロジェクトのファイル保存フォーマット
/// `DawState` をラップし、将来のメタデータ（バージョン、作成日時など）を
/// 追加しやすいようにしています。
#[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct ProjectState {
    pub daw_state: DawState,
}

impl ProjectState {
    /// 新しい ProjectState を作成します。
    pub fn new(daw_state: DawState) -> Self {
        Self { daw_state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_state_serialization() {
        // デフォルト状態で作成
        let mut state = ProjectState::default();
        state.daw_state.bpm = 140.0;
        state.daw_state.master_volume = 0.5;
        state.daw_state.add_track("Test Track");

        // 一時的な状態の変更 (これらはシリアライズされないはず)
        state.daw_state.is_playing = true;
        state.daw_state.playhead_pos = 50.0;

        // シリアライズ
        let encoded: Vec<u8> = bincode::serialize(&state).expect("Failed to serialize ProjectState");

        // デシリアライズ
        let decoded: ProjectState = bincode::deserialize(&encoded).expect("Failed to deserialize ProjectState");

        // 保存されるべきデータが復元されているか確認
        assert_eq!(decoded.daw_state.bpm, 140.0);
        assert_eq!(decoded.daw_state.master_volume, 0.5);
        assert_eq!(decoded.daw_state.tracks.len(), 1);
        assert_eq!(decoded.daw_state.tracks[0].name, "Test Track");

        // 一時的な状態はデフォルト値(false, 0.0)に戻っているか確認
        assert!(!decoded.daw_state.is_playing);
        assert_eq!(decoded.daw_state.playhead_pos, 0.0);
    }
}
