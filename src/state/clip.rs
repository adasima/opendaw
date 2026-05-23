//! オーディオクリップの状態管理モジュール
//!
//! 録音されたオーディオデータのメタデータ（長さ、波形サマリー等）を保持する。

use serde::{Deserialize, Serialize};

/// 録音されたオーディオクリップのデータを保持する構造体
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioClip {
    /// クリップの一意なID
    pub id: usize,
    /// クリップ名
    pub name: String,
    /// タイムライン上の開始位置 (秒など)
    pub start_pos: f32,
    /// クリップの長さ
    pub length: f32,
    /// 波形描画用のサマリーデータ（ピーク値の配列等）
    pub waveform_summary: Vec<f32>,
}

impl AudioClip {
    /// 新しいオーディオクリップを作成します。
    pub fn new(id: usize, name: impl Into<String>, start_pos: f32, length: f32) -> Self {
        Self {
            id,
            name: name.into(),
            start_pos,
            length,
            waveform_summary: Vec::new(),
        }
    }

    /// 波形サマリーのデータを設定します。
    pub fn set_waveform_summary(&mut self, summary: Vec<f32>) {
        self.waveform_summary = summary;
    }
}

/// MIDIクリップのデータを保持する構造体
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MidiClip {
    /// クリップの一意なID
    pub id: usize,
    /// クリップ名
    pub name: String,
    /// タイムライン上の開始位置 (拍単位など)
    pub start_beat: f64,
    /// クリップの長さ (拍単位など)
    pub length_beats: f64,
    /// ノート列データ
    pub sequence: crate::midi::sequence::Sequence,
}

impl MidiClip {
    /// 新しいMIDIクリップを作成します。
    pub fn new(id: usize, name: impl Into<String>, start_beat: f64, length_beats: f64) -> Self {
        Self {
            id,
            name: name.into(),
            start_beat,
            length_beats,
            sequence: crate::midi::sequence::Sequence::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_clip_new() {
        let clip = AudioClip::new(1, "Vocal Take 1", 0.0, 10.5);
        assert_eq!(clip.id, 1);
        assert_eq!(clip.name, "Vocal Take 1");
        assert_eq!(clip.start_pos, 0.0);
        assert_eq!(clip.length, 10.5);
        assert!(clip.waveform_summary.is_empty());
    }

    #[test]
    fn test_audio_clip_set_waveform_summary() {
        let mut clip = AudioClip::new(1, "Guitar", 5.0, 2.0);
        clip.set_waveform_summary(vec![0.1, 0.5, 0.8, 0.3]);
        assert_eq!(clip.waveform_summary.len(), 4);
        assert_eq!(clip.waveform_summary[2], 0.8);
    }

    #[test]
    fn test_midi_clip_new() {
        let clip = MidiClip::new(1, "Synth Melody", 0.0, 4.0);
        assert_eq!(clip.id, 1);
        assert_eq!(clip.name, "Synth Melody");
        assert_eq!(clip.start_beat, 0.0);
        assert_eq!(clip.length_beats, 4.0);
        assert_eq!(clip.sequence.notes.len(), 0);
    }
}
