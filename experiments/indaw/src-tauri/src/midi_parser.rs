use midly::{Smf, TrackEventKind, MidiMessage, MetaMessage, Timing, num::u15};
use anyhow::{Result, Context};
use std::fs;
use crate::sequence::NoteEvent;
use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct MidiMetadata {
    pub file_name: String,
    pub track_count: usize,
    pub ppq: u16,
    pub duration: f64,
    pub initial_bpm: f64,
    pub time_signatures: Vec<TimeSignatureInfo>,
    pub labels: Vec<String>, // Track names
}

#[derive(Debug, Serialize, Clone)]
pub struct TimeSignatureInfo {
    pub time: f64, // seconds
    pub numerator: u8,
    pub denominator: u8,
}

#[derive(Debug, Serialize, Clone)]
pub struct ParsedTrack {
    pub name: String,
    pub notes: Vec<NoteEvent>,
    pub program: Option<u8>, // Program Change (0-127)
    pub is_drum: bool,
}

pub struct ParsingOptions {
    pub bake_sustain: bool,
}

impl Default for ParsingOptions {
    fn default() -> Self {
        Self { bake_sustain: true }
    }
}

// Internal struct for Tempo Map
struct TempoChange {
    tick: u64,
    microseconds_per_beat: u32,
}

struct TimeSigChange {
    tick: u64,
    numerator: u8,
    denominator: u8,
}

struct TempoMap {
    tempos: Vec<TempoChange>,
    ppq: f64,
}

impl TempoMap {
    fn new(ppq: u15) -> Self {
        Self {
            tempos: Vec::new(),
            ppq: ppq.as_int() as f64,
        }
    }

    fn add(&mut self, tick: u64, mpb: u32) {
        self.tempos.push(TempoChange { tick, microseconds_per_beat: mpb });
    }

    fn finish(&mut self) {
        self.tempos.sort_by_key(|t| t.tick);
    }

    fn tick_to_seconds(&self, tick: u64) -> f64 {
        let mut current_tick = 0;
        let mut current_time = 0.0;
        let mut current_tempo = 500_000.0; // Default 120 BPM

        for change in &self.tempos {
            if change.tick > tick {
                break;
            }
            if change.tick > current_tick {
                let delta = (change.tick - current_tick) as f64;
                current_time += (delta / self.ppq) * (current_tempo / 1_000_000.0);
                current_tick = change.tick;
            }
            current_tempo = change.microseconds_per_beat as f64;
        }

        if tick > current_tick {
            let delta = (tick - current_tick) as f64;
            current_time += (delta / self.ppq) * (current_tempo / 1_000_000.0);
        }

        current_time
    }
}

