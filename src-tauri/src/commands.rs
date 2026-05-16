use tauri::{State, AppHandle, Manager};
use crate::state::{SharedAudioState, AudioStateSnapshot};
use crate::audio_engine::{AudioCommand, AudioEngine};
use crossbeam_channel::Sender;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use crate::midi_parser::{ParsingOptions, MidiMetadata};
use std::process::Command;
use crate::error::AppError;

#[derive(serde::Deserialize)]
pub struct ImportOptions {
    bake_sustain: bool,
    import_tempo: bool,
    scale_to_bpm: Option<f64>,
}

#[tauri::command]
pub fn get_audio_state(state: State<SharedAudioState>) -> AudioStateSnapshot {
    state.get_snapshot()
}

#[tauri::command]
pub fn play(state: State<SharedAudioState>, sender: State<Sender<AudioCommand>>) -> Result<(), AppError> {
    state.is_playing.store(true, Ordering::Relaxed);
    sender.send(AudioCommand::Play).map_err(|e| AppError::internal(e.to_string()))
}

#[tauri::command]
pub fn pause(state: State<SharedAudioState>, sender: State<Sender<AudioCommand>>) -> Result<(), AppError> {
    state.is_playing.store(false, Ordering::Relaxed);
    sender.send(AudioCommand::Pause).map_err(|e| AppError::internal(e.to_string()))
}

#[tauri::command]
pub fn stop(state: State<SharedAudioState>, sender: State<Sender<AudioCommand>>) -> Result<(), AppError> {
    state.is_playing.store(false, Ordering::Relaxed);
    state.playhead_position.store(0f64.to_bits(), Ordering::Relaxed); 
    sender.send(AudioCommand::Stop).map_err(|e| AppError::internal(e.to_string()))
}

#[tauri::command]
pub fn seek(state: State<SharedAudioState>, sender: State<Sender<AudioCommand>>, position: f64) -> Result<(), AppError> {
    state.playhead_position.store(position.to_bits(), Ordering::Relaxed);
    sender.send(AudioCommand::Seek(position)).map_err(|e| AppError::internal(e.to_string()))
}

#[tauri::command]
pub fn set_looping(state: State<SharedAudioState>, sender: State<Sender<AudioCommand>>, enabled: bool) -> Result<(), AppError> {
    state.is_looping.store(enabled, Ordering::Relaxed);
    sender.send(AudioCommand::SetLooping(enabled)).map_err(|e| AppError::internal(e.to_string()))
}

#[tauri::command]
pub fn set_loop_region(state: State<SharedAudioState>, sender: State<Sender<AudioCommand>>, start: f64, end: f64) -> Result<(), AppError> {
    state.loop_start.store(start.to_bits(), Ordering::Relaxed);
    state.loop_end.store(end.to_bits(), Ordering::Relaxed);
    sender.send(AudioCommand::SetLoopRegion(start, end)).map_err(|e| AppError::internal(e.to_string()))
}

#[tauri::command]
pub fn get_sequence(state: State<SharedAudioState>) -> crate::sequence::Sequence {
    // Legacy support: Return content of Track 0
    let tracks_guard = state.tracks.read().unwrap();
    if let Some(track) = tracks_guard.first() {
        if let Ok(content) = track.content.read() {
            if let crate::state::TrackContent::Midi(seq) = &**content {
                return seq.clone();
            }
        }
    }
    // Fallback if no tracks or not MIDI
    crate::sequence::Sequence::new(120.0)
}

#[tauri::command]
pub fn update_sequence(state: State<SharedAudioState>, _sender: State<Sender<AudioCommand>>, track_id: usize, notes: Vec<crate::sequence::NoteEvent>) -> Result<(), AppError> {
    let tracks_guard = state.tracks.read().map_err(|e| AppError::internal(e.to_string()))?;
    
    if let Some(track) = tracks_guard.get(track_id) {
        let mut content_guard = track.content.write().map_err(|e| AppError::internal(e.to_string()))?;
        
        let new_content = match &**content_guard {
            crate::state::TrackContent::Midi(seq) => {
                let mut new_seq = seq.clone();
                new_seq.notes = notes;
                crate::state::TrackContent::Midi(new_seq)
            },
            crate::state::TrackContent::Audio(_) => return Err(AppError::invalid_operation(format!("Track {} is an Audio track, cannot update sequence", track_id)))
        };
        
        *content_guard = Arc::new(new_content);
    } else {
        return Err(AppError::invalid_operation(format!("Track {} not found", track_id)));
    }
    Ok(())
}

