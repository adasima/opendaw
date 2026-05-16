use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct NoteEvent {
    pub note: u8,
    pub velocity: u8,
    pub start_time: f64, // in seconds
    pub duration: f64,   // in seconds
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct Sequence {
    pub notes: Vec<NoteEvent>,
    pub tempo: f64,
}

impl Sequence {
    pub fn new(tempo: f64) -> Self {
        Self {
            notes: Vec::new(),
            tempo,
        }
    }
}
