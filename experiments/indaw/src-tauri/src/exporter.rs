use std::sync::atomic::Ordering;
use crate::state::{SharedAudioState, TrackContent};
use hound;
use anyhow::{Context, Result};

pub fn export_mixdown(state: SharedAudioState, output_path: String) -> Result<()> {
    // 1. Determine Export Duration
    // Scan all tracks to find the end time (last note or audio end)
    let tracks = state.tracks.read().unwrap();
    let mut duration_sec = 0.0;
    
    // Also grab synth params once
    let synth_params = state.synth.snapshot();

    for track in tracks.iter() {
        if let Ok(content) = track.content.read() {
            match &**content {
                TrackContent::Midi(seq) => {
                    for note in &seq.notes {
                        let end = note.start_time + note.duration + synth_params.release;
                        if end > duration_sec { duration_sec = end; }
                    }
                },
                TrackContent::Audio(clip) => {
                    if clip.duration_seconds > duration_sec {
                        duration_sec = clip.duration_seconds;
                    }
                }
            }
        }
    }
    
    // Add 1 second tail
    duration_sec += 1.0; 
    
    // 2. Setup Writer (WAV 32-bit Float, 44.1kHz Stereo)
    let sample_rate = 44100;
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: sample_rate as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    
    let mut writer = hound::WavWriter::create(&output_path, spec).context("Failed to create WAV writer")?;
    
    // 3. Render Loop
    // Render time step: 1.0 / 44100.0
    let total_frames = (duration_sec * sample_rate as f64).ceil() as usize;
    let dt = 1.0 / sample_rate as f64;
    
    let mut current_time = 0.0;
    
    // Pre-calculate solo state
    let any_solo = tracks.iter().any(|t| t.soloed.load(Ordering::Relaxed));

    for _ in 0..total_frames {
        let mut mix_l = 0.0;
        let mut mix_r = 0.0;

        for track in tracks.iter() {
             let soloed = track.soloed.load(Ordering::Relaxed);
             let muted = track.muted.load(Ordering::Relaxed);

             if (any_solo && !soloed) || (!any_solo && muted) {
                 continue;
             }

             let vol = f64::from_bits(track.volume.load(Ordering::Relaxed)) as f32;
             let pan = f64::from_bits(track.pan.load(Ordering::Relaxed)) as f32;
             let pan_l = (1.0 - pan).min(1.0).max(0.0);
             let pan_r = (1.0 + pan).min(1.0).max(0.0);

             let mut track_sample = 0.0;

             if let Ok(content) = track.content.read() {
                 match &**content {
                     TrackContent::Midi(seq) => {
                         // Synth Logic (Duplicate of audio_engine for now)
                         // TODO: Refactor shared logic
                         for note in &seq.notes {
                              let note_end = note.start_time + note.duration;
                              let release_end = note_end + synth_params.release;
                              
                              if current_time >= note.start_time && current_time < release_end {
                                  let t = current_time - note.start_time;
                                  let envelope = if t < synth_params.attack {
                                      t / synth_params.attack
                                  } else if t < synth_params.attack + synth_params.decay {
                                      1.0 - (1.0 - synth_params.sustain) * ((t - synth_params.attack) / synth_params.decay)
                                  } else if t < note.duration {
                                      synth_params.sustain
                                  } else {
                                      let release_t = t - note.duration;
                                      if release_t < synth_params.release {
                                          synth_params.sustain * (1.0 - (release_t / synth_params.release))
                                      } else { 0.0 }
                                  };
                                  
                                  let freq = 440.0 * 2.0_f32.powf((note.note as f32 - 69.0) / 12.0);
                                  track_sample += (current_time as f32 * freq * 2.0 * std::f32::consts::PI).sin() * 0.1 * envelope as f32 * (note.velocity as f32 / 127.0);
                              }
                         }
                     },
                     TrackContent::Audio(clip) => {
                         let sample_idx = (current_time * clip.sample_rate as f64).floor() as usize;
                         if dim_check_ok(sample_idx, clip) {
                              if clip.channels == 2 {
                                  let idx = sample_idx * 2;
                                  let l = clip.samples[idx];
                                  let r = clip.samples[idx+1];
                                  track_sample += (l + r) * 0.5;
                              } else {
                                  track_sample += clip.samples[sample_idx];
                              }
                         }
                     }
                 }
             }
             
             mix_l += track_sample * vol * pan_l;
             mix_r += track_sample * vol * pan_r;
        }

        writer.write_sample(mix_l).unwrap();
        writer.write_sample(mix_r).unwrap();
        
        current_time += dt;
    }
    
    writer.finalize().context("Failed to finalize WAV file")?;
    Ok(())
}

fn dim_check_ok(idx: usize, clip: &crate::state::AudioClip) -> bool {
    if clip.channels == 2 {
        idx * 2 + 1 < clip.samples.len()
    } else {
        idx < clip.samples.len()
    }
}
