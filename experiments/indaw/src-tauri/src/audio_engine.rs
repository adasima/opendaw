use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{unbounded, Receiver, Sender};
use anyhow::{Context, Result};
use cpal::SizedSample;
use std::sync::atomic::Ordering;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use crate::state::SharedAudioState;
use crate::sequence::Sequence;
use crate::constants;

#[derive(Debug, Clone, serde::Serialize, ts_rs::TS)]
#[ts(export, export_to = "../src/lib/bindings/")]
pub struct MeterData {
    pub master_peak: [f32; 2], // Left, Right
    pub track_peaks: HashMap<usize, [f32; 2]>,
}

pub enum AudioCommand {
    Play,
    Pause,
    Stop,
    SetVolume(f32),
    SetLooping(bool),
    SetLoopRegion(f64, f64),
    Seek(f64),
    LoadSequence(Sequence),
}

// Wrapper to allow sending Stream across threads
struct SendStream(cpal::Stream);
unsafe impl Send for SendStream {}
unsafe impl Sync for SendStream {}

 
pub struct AudioEngine {
    // External Interface
    pub command_sender: Sender<AudioCommand>,
    pub meter_receiver: Receiver<MeterData>,
    pub playhead_receiver: Receiver<f64>,
    
    // Internal State for Restarting
    cmd_receiver_internal: Receiver<AudioCommand>,
    meter_sender_internal: Sender<MeterData>,
    playhead_sender_internal: Sender<f64>,
    shared_state: SharedAudioState,

    stream: Option<SendStream>, 
    input_stream: Option<SendStream>,

    // Recording (Shared with callback)
    recording_dest: std::sync::Arc<std::sync::Mutex<Option<Sender<Vec<f32>>>>>,
    input_sample_rate: u32,
    input_channels: u16,
}

impl AudioEngine {
    pub fn new(shared_state: SharedAudioState) -> Result<Self> {
        let (cmd_sender, cmd_receiver) = unbounded();
        let (meter_sender, meter_receiver) = unbounded();
        let (playhead_sender, playhead_receiver) = unbounded();
        
        // Recording shared state
        let recording_dest = std::sync::Arc::new(std::sync::Mutex::new(None));

        let mut engine = Self {
            command_sender: cmd_sender,
            meter_receiver,
            playhead_receiver,
            
            cmd_receiver_internal: cmd_receiver,
            meter_sender_internal: meter_sender,
            playhead_sender_internal: playhead_sender,
            shared_state,
            
            stream: None,
            input_stream: None,
            
            recording_dest,
            input_sample_rate: 44100,
            input_channels: 2,
        };
        
        engine.start(None)?;
        // Auto-start monitor
        let _ = engine.start_input(None);

        Ok(engine)
    }
    
    // ... start() implementation is assumed unchanged or included if needed. 
    // Since replacment covers start(), I must include it.
    