#[tauri::command]
pub fn set_oscillator_type(state: State<SharedAudioState>, oscillator_type: crate::state::OscillatorType) -> Result<(), AppError> {
    state.synth.oscillator_type.store(oscillator_type as u8, Ordering::Relaxed);
    Ok(())
}



#[tauri::command]
pub fn set_adsr(state: State<SharedAudioState>, attack: f64, decay: f64, sustain: f64, release: f64) -> Result<(), AppError> {
    state.synth.attack.store(attack.to_bits(), Ordering::Relaxed);
    state.synth.decay.store(decay.to_bits(), Ordering::Relaxed);
    state.synth.sustain.store(sustain.to_bits(), Ordering::Relaxed);
    state.synth.release.store(release.to_bits(), Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub async fn get_midi_info(path: String) -> Result<MidiMetadata, AppError> {
    let path_clone = path.clone();
    let (metadata, _) = tokio::task::spawn_blocking(move || {
        crate::midi_parser::load_midi_file(&path_clone, None)
    }).await
        .map_err(|e| AppError::internal(e.to_string()))? // JoinError
        .map_err(|e| AppError::midi_parse(e.to_string()))?; // Parser error
    Ok(metadata)
}

#[tauri::command]
pub async fn import_midi(state: State<'_, SharedAudioState>, path: String, options: ImportOptions) -> Result<Vec<usize>, AppError> {
    let path_clone = path.clone();
    let parse_opts = ParsingOptions {
        bake_sustain: options.bake_sustain,
    };

    let (metadata, parsed_tracks) = tokio::task::spawn_blocking(move || {
        crate::midi_parser::load_midi_file(&path_clone, Some(parse_opts))
    }).await
        .map_err(|e| AppError::internal(e.to_string()))?
        .map_err(|e| AppError::midi_parse(e.to_string()))?;

    // BPM Handling
    if options.import_tempo {
        state.tempo.store(metadata.initial_bpm.to_bits(), Ordering::Relaxed);
    }

    let scale_factor = if let Some(target_bpm) = options.scale_to_bpm {
         metadata.initial_bpm / target_bpm
    } else {
        1.0
    };

    let mut tracks = state.tracks.write().map_err(|e| AppError::internal(e.to_string()))?;
    let start_id = tracks.len();
    let mut new_ids = Vec::new();

    use std::sync::RwLock;
    use crate::state::{AtomicTrack, TrackContent};

    for (i, p_track) in parsed_tracks.into_iter().enumerate() {
        let new_id = start_id + i;
        let mut notes = p_track.notes;
        
        // Apply scaling
        if (scale_factor - 1.0).abs() > 0.0001 {
            for note in &mut notes {
                note.start_time *= scale_factor;
                note.duration *= scale_factor;
            }
        }
        
        let sequence = crate::sequence::Sequence {
            notes,
            tempo: metadata.initial_bpm, 
        };

        let track = Arc::new(AtomicTrack {
            id: new_id,
            name: RwLock::new(p_track.name),
            volume: std::sync::atomic::AtomicU64::new(1.0f64.to_bits()),
            pan: std::sync::atomic::AtomicU64::new(0.0f64.to_bits()),
            muted: std::sync::atomic::AtomicBool::new(false),
            soloed: std::sync::atomic::AtomicBool::new(false),
            content: RwLock::new(Arc::new(TrackContent::Midi(sequence))),
        });

        tracks.push(track);
        new_ids.push(new_id);
    }

    Ok(new_ids)
}

#[tauri::command]
pub async fn add_audio_track(state: State<'_, SharedAudioState>, path: String) -> Result<(), AppError> {
    // Run blocking import in a separate thread (async command)
    let path_clone = path.clone();
    let loaded = tokio::task::spawn_blocking(move || {
        crate::importer::import_audio_file(&path_clone)
    }).await
        .map_err(|e| AppError::internal(e.to_string()))?
        .map_err(|e| AppError::file_io(e.to_string(), Some(path.clone())))?;

    let mut tracks = state.tracks.write().map_err(|e| AppError::internal(e.to_string()))?;
    let new_id = tracks.len(); 
    
    // Create Audio Track
    use std::sync::RwLock;
    use crate::state::{AtomicTrack, TrackContent, AudioClip};
    
    let clip = AudioClip {
        path: path.clone(),
        samples: Arc::new(loaded.samples),
        sample_rate: loaded.sample_rate,
        channels: loaded.channels,
        duration_seconds: loaded.duration_seconds,
    };
    
    let track = Arc::new(AtomicTrack {
        id: new_id,
        name: RwLock::new(format!("Audio {}", new_id)), // Default name
        volume: std::sync::atomic::AtomicU64::new(1.0f64.to_bits()),
        pan: std::sync::atomic::AtomicU64::new(0.0f64.to_bits()),
        muted: std::sync::atomic::AtomicBool::new(false),
        soloed: std::sync::atomic::AtomicBool::new(false),
        content: RwLock::new(Arc::new(TrackContent::Audio(clip))),
    });
    
    tracks.push(track);
    Ok(())
}

#[tauri::command]
pub async fn export_project(state: State<'_, SharedAudioState>, path: String) -> Result<(), AppError> {
    // Run export in blocking thread
    let state_clone = state.inner().clone();
    // Assuming exporter returns Result<(), anyhow::Error> or similar
    let path_clone = path.clone();
    tokio::task::spawn_blocking(move || {
        crate::exporter::export_mixdown(state_clone, path_clone)
    }).await
        .map_err(|e| AppError::internal(e.to_string()))?
        .map_err(|e| AppError::file_io(e.to_string(), Some(path)))?;
    Ok(())
}

#[tauri::command]
pub fn set_track_volume(state: State<SharedAudioState>, track_id: usize, volume: f64) -> Result<(), AppError> {
    let tracks = state.tracks.read().map_err(|e| AppError::internal(e.to_string()))?;
    if let Some(track) = tracks.get(track_id) {
        track.volume.store(volume.to_bits(), Ordering::Relaxed);
        Ok(())
    } else {
        Err(AppError::invalid_operation("Track not found"))
    }
}

#[tauri::command]
pub fn set_master_volume(state: State<SharedAudioState>, volume: f64) -> Result<(), AppError> {
    state.master_volume.store(volume.to_bits(), Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_track_pan(state: State<SharedAudioState>, track_id: usize, pan: f64) -> Result<(), AppError> {
    let tracks = state.tracks.read().map_err(|e| AppError::internal(e.to_string()))?;
    if let Some(track) = tracks.get(track_id) {
        track.pan.store(pan.to_bits(), Ordering::Relaxed);
        Ok(())
    } else {
        Err(AppError::invalid_operation("Track not found"))
    }
}

#[tauri::command]
pub fn set_track_mute(state: State<SharedAudioState>, track_id: usize, muted: bool) -> Result<(), AppError> {
    let tracks = state.tracks.read().map_err(|e| AppError::internal(e.to_string()))?;
    if let Some(track) = tracks.get(track_id) {
        track.muted.store(muted, Ordering::Relaxed);
        Ok(())
    } else {
        Err(AppError::invalid_operation("Track not found"))
    }
}

#[tauri::command]
pub fn set_track_solo(state: State<SharedAudioState>, track_id: usize, soloed: bool) -> Result<(), AppError> {
    let tracks = state.tracks.read().map_err(|e| AppError::internal(e.to_string()))?;
    if let Some(track) = tracks.get(track_id) {
        track.soloed.store(soloed, Ordering::Relaxed);
        Ok(())
    } else {
        Err(AppError::invalid_operation("Track not found"))
    }
}

#[tauri::command]
pub fn export_midi(state: State<SharedAudioState>, track_id: usize, path: String) -> Result<(), AppError> {
    let tracks_guard = state.tracks.read().map_err(|e| AppError::internal(e.to_string()))?;
    
    if let Some(track) = tracks_guard.get(track_id) {
        let content_guard = track.content.read().map_err(|e| AppError::internal(e.to_string()))?;
        
        match &**content_guard {
            crate::state::TrackContent::Midi(seq) => {
                let tempo = f64::from_bits(state.tempo.load(Ordering::Relaxed));
                crate::midi_exporter::export_midi_file(seq, &path, tempo)
                    .map_err(|e| AppError::file_io(e.to_string(), Some(path)))?;
                Ok(())
            }
            crate::state::TrackContent::Audio(_) => {
                Err(AppError::invalid_operation("Cannot export Audio track as MIDI"))
            }
        }
    } else {
        Err(AppError::invalid_operation(format!("Track {} not found", track_id)))
    }
}

#[tauri::command]
pub fn new_project(state: State<SharedAudioState>, sender: State<Sender<AudioCommand>>) -> Result<(), AppError> {
    // 1. Stop playback
    state.is_playing.store(false, Ordering::Relaxed);
    state.playhead_position.store(0f64.to_bits(), Ordering::Relaxed);
    let _ = sender.send(AudioCommand::Stop); 

    // 2. Clear tracks
    let mut tracks = state.tracks.write().map_err(|e| AppError::internal(e.to_string()))?;
    tracks.clear();

    // 3. Reset loop & general state
    state.is_looping.store(false, Ordering::Relaxed);
    state.loop_start.store(0f64.to_bits(), Ordering::Relaxed);
    state.loop_end.store(4f64.to_bits(), Ordering::Relaxed); 
    state.tempo.store(120.0f64.to_bits(), Ordering::Relaxed); // Reset tempo

    Ok(())
}


#[tauri::command]
pub fn open_locales_folder(app: AppHandle) -> Result<(), AppError> {
    let path = app.path().app_local_data_dir().map_err(|e| AppError::file_io(e.to_string(), None))?;
    let locales_path = path.join("locales");
    if !locales_path.exists() {
        std::fs::create_dir_all(&locales_path).map_err(|e| AppError::file_io(e.to_string(), Some(locales_path.to_string_lossy().into())))?;
    }
    // Create custom.json template if not exists
    let custom_path = locales_path.join("custom.json");
    if !custom_path.exists() {
         let _ = std::fs::write(&custom_path, "{}");
    }
    
    #[cfg(target_os = "windows")]
    Command::new("explorer").arg(&locales_path).spawn().map_err(|e| AppError::file_io(e.to_string(), Some(locales_path.to_string_lossy().into())))?;

    #[cfg(target_os = "macos")]
    Command::new("open").arg(&locales_path).spawn().map_err(|e| AppError::file_io(e.to_string(), Some(locales_path.to_string_lossy().into())))?;

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(&locales_path).spawn().map_err(|e| AppError::file_io(e.to_string(), Some(locales_path.to_string_lossy().into())))?;

    Ok(())
}

#[tauri::command]
pub fn load_custom_locale(app: AppHandle) -> Result<serde_json::Value, AppError> {
     let path = app.path().app_local_data_dir().map_err(|e| AppError::file_io(e.to_string(), None))?;
     let custom_path = path.join("locales").join("custom.json");
     if custom_path.exists() {
         let content = std::fs::read_to_string(&custom_path).map_err(|e| AppError::file_io(e.to_string(), Some(custom_path.to_string_lossy().into())))?;
         let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| AppError::internal(e.to_string()))?; // JSON error
         Ok(json)
     } else {
         Ok(serde_json::json!({}))
     }
}

#[derive(serde::Serialize)]
pub struct AudioDeviceInfo {
    pub name: String,
    pub is_default: bool,
}

#[tauri::command]
pub fn get_output_devices() -> Result<Vec<AudioDeviceInfo>, AppError> {
    use cpal::traits::{HostTrait, DeviceTrait};
    
    let host = cpal::default_host();
    let default_device = host.default_output_device();
    let default_name = default_device.as_ref().and_then(|d| d.name().ok());
    
    let devices: Vec<AudioDeviceInfo> = host.output_devices()
        .map_err(|e| AppError::audio_device(e.to_string()))?
        .filter_map(|device| {
            device.name().ok().map(|name| {
                let is_default = default_name.as_ref().map(|dn| dn == &name).unwrap_or(false);
                AudioDeviceInfo { name, is_default }
            })
        })
        .collect();
    
    Ok(devices)
}

#[tauri::command]
pub async fn set_output_device(
    engine_state: State<'_, Arc<Mutex<AudioEngine>>>,
    name: String
) -> Result<(), AppError> {
    let engine_state = engine_state.inner().clone();
    
    // Run stream restart in blocking thread as it involves cpal I/O
    tokio::task::spawn_blocking(move || {
        let mut engine = engine_state.lock().map_err(|e| AppError::internal(e.to_string()))?;
        engine.start(Some(name)).map_err(|e| AppError::audio_device(e.to_string()))
    }).await
    .map_err(|e| AppError::internal(e.to_string()))??;
    
    Ok(())
}

#[tauri::command]
pub async fn start_recording(
    engine_state: State<'_, Arc<Mutex<AudioEngine>>>,
    path: String
) -> Result<(), AppError> {
    let engine_state = engine_state.inner().clone();
    tokio::task::spawn_blocking(move || {
        let mut engine = engine_state.lock().map_err(|e| AppError::internal(e.to_string()))?;
        engine.start_recording(path).map_err(|e| AppError::file_io(e.to_string(), None))
    }).await
    .map_err(|e| AppError::internal(e.to_string()))??;
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    engine_state: State<'_, Arc<Mutex<AudioEngine>>>
) -> Result<(), AppError> {
    let engine_state = engine_state.inner().clone();
    tokio::task::spawn_blocking(move || {
        let mut engine = engine_state.lock().map_err(|e| AppError::internal(e.to_string()))?;
        engine.stop_recording().map_err(|e| AppError::internal(e.to_string()))
    }).await
    .map_err(|e| AppError::internal(e.to_string()))??;
    Ok(())
}
