use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use crate::sequence::Sequence;
use std::sync::RwLock;

// --- Synth Params ---
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub enum OscillatorType {
    Sine = 0,
    Square = 1,
    Sawtooth = 2,
    Triangle = 3,
}

impl From<u8> for OscillatorType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Sine,
            1 => Self::Square,
            2 => Self::Sawtooth,
            3 => Self::Triangle,
            _ => Self::Sine,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct SynthParams {
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
    pub oscillator_type: OscillatorType,
}

pub struct AtomicSynthParams {
    pub attack: AtomicU64,
    pub decay: AtomicU64,
    pub sustain: AtomicU64,
    pub release: AtomicU64,
    pub oscillator_type: std::sync::atomic::AtomicU8,
}

impl AtomicSynthParams {
    pub fn new(attack: f64, decay: f64, sustain: f64, release: f64, oscillator_type: OscillatorType) -> Self {
        Self {
            attack: AtomicU64::new(attack.to_bits()),
            decay: AtomicU64::new(decay.to_bits()),
            sustain: AtomicU64::new(sustain.to_bits()),
            release: AtomicU64::new(release.to_bits()),
            oscillator_type: std::sync::atomic::AtomicU8::new(oscillator_type as u8),
        }
    }

    pub fn snapshot(&self) -> SynthParams {
        SynthParams {
            attack: f64::from_bits(self.attack.load(Ordering::Relaxed)),
            decay: f64::from_bits(self.decay.load(Ordering::Relaxed)),
            sustain: f64::from_bits(self.sustain.load(Ordering::Relaxed)),
            release: f64::from_bits(self.release.load(Ordering::Relaxed)),
            oscillator_type: OscillatorType::from(self.oscillator_type.load(Ordering::Relaxed)),
        }
    }
}

// --- Track System ---

#[derive(Clone, Debug, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct AudioClip {
    pub path: String,
    // We don't serialize the raw samples to frontend usually, too heavy.
    // Frontend loads by path or we send a waveform thumbnail separate.
    #[serde(skip)]
    #[ts(skip)]
    pub samples: Arc<Vec<f32>>, 
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_seconds: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub enum TrackContent {
    Midi(Sequence),
    Audio(AudioClip),
}

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct TrackSnapshot {
    pub id: usize,
    pub name: String,
    pub kind: String, // "Midi" or "Audio"
    pub volume: f64,
    pub pan: f64,
    pub muted: bool,
    pub soloed: bool,
    // For now, we only send Sequence data if it's a MIDI track. 
    // Heavy audio data is skipped in snapshot.
    pub content: Option<TrackContent>, 
}

pub struct AtomicTrack {
    pub id: usize,
    pub name: RwLock<String>,
    pub volume: AtomicU64,
    pub pan: AtomicU64,
    pub muted: AtomicBool,
    pub soloed: AtomicBool,
    pub content: RwLock<Arc<TrackContent>>,
}

impl AtomicTrack {
    pub fn new_midi(id: usize, name: String, sequence: Sequence) -> Self {
        Self {
            id,
            name: RwLock::new(name),
            volume: AtomicU64::new(1.0f64.to_bits()),
            pan: AtomicU64::new(0.0f64.to_bits()),
            muted: AtomicBool::new(false),
            soloed: AtomicBool::new(false),
            content: RwLock::new(Arc::new(TrackContent::Midi(sequence))),
        }
    }

    pub fn snapshot(&self) -> TrackSnapshot {
        let content_guard = self.content.read().unwrap();
        let name_guard = self.name.read().unwrap();
        
        // Clone content for snapshot (careful with heavy audio)
        // For MIDI it is fine. For Audio, we might want to sanitize.
        let content_clone = match &**content_guard {
            TrackContent::Midi(seq) => Some(TrackContent::Midi(seq.clone())),
            TrackContent::Audio(clip) => {
                // Return metadata only? Or full clip struct but samples skipped by serde
                Some(TrackContent::Audio(clip.clone()))
            }
        };

        TrackSnapshot {
            id: self.id,
            name: name_guard.clone(),
            kind: match &**content_guard { TrackContent::Midi(_) => "Midi".into(), _ => "Audio".into() },
            volume: f64::from_bits(self.volume.load(Ordering::Relaxed)),
            pan: f64::from_bits(self.pan.load(Ordering::Relaxed)),
            muted: self.muted.load(Ordering::Relaxed),
            soloed: self.soloed.load(Ordering::Relaxed),
            content: content_clone,
        }
    }
}

// --- Audio State ---

#[derive(Debug, Serialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct AudioStateSnapshot {
    pub is_playing: bool,
    pub playhead_position: f64,
    pub tempo: f64,
    pub is_looping: bool,
    pub loop_start: f64,
    pub loop_end: f64,
    pub synth: SynthParams,
    pub tracks: Vec<TrackSnapshot>,
    pub master_volume: f64,
}

pub struct AtomicAudioState {
    pub is_playing: AtomicBool,
    pub playhead_position: AtomicU64,
    pub tempo: AtomicU64,
    pub is_looping: AtomicBool,
    pub loop_start: AtomicU64,
    pub loop_end: AtomicU64,
    
    pub synth: AtomicSynthParams, // Global synth params for now, could be per track later
    pub master_volume: AtomicU64,
    
    pub tracks: RwLock<Vec<Arc<AtomicTrack>>>,
}

impl AtomicAudioState {
    pub fn new(sequence: Sequence) -> Self {
        // Create initial MIDI track 0
        let track0 = Arc::new(AtomicTrack::new_midi(0, "Inst 1".to_string(), sequence));

        Self {
            is_playing: AtomicBool::new(false),
            playhead_position: AtomicU64::new(0f64.to_bits()),
            tempo: AtomicU64::new(120.0f64.to_bits()),
            is_looping: AtomicBool::new(false),
            loop_start: AtomicU64::new(0f64.to_bits()),
            loop_end: AtomicU64::new(4.0f64.to_bits()),
            
            synth: AtomicSynthParams::new(0.05, 0.1, 0.7, 0.2, OscillatorType::Sine), 
            master_volume: AtomicU64::new(1.0f64.to_bits()),
            
            tracks: RwLock::new(vec![track0]),
        }
    }

    pub fn get_snapshot(&self) -> AudioStateSnapshot {
        let tracks_guard = self.tracks.read().unwrap();
        let track_snapshots: Vec<TrackSnapshot> = tracks_guard.iter().map(|t| t.snapshot()).collect();

        AudioStateSnapshot {
            is_playing: self.is_playing.load(Ordering::Relaxed),
            playhead_position: f64::from_bits(self.playhead_position.load(Ordering::Relaxed)),
            tempo: f64::from_bits(self.tempo.load(Ordering::Relaxed)),
            is_looping: self.is_looping.load(Ordering::Relaxed),
            loop_start: f64::from_bits(self.loop_start.load(Ordering::Relaxed)),
            loop_end: f64::from_bits(self.loop_end.load(Ordering::Relaxed)),
            synth: self.synth.snapshot(),
            master_volume: f64::from_bits(self.master_volume.load(Ordering::Relaxed)),
            tracks: track_snapshots,
        }
    }
}

pub type SharedAudioState = Arc<AtomicAudioState>;
