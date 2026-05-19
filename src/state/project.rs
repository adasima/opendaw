//! プロジェクトのシリアライズ・保存・読み込み機能

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use crate::state::DawState;
use bincode;

/// プロジェクトの保存・読み込み時のエラー
#[derive(Debug)]
pub enum ProjectError {
    /// IO関連のエラー
    IoError(std::io::Error),
    /// シリアライズ・デシリアライズ関連のエラー
    BincodeError(bincode::Error),
}

impl From<std::io::Error> for ProjectError {
    fn from(err: std::io::Error) -> Self {
        ProjectError::IoError(err)
    }
}

impl From<bincode::Error> for ProjectError {
    fn from(err: bincode::Error) -> Self {
        ProjectError::BincodeError(err)
    }
}

/// DawStateをバイナリとしてファイルに保存します
pub fn save_project<P: AsRef<Path>>(state: &DawState, path: P) -> Result<(), ProjectError> {
    let encoded: Vec<u8> = bincode::serialize(state)?;
    let mut file = File::create(path)?;
    file.write_all(&encoded)?;
    Ok(())
}

/// DAWStateをバイナリファイルから読み込みます
pub fn load_project<P: AsRef<Path>>(path: P) -> Result<DawState, ProjectError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let state: DawState = bincode::deserialize(&buffer)?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::DawState;

    #[test]
    fn test_save_and_load_project() {
        let mut original_state = DawState::default();
        original_state.add_track("Test Track");
        original_state.bpm = 140.0;

        let path = "test_project.bin";

        save_project(&original_state, path).unwrap();

        let loaded_state = load_project(path).unwrap();

        assert_eq!(original_state.bpm, loaded_state.bpm);
        assert_eq!(original_state.tracks.len(), loaded_state.tracks.len());
        assert_eq!(original_state.tracks[0].name, loaded_state.tracks[0].name);

        // テスト用のファイルを削除
        std::fs::remove_file(path).unwrap();
    }
}