    pub fn start(&mut self, device_name: Option<String>) -> Result<()> {
        let host = cpal::default_host();
        let device = if let Some(name) = device_name {
            host.output_devices()?
                .find(|d| d.name().map(|n| n == name).unwrap_or(false))
                .context("Device not found")?
        } else {
            host.default_output_device().context("Failed to find default output device")?
        };

        let config = device.default_output_config()?;
        self.stream = None;

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), self.cmd_receiver_internal.clone(), self.meter_sender_internal.clone(), self.playhead_sender_internal.clone(), self.shared_state.clone()),
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), self.cmd_receiver_internal.clone(), self.meter_sender_internal.clone(), self.playhead_sender_internal.clone(), self.shared_state.clone()),
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), self.cmd_receiver_internal.clone(), self.meter_sender_internal.clone(), self.playhead_sender_internal.clone(), self.shared_state.clone()),
            _ => anyhow::bail!("Unsupported sample format"),
        }?;

        stream.play()?;
        self.stream = Some(SendStream(stream));
        Ok(())
    }

    pub fn start_input(&mut self, device_name: Option<String>) -> Result<()> {
         let host = cpal::default_host();
         let device = if let Some(name) = device_name {
             host.input_devices()?
                 .find(|d| d.name().map(|n| n == name).unwrap_or(false))
                 .context("Input Device not found")?
         } else {
             match host.default_input_device() {
                 Some(d) => d,
                 None => return Ok(()) // No input
             }
         };

         let config = device.default_input_config()?;
         self.input_sample_rate = config.sample_rate().0;
         self.input_channels = config.channels();
         
         let dest = self.recording_dest.clone();
         
         self.input_stream = None;

         let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => run_input::<f32>(&device, &config.into(), dest),
            cpal::SampleFormat::I16 => run_input::<i16>(&device, &config.into(), dest),
            cpal::SampleFormat::U16 => run_input::<u16>(&device, &config.into(), dest),
            _ => anyhow::bail!("Unsupported input sample format"),
         };
         
         if let Ok(s) = stream {
             s.play()?;
             self.input_stream = Some(SendStream(s));
         }

         Ok(())
    }
    
    pub fn start_recording(&mut self, path: String) -> Result<()> {
        let (tx, rx) = unbounded();
        
        // Update shared state to point to new channel
        {
            let mut dest = self.recording_dest.lock().unwrap();
            *dest = Some(tx);
        }
        
        let sample_rate = self.input_sample_rate;
        let channels = self.input_channels;
        
        // Spawn writer thread
        std::thread::spawn(move || {
            let spec = hound::WavSpec {
                channels,
                sample_rate,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            
            match hound::WavWriter::create(&path, spec) {
                Ok(mut writer) => {
                    while let Ok(buffer) = rx.recv() {
                        if buffer.is_empty() { break; } // stop signal
                        for sample in buffer {
                            if let Err(e) = writer.write_sample(sample) {
                                eprintln!("Wav write error: {}", e);
                            }
                        }
                    }
                    let _ = writer.finalize();
                },
                Err(e) => eprintln!("Failed to create wav writer: {}", e)
            }
        });
        
        Ok(())
    }
    
    pub fn stop_recording(&mut self) -> Result<()> {
        let mut dest = self.recording_dest.lock().unwrap();
        if let Some(tx) = dest.take() {
            let _ = tx.send(Vec::new()); // Stop signal
        }
        Ok(())
    }
    


    pub fn send_command(&self, command: AudioCommand) -> Result<()> {
        self.command_sender.send(command).context("Failed to send audio command")
    }
}

// Internal Voice Stuctures
struct Voice {
    note: u8,
    frequency: f32,
    velocity: f32,
    start_time: f64,
    end_time: f64,
    release_end_time: f64,
}

