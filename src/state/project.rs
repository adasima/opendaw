use crate::state::DawState;
use serde::{Deserialize, Serialize};

/// プロジェクトのファイル保存フォーマット
/// `DawState` をラップし、将来のメタデータ（バージョン、作成日時など）を
/// 追加しやすいようにしています。
#[derive(Serialize, Deserialize, Default)]
pub struct ProjectState {
    pub daw_state: DawState,
}

impl ProjectState {
    /// 新しい ProjectState を作成します。
    pub fn new(daw_state: DawState) -> Self {
        Self { daw_state }
    }

    /// プロジェクト状態をファイルに保存します。
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        bincode::serialize_into(writer, self).map_err(std::io::Error::other)
    }

    /// ファイルからプロジェクト状態を読み込みます。
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        bincode::deserialize_from(reader).map_err(std::io::Error::other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_state_serialization() -> Result<(), Box<dyn std::error::Error>> {
        // デフォルト状態で作成
        let mut state = ProjectState::default();
        state.daw_state.bpm = 140.0;
        state.daw_state.master_volume = 0.5;
        state.daw_state.add_track("Test Track");

        // シンセサイザーの状態を変更
        state.daw_state.tracks[0].synth.is_enabled = true;
        state.daw_state.tracks[0].synth.frequency = 880.0;

        // 一時的な状態の変更 (これらはシリアライズされないはず)
        state.daw_state.is_playing = true;
        state.daw_state.playhead_pos = 50.0;

        // シリアライズ
        let encoded: Vec<u8> = bincode::serialize(&state)?;

        // デシリアライズ
        let decoded: ProjectState = bincode::deserialize(&encoded)?;

        // 保存されるべきデータが復元されているか確認
        assert_eq!(decoded.daw_state.bpm, 140.0);
        assert_eq!(decoded.daw_state.master_volume, 0.5);
        assert_eq!(decoded.daw_state.tracks.len(), 1);
        assert_eq!(decoded.daw_state.tracks[0].name, "Test Track");

        // シンセサイザー状態が復元されているか確認
        assert!(decoded.daw_state.tracks[0].synth.is_enabled);
        assert_eq!(decoded.daw_state.tracks[0].synth.frequency, 880.0);

        // 一時的な状態はデフォルト値(false, 0.0)に戻っているか確認
        assert!(!decoded.daw_state.is_playing);
        assert_eq!(decoded.daw_state.playhead_pos, 0.0);
        Ok(())
    }

    #[test]
    fn test_save_load_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut state = ProjectState::default();
        state.daw_state.bpm = 125.0;
        state.daw_state.add_track("File Track");

        // シンセサイザーの状態を変更
        state.daw_state.tracks[0].synth.is_enabled = true;
        state.daw_state.tracks[0].synth.frequency = 523.25; // C5

        // 一時ファイルへの保存
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_project_state.bincode");

        state.save_to_file(&file_path)?;

        // ファイルからの読み込み
        let loaded_state = ProjectState::load_from_file(&file_path)?;

        assert_eq!(loaded_state.daw_state.bpm, 125.0);
        assert_eq!(loaded_state.daw_state.tracks.len(), 1);
        assert_eq!(loaded_state.daw_state.tracks[0].name, "File Track");

        // シンセサイザー状態が復元されているか確認
        assert!(loaded_state.daw_state.tracks[0].synth.is_enabled);
        assert_eq!(loaded_state.daw_state.tracks[0].synth.frequency, 523.25);

        // 一時ファイルの削除
        let _ = std::fs::remove_file(file_path);
        Ok(())
    }
}
