use midly::{Header, Format, Timing, TrackEvent, TrackEventKind, MidiMessage};
use anyhow::{Result, Context};
use std::fs::File;
use std::io::Write;
use crate::sequence::Sequence;

/// Export a Sequence to a Standard MIDI File (SMF Format 0)
pub fn export_midi_file(sequence: &Sequence, path: &str, tempo_bpm: f64) -> Result<()> {
    let ppq = 480u16; // Pulses Per Quarter note
    
    // Calculate microseconds per beat from BPM
    let microseconds_per_beat = (60_000_000.0 / tempo_bpm) as u32;
    
    // Convert NoteEvents to MIDI events
    let mut midi_events: Vec<(u32, TrackEventKind<'static>)> = Vec::new();
    
    // Add tempo meta event at the beginning
    midi_events.push((0, TrackEventKind::Meta(midly::MetaMessage::Tempo(
        midly::num::u24::new(microseconds_per_beat)
    ))));
    
    // Convert each note to NoteOn and NoteOff events
    for note in &sequence.notes {
        let start_tick = seconds_to_ticks(note.start_time, ppq, tempo_bpm);
        let end_tick = seconds_to_ticks(note.start_time + note.duration, ppq, tempo_bpm);
        let velocity = if note.velocity > 0 { note.velocity } else { 100 }; // Default velocity
        
        // NoteOn
        midi_events.push((start_tick, TrackEventKind::Midi {
            channel: midly::num::u4::new(0),
            message: MidiMessage::NoteOn {
                key: midly::num::u7::new(note.note),
                vel: midly::num::u7::new(velocity),
            }
        }));
        
        // NoteOff
        midi_events.push((end_tick, TrackEventKind::Midi {
            channel: midly::num::u4::new(0),
            message: MidiMessage::NoteOff {
                key: midly::num::u7::new(note.note),
                vel: midly::num::u7::new(0),
            }
        }));
    }
    
    // Sort events by absolute tick time
    midi_events.sort_by_key(|(tick, _)| *tick);
    
    // Convert absolute ticks to delta ticks
    let mut track_events: Vec<TrackEvent<'static>> = Vec::new();
    let mut last_tick = 0u32;
    
    for (abs_tick, kind) in midi_events {
        let delta = abs_tick.saturating_sub(last_tick);
        track_events.push(TrackEvent {
            delta: midly::num::u28::new(delta),
            kind,
        });
        last_tick = abs_tick;
    }
    
    // Add End of Track meta event
    track_events.push(TrackEvent {
        delta: midly::num::u28::new(0),
        kind: TrackEventKind::Meta(midly::MetaMessage::EndOfTrack),
    });
    
    // Create SMF structure
    let _header = Header::new(
        Format::SingleTrack,
        Timing::Metrical(midly::num::u15::new(ppq)),
    );
    
    // Write to file
    let mut output_bytes = Vec::new();
    
    // Write header manually
    output_bytes.extend_from_slice(b"MThd");
    output_bytes.extend_from_slice(&6u32.to_be_bytes()); // Header length
    output_bytes.extend_from_slice(&0u16.to_be_bytes()); // Format 0
    output_bytes.extend_from_slice(&1u16.to_be_bytes()); // 1 track
    output_bytes.extend_from_slice(&ppq.to_be_bytes()); // Ticks per quarter
    
    // Write track
    let track_data = encode_track_events(&track_events)?;
    output_bytes.extend_from_slice(b"MTrk");
    output_bytes.extend_from_slice(&(track_data.len() as u32).to_be_bytes());
    output_bytes.extend_from_slice(&track_data);
    
    // Write to file
    let mut file = File::create(path).context("Failed to create MIDI file")?;
    file.write_all(&output_bytes).context("Failed to write MIDI file")?;
    
    Ok(())
}

fn seconds_to_ticks(seconds: f64, ppq: u16, tempo_bpm: f64) -> u32 {
    let beats = seconds * (tempo_bpm / 60.0);
    (beats * ppq as f64) as u32
}

fn encode_track_events(events: &[TrackEvent<'static>]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    
    for event in events {
        // Encode delta time as variable-length quantity
        encode_vlq(event.delta.as_int(), &mut data);
        
        // Encode event
        match &event.kind {
            TrackEventKind::Midi { channel, message } => {
                match message {
                    MidiMessage::NoteOn { key, vel } => {
                        data.push(0x90 | channel.as_int());
                        data.push(key.as_int());
                        data.push(vel.as_int());
                    }
                    MidiMessage::NoteOff { key, vel } => {
                        data.push(0x80 | channel.as_int());
                        data.push(key.as_int());
                        data.push(vel.as_int());
                    }
                    _ => {}
                }
            }
            TrackEventKind::Meta(meta) => {
                data.push(0xFF);
                match meta {
                    midly::MetaMessage::Tempo(t) => {
                        data.push(0x51);
                        data.push(0x03);
                        let bytes = t.as_int().to_be_bytes();
                        data.extend_from_slice(&bytes[1..4]); // 3 bytes
                    }
                    midly::MetaMessage::EndOfTrack => {
                        data.push(0x2F);
                        data.push(0x00);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    Ok(data)
}

fn encode_vlq(value: u32, output: &mut Vec<u8>) {
    if value == 0 {
        output.push(0);
        return;
    }
    
    let mut bytes = Vec::new();
    let mut v = value;
    
    while v > 0 {
        bytes.push((v & 0x7F) as u8);
        v >>= 7;
    }
    
    bytes.reverse();
    
    for (i, b) in bytes.iter().enumerate() {
        if i < bytes.len() - 1 {
            output.push(b | 0x80);
        } else {
            output.push(*b);
        }
    }
}