struct TrackRuntime {
    next_note_idx: usize,
    active_voices: Vec<Voice>,
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    command_receiver: Receiver<AudioCommand>,
    meter_sender: Sender<MeterData>,
    playhead_sender: Sender<f64>,
    shared_state: SharedAudioState,
) -> Result<cpal::Stream>
where
    T: cpal::Sample + cpal::FromSample<f32> + SizedSample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Audio Thread Local State
    let mut current_pos_sec = 0.0;
    let mut playing = false;
    let mut track_runtimes: HashMap<usize, TrackRuntime> = HashMap::new();
    
    // Meter Timing & Accumulators
    let mut last_meter_time = Instant::now();
    let meter_interval = Duration::from_millis(constants::METER_UPDATE_INTERVAL_MS);
    let mut last_playhead_time = Instant::now();
    let playhead_interval = Duration::from_millis(constants::PLAYHEAD_UPDATE_INTERVAL_MS);
    let mut acc_master_peak = [0.0f32; 2];
    let mut acc_track_peaks: HashMap<usize, [f32; 2]> = HashMap::new();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // 1. Process Commands
             while let Ok(command) = command_receiver.try_recv() {
                match command {
                    AudioCommand::Play => playing = true,
                    AudioCommand::Pause => playing = false,
                    AudioCommand::Stop => {
                        playing = false;
                        current_pos_sec = 0.0;
                        // Reset all voices & meters
                        for rt in track_runtimes.values_mut() {
                            rt.next_note_idx = 0;
                            rt.active_voices.clear();
                        }
                        acc_master_peak = [0.0, 0.0];
                        acc_track_peaks.clear();
                    },
                    AudioCommand::SetVolume(_vol) => { }, 
                    AudioCommand::SetLooping(_enabled) => { },
                    AudioCommand::SetLoopRegion(_start, _end) => { },
                    AudioCommand::Seek(pos) => {
                        current_pos_sec = pos;
                        acc_master_peak = [0.0, 0.0];
                        acc_track_peaks.clear();
                        // Reset voices on seek
                        for rt in track_runtimes.values_mut() {
                            rt.next_note_idx = 0;
                            rt.active_voices.clear();
                        }
                    },
                    AudioCommand::LoadSequence(_) => { }
                }
            }

            // 2. Global State Snapshot
            let is_looping = shared_state.is_looping.load(Ordering::Relaxed);
            let loop_start = f64::from_bits(shared_state.loop_start.load(Ordering::Relaxed));
            let loop_end = f64::from_bits(shared_state.loop_end.load(Ordering::Relaxed));
            let synth_params = shared_state.synth.snapshot();

            // Calculate buffer end time
            let buffer_duration = data.len() as f64 / channels as f64 / sample_rate as f64;
            let buffer_end_pos = current_pos_sec + buffer_duration;

            // 3. Acquire Tracks Lock ONCE
            if let Ok(tracks_guard) = shared_state.tracks.try_read() {
                let any_solo = tracks_guard.iter().any(|t| t.soloed.load(Ordering::Relaxed));

                // 3a. Update Voices
                for track in tracks_guard.iter() {
                     let soloed = track.soloed.load(Ordering::Relaxed);
                     let muted = track.muted.load(Ordering::Relaxed);
                     
                     let runtime = track_runtimes.entry(track.id).or_insert(TrackRuntime {
                         next_note_idx: 0,
                         active_voices: Vec::with_capacity(constants::VOICE_INITIAL_CAPACITY),
                     });

                     if (any_solo && !soloed) || (!any_solo && muted) {
                         runtime.active_voices.clear();
                         continue;
                     }

                     if let Ok(content) = track.content.try_read() {
                         match &**content {
                             crate::state::TrackContent::Midi(seq) => {
                                 while runtime.next_note_idx < seq.notes.len() {
                                     let note = &seq.notes[runtime.next_note_idx];
                                     if note.start_time >= buffer_end_pos { break; }
                                     let release_end = note.start_time + note.duration + synth_params.release;
                                     if release_end < current_pos_sec {
                                         runtime.next_note_idx += 1;
                                         continue;
                                     }
                                     runtime.active_voices.push(Voice {
                                         note: note.note,
                                         frequency: constants::MIDI_A4_FREQUENCY * 2.0_f32.powf((note.note as f32 - constants::MIDI_A4_NOTE as f32) / 12.0),
                                         velocity: note.velocity as f32 / constants::MIDI_VELOCITY_MAX,
                                         start_time: note.start_time,
                                         end_time: note.start_time + note.duration,
                                         release_end_time: release_end,
                                     });
                                     runtime.next_note_idx += 1;
                                 }
                             },
                             _ => {} 
                         }
                     }
                     runtime.active_voices.retain(|v| v.release_end_time > current_pos_sec);
                }

                // 4. Render Setup
                struct ActiveRenderer<'a> {
                     id: usize,
                     content: std::sync::RwLockReadGuard<'a, std::sync::Arc<crate::state::TrackContent>>,
                     voices: &'a [Voice],
                     vol: f32,
                     pan_l: f32,
                     pan_r: f32,
                }
                 
                let mut active_renderers = Vec::with_capacity(tracks_guard.len());
                 
                for track in tracks_guard.iter() {
                     let soloed = track.soloed.load(Ordering::Relaxed);
                     let muted = track.muted.load(Ordering::Relaxed);
                     if (any_solo && !soloed) || (!any_solo && muted) { continue; }
                     
                     if let Some(runtime) = track_runtimes.get(&track.id) {
                         if let Ok(content) = track.content.try_read() {
                             let vol = f64::from_bits(track.volume.load(Ordering::Relaxed)) as f32;
                             let pan = f64::from_bits(track.pan.load(Ordering::Relaxed)) as f32;
                             let pan_l = (1.0 - pan).min(1.0).max(0.0);
                             let pan_r = (1.0 + pan).min(1.0).max(0.0);
                             
                             active_renderers.push(ActiveRenderer {
                                 id: track.id,
                                 content,
                                 voices: &runtime.active_voices,
                                 vol,
                                 pan_l,
                                 pan_r,
                             });
                         }
                     }
                }

                // 5. Render Loop (Per Sample)
                for frame in data.chunks_mut(channels) {
                     let mut mix_left = 0.0;
                     let mut mix_right = 0.0;
                     
                     if playing {
                         if is_looping && loop_end > loop_start {
                             if current_pos_sec >= loop_end {
                                 current_pos_sec = loop_start;
                             }
                         }

                         for renderer in &active_renderers {
                             let mut track_sample = 0.0;
                             
                             // Midi
                             if let crate::state::TrackContent::Midi(_) = &**renderer.content {
                                  for voice in renderer.voices {
                                      if current_pos_sec >= voice.start_time && current_pos_sec < voice.release_end_time {
                                          let t = current_pos_sec - voice.start_time;
                                          let envelope = if t < synth_params.attack {
                                              t / synth_params.attack
                                          } else if t < synth_params.attack + synth_params.decay {
                                              1.0 - (1.0 - synth_params.sustain) * ((t - synth_params.attack) / synth_params.decay)
                                          } else if t < voice.end_time - voice.start_time {
                                              synth_params.sustain
                                          } else {
                                              let release_t = current_pos_sec - voice.end_time;
                                              if release_t < synth_params.release {
                                                  synth_params.sustain * (1.0 - (release_t / synth_params.release))
                                              } else { 0.0 }
                                          };
                                          let phase = current_pos_sec as f32 * voice.frequency * 2.0 * std::f32::consts::PI;
                                          let osc_val = match synth_params.oscillator_type {
                                              crate::state::OscillatorType::Sine => phase.sin(),
                                              crate::state::OscillatorType::Square => if phase.sin() > 0.0 { 1.0 } else { -1.0 },
                                              crate::state::OscillatorType::Sawtooth => {
                                                  // Simple Sawtooth: 2 * (t * f - floor(t*f + 0.5))
                                                  // t * f is phase / 2PI.
                                                  let period_pos = (phase / (2.0 * std::f32::consts::PI)) % 1.0;
                                                  2.0 * (period_pos - 0.5)
                                              },
                                              crate::state::OscillatorType::Triangle => {
                                                  // Triangle from Sine or Saw
                                                  // 2/PI * asin(sin(phase)) is exact but slow.
                                                  // |Saw| is easier? 
                                                  // 4 * abs(period_pos - 0.5) - 1.0
                                                  let period_pos = (phase / (2.0 * std::f32::consts::PI)).fract(); // 0..1
                                                  if period_pos < 0.5 {
                                                      4.0 * period_pos - 1.0
                                                  } else {
                                                      3.0 - 4.0 * period_pos
                                                  }
                                              }
                                          };
                                          track_sample += osc_val * constants::SYNTH_OUTPUT_GAIN * envelope as f32 * voice.velocity;
                                      }
                                  }
                             } else if let crate::state::TrackContent::Audio(clip) = &**renderer.content {
                                  let sample_idx = (current_pos_sec * clip.sample_rate as f64).floor() as usize;
                                  if dim_check_ok(sample_idx, clip) {
                                      if clip.channels == 2 {
                                          let idx = sample_idx * 2;
                                          if idx + 1 < clip.samples.len() {
                                              track_sample += (clip.samples[idx] + clip.samples[idx+1]) * 0.5;
                                          }
                                      } else if sample_idx < clip.samples.len() {
                                          track_sample += clip.samples[sample_idx];
                                      }
                                  }
                             }
                             
                             let track_l = track_sample * renderer.vol * renderer.pan_l;
                             let track_r = track_sample * renderer.vol * renderer.pan_r;
                             
                             // Metering Track accumulation
                             let peaks = acc_track_peaks.entry(renderer.id).or_insert([0.0, 0.0]);
                             if track_l.abs() > peaks[0] { peaks[0] = track_l.abs(); }
                             if track_r.abs() > peaks[1] { peaks[1] = track_r.abs(); }

                             mix_left += track_l;
                             mix_right += track_r;
                         }
                         
                         current_pos_sec += 1.0 / sample_rate as f64;
                     } // end if playing
                     
                     // Apply Master Volume
                     let master_vol = f64::from_bits(shared_state.master_volume.load(Ordering::Relaxed)) as f32;
                     mix_left *= master_vol;
                     mix_right *= master_vol;
                     
                     // Metering Master accumulation
                     if mix_left.abs() > acc_master_peak[0] { acc_master_peak[0] = mix_left.abs(); }
                     if mix_right.abs() > acc_master_peak[1] { acc_master_peak[1] = mix_right.abs(); }

                     if channels >= 2 {
                         if let Some(s) = frame.get_mut(0) { *s = cpal::Sample::from_sample(mix_left); }
                         if let Some(s) = frame.get_mut(1) { *s = cpal::Sample::from_sample(mix_right); }
                     } else if let Some(s) = frame.get_mut(0) {
                         *s = cpal::Sample::from_sample((mix_left + mix_right) * 0.5);
                     }
                } // end frame loop
            
            } else {
                // FAILED TO ACQUIRE LOCK or EMPTY TRACKS
                // Output silence safely
                for frame in data.chunks_mut(channels) {
                    for sample in frame { *sample = cpal::Sample::from_sample(0.0); }
                }
            }

            // 6. Update Shared State
            if playing {
                shared_state.playhead_position.store(current_pos_sec.to_bits(), Ordering::Relaxed);
                shared_state.is_playing.store(true, Ordering::Relaxed);

                // --- SEND PLAYHEAD ---
                if last_playhead_time.elapsed() >= playhead_interval {
                    let _ = playhead_sender.try_send(current_pos_sec);
                    last_playhead_time = Instant::now();
                }

                // --- SEND METER DATA ---
                if last_meter_time.elapsed() >= meter_interval {
                    let _ = meter_sender.try_send(MeterData {
                        master_peak: acc_master_peak,
                        track_peaks: acc_track_peaks.clone()
                    });
                    
                    // Reset accumulators
                    acc_master_peak = [0.0, 0.0];
                    acc_track_peaks.clear();
                    last_meter_time = Instant::now();
                }
            } else {
                if last_meter_time.elapsed() >= meter_interval {
                     let _ = meter_sender.try_send(MeterData {
                        master_peak: [0.0, 0.0],
                        track_peaks: HashMap::new(),
                    });
                     // Reset accumulators
                    acc_master_peak = [0.0, 0.0];
                    acc_track_peaks.clear();
                    last_meter_time = Instant::now();
                }
            }
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn dim_check_ok(idx: usize, clip: &crate::state::AudioClip) -> bool {
    if clip.channels == 2 {
        idx * 2 + 1 < clip.samples.len()
    } else {
        idx < clip.samples.len()
    }
}

fn run_input<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    recording_dest: std::sync::Arc<std::sync::Mutex<Option<Sender<Vec<f32>>>>>,
) -> Result<cpal::Stream>
where
    T: cpal::Sample + SizedSample,
    f32: cpal::FromSample<T>, 
{
    // T must implement Sample. 
    // to_sample() comes from cpal::Sample trait.

    let err_fn = |err| eprintln!("an error occurred on input stream: {}", err);

    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| {
             // Check if recording
            if let Ok(dest_guard) = recording_dest.try_lock() {
                if let Some(sender) = &*dest_guard {
                    // Convert and send
                    let samples: Vec<f32> = data.iter()
                        .map(|s| s.to_sample::<f32>())
                        .collect();
                    let _ = sender.try_send(samples);
                }
            }
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}
