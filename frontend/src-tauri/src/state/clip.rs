use serde::{Deserialize, Serialize};

/// 録音されたオーディオクリップのデータを保持する構造体
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioClip {
    pub id: usize,
    pub name: String,
    pub start_pos: f32,
    pub length: f32,
    pub waveform_summary: Vec<f32>,
}

/// MIDIクリップのデータを保持する構造体
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MidiClip {
    pub id: usize,
    pub name: String,
    pub start_beat: f64,
    pub length_beats: f64,
    pub sequence: crate::midi::sequence::Sequence,
}