pub fn load_midi_file(path: &str, options: Option<ParsingOptions>) -> Result<(MidiMetadata, Vec<ParsedTrack>)> {
    let bytes = fs::read(path).context("Failed to read MIDI file")?;
    let smf = Smf::parse(&bytes).context("Failed to parse MIDI file")?;

    let opts = options.unwrap_or_default();

    let ppq = match smf.header.timing {
        Timing::Metrical(n) => n,
        _ => anyhow::bail!("Unsupported time format (must be metrical)"),
    };

    // 1. Build Tempo Map & Time Signature Map (from all tracks, though usually Track 0)
    let mut tempo_map = TempoMap::new(ppq);
    let mut time_sig_changes: Vec<TimeSigChange> = Vec::new();
    let mut track_names: Vec<String> = vec![String::new(); smf.tracks.len()];
    
    // First pass mainly for global meta events
    for (i, track) in smf.tracks.iter().enumerate() {
        let mut current_tick = 0;
        for event in track {
            current_tick += event.delta.as_int() as u64;
            if let TrackEventKind::Meta(meta) = event.kind {
                match meta {
                    MetaMessage::Tempo(t) => {
                        tempo_map.add(current_tick, t.as_int());
                    }
                    MetaMessage::TimeSignature(n, d, _, _) => {
                         time_sig_changes.push(TimeSigChange {
                             tick: current_tick,
                             numerator: n,
                             denominator: 2u8.pow(d as u32),
                         });
                    }
                    MetaMessage::TrackName(text) => {
                        if let Ok(name) = std::str::from_utf8(text) {
                            if track_names[i].is_empty() {
                                track_names[i] = name.to_string();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    tempo_map.finish();

    // Convert TimeSignatures to seconds for metadata
    let mut time_signatures_info = Vec::new();
    for ts in time_sig_changes {
        time_signatures_info.push(TimeSignatureInfo {
            time: tempo_map.tick_to_seconds(ts.tick),
            numerator: ts.numerator,
            denominator: ts.denominator,
        });
    }

    // 2. Parse Notes per Track
    let mut parsed_tracks = Vec::new();

    for (i, track) in smf.tracks.into_iter().enumerate() {
        let mut notes = Vec::new();
        let mut active_notes: [[Option<(f64, u8)>; 128]; 16] = [[None; 128]; 16]; // [channel][note] -> (start_time, velocity)
        let mut sustain_active: [bool; 16] = [false; 16];
        let mut pending_off: [Vec<u8>; 16] = Default::default(); // Notes waiting for sustain release

        let mut current_tick = 0;
        let mut program: Option<u8> = None;
        let mut is_drum = false; // Simple heuristic: Channel 9 (10)

        // Track name fallback
        let track_name = if !track_names[i].is_empty() {
            track_names[i].clone()
        } else {
            format!("Track {}", i + 1)
        };

        for event in track {
            current_tick += event.delta.as_int() as u64;
            let time_sec = tempo_map.tick_to_seconds(current_tick);

            match event.kind {
                TrackEventKind::Midi { channel: ch_u4, message } => {
                    let ch = ch_u4.as_int() as usize;
                    if ch == 9 { is_drum = true; } // Channel 10 is drum

                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            let note = key.as_int();
                            let velocity = vel.as_int();

                            if velocity > 0 {
                                // Note On
                                active_notes[ch][note as usize] = Some((time_sec, velocity));
                                // Make sure it's not pending off (retrigger logic)
                                if let Some(idx) = pending_off[ch].iter().position(|&n| n == note) {
                                    pending_off[ch].remove(idx);
                                }
                            } else {
                                // Note On with vel 0 => Note Off
                                handle_note_off(
                                    ch, note, time_sec,
                                    &mut active_notes, &mut notes, &mut sustain_active, &mut pending_off, opts.bake_sustain
                                );
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                             let note = key.as_int();
                             handle_note_off(
                                ch, note, time_sec,
                                &mut active_notes, &mut notes, &mut sustain_active, &mut pending_off, opts.bake_sustain
                            );
                        }
                        MidiMessage::Controller { controller, value } => {
                            if controller.as_int() == 64 {
                                let val = value.as_int();
                                let pedal_down = val >= 64;
                                
                                if sustain_active[ch] && !pedal_down {
                                    // Sustain Release
                                    sustain_active[ch] = false;
                                    // Process pending offs
                                    let pending = pending_off[ch].clone();
                                    pending_off[ch].clear();
                                    for note in pending {
                                        // Force end note now
                                        if let Some((start, velocity)) = active_notes[ch][note as usize] {
                                            notes.push(NoteEvent {
                                                note,
                                                velocity,
                                                start_time: start,
                                                duration: time_sec - start,
                                            });
                                            active_notes[ch][note as usize] = None;
                                        }
                                    }
                                } else if !sustain_active[ch] && pedal_down {
                                    sustain_active[ch] = true;
                                }
                            }
                        }
                        MidiMessage::ProgramChange { program: p } => {
                            if program.is_none() {
                                program = Some(p.as_int());
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        // End remaining notes at end of track
        let end_time = tempo_map.tick_to_seconds(current_tick);
        for ch in 0..16 {
             for note in 0..128 {
                 if let Some((start, velocity)) = active_notes[ch][note] {
                     notes.push(NoteEvent {
                         note: note as u8,
                         velocity,
                         start_time: start,
                         duration: end_time - start,
                     });
                 }
             }
        }

        // Only add tracks with notes
        if !notes.is_empty() {
             notes.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
             parsed_tracks.push(ParsedTrack {
                 name: track_name,
                 notes,
                 program,
                 is_drum,
             });
        }
    }

    let initial_bpm = if !tempo_map.tempos.is_empty() {
        60_000_000.0 / tempo_map.tempos[0].microseconds_per_beat as f64
    } else {
        120.0
    };
    
    // Calculate total duration
    let max_duration = parsed_tracks.iter().flat_map(|t| t.notes.iter()).map(|n| n.start_time + n.duration).fold(0.0/0.0, f64::max).max(0.0);
    // Handle NaN if no notes
    let duration = if max_duration.is_nan() { 0.0 } else { max_duration };

    let metadata = MidiMetadata {
        file_name: std::path::Path::new(path).file_name().unwrap_or_default().to_string_lossy().to_string(),
        track_count: parsed_tracks.len(),
        ppq: ppq.as_int(),
        duration,
        initial_bpm,
        time_signatures: time_signatures_info,
        labels: parsed_tracks.iter().map(|t| t.name.clone()).collect(),
    };

    Ok((metadata, parsed_tracks))
}

fn handle_note_off(
    ch: usize, note: u8, time_sec: f64,
    active_notes: &mut [[Option<(f64, u8)>; 128]; 16],
    notes: &mut Vec<NoteEvent>,
    sustain_active: &mut [bool; 16],
    pending_off: &mut [Vec<u8>; 16],
    bake_sustain: bool,
) {
    if let Some((start, velocity)) = active_notes[ch][note as usize] {
        if bake_sustain && sustain_active[ch] {
            // Defer note off
            if !pending_off[ch].contains(&note) {
                pending_off[ch].push(note);
            }
        } else {
            // End note
            notes.push(NoteEvent {
                note,
                velocity,
                start_time: start,
                duration: time_sec - start,
            });
            active_notes[ch][note as usize] = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::midi_exporter::export_midi_file;
    use crate::sequence::Sequence;

    #[test]
    fn test_midi_roundtrip() {
        let test_file = "test_roundtrip.mid";
        let notes = vec![
            NoteEvent { note: 60, velocity: 100, start_time: 0.0, duration: 1.0 },
            NoteEvent { note: 64, velocity: 80, start_time: 1.0, duration: 0.5 },
        ];
        let seq = Sequence { notes: notes.clone(), tempo: 120.0 };
        
        // Export
        export_midi_file(&seq, test_file, 120.0).expect("Export failed");

        // Import
        let opts = ParsingOptions { bake_sustain: false };
        let (metadata, tracks) = load_midi_file(test_file, Some(opts)).expect("Load failed");

        // Verify
        assert_eq!(metadata.initial_bpm, 120.0);
        assert!(!tracks.is_empty());
        let track = &tracks[0];
        assert_eq!(track.notes.len(), 2);
        
        let n1 = &track.notes[0];
        assert_eq!(n1.note, 60);
        assert!((n1.start_time - 0.0).abs() < 0.01);
        assert!((n1.duration - 1.0).abs() < 0.01);
        assert_eq!(n1.velocity, 100);

        let n2 = &track.notes[1];
        assert_eq!(n2.note, 64);
        assert!((n2.start_time - 1.0).abs() < 0.01);
        assert!((n2.duration - 0.5).abs() < 0.01);
        assert_eq!(n2.velocity, 80);

        // Clean up
        let _ = std::fs::remove_file(test_file);
    }
}
