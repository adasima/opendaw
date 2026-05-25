use log::info;
use tauri::State;
use std::sync::Arc;
use crate::engine::EngineHandle;

pub struct AppState {
    pub engine: Arc<EngineHandle>,
}

/// 再生を開始する
#[tauri::command]
pub fn play(state: State<'_, AppState>) {
    info!("Transport: Play");
    let mut proj = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    proj.is_playing = true;
    state.engine.play();
}

/// 再生を一時停止する
#[tauri::command]
pub fn pause(state: State<'_, AppState>) {
    info!("Transport: Pause");
    let mut proj = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    proj.is_playing = false;
    state.engine.pause();
}

/// 再生を停止する
#[tauri::command]
pub fn stop(state: State<'_, AppState>) {
    info!("Transport: Stop");
    let mut proj = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    proj.is_playing = false;
    state.engine.stop();
}

/// BPMを設定する
#[tauri::command]
pub fn set_bpm(bpm: f64, state: State<'_, AppState>) {
    info!("Transport: Set BPM to {}", bpm);
    let mut proj = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    proj.bpm = bpm;
    state.engine.set_bpm(bpm);
}

/// マスターボリュームを設定する
#[tauri::command]
pub fn set_master_volume(volume: f64, state: State<'_, AppState>) {
    info!("Mixer: Set Master Volume to {}", volume);
    let mut proj = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    proj.master_volume = volume;
    state.engine.set_master_volume(volume);
}

/// MIDIデバイスのリストを取得する
#[tauri::command]
pub fn get_midi_devices() -> Vec<String> {
    info!("MIDI: Get MIDI devices");
    // ダミーデータを返す（将来的にはmidir等を使用して実際のデバイスリストを取得する）
    vec!["Launchkey Mini".to_string(), "Scarlett 2i2 USB".to_string(), "Virtual MIDI Bus".to_string()]
}

/// トラックに対するMIDIデバイスとチャンネルのルーティングを設定する
#[tauri::command]
pub fn set_track_midi_routing(track_id: u32, device: String, channel: u8, state: State<'_, AppState>) {
    info!("MIDI Route: Set track {} to device '{}' channel {}", track_id, device, channel);
    state.engine.set_track_midi_route(track_id, device, channel);
}

/// トラックのボリュームを設定する
#[tauri::command]
pub fn set_track_volume(track_id: u32, volume: f64, _state: State<'_, AppState>) {
    info!("Mixer: Set track {} volume to {}", track_id, volume);
    let mut proj = _state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = proj.tracks.iter_mut().find(|t| t.id == track_id as usize) {
        track.volume = volume as f32;
    }
    // _state.engine.set_track_volume(track_id, volume);
}

/// トラックのパンを設定する
#[tauri::command]
pub fn set_track_pan(track_id: u32, pan: f64, _state: State<'_, AppState>) {
    info!("Mixer: Set track {} pan to {}", track_id, pan);
    let mut proj = _state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = proj.tracks.iter_mut().find(|t| t.id == track_id as usize) {
        track.pan = pan as f32;
    }
    // _state.engine.set_track_pan(track_id, pan);
}

/// プロジェクトの現在の状態をJSONとして取得する
#[tauri::command]
pub fn get_project_state(state: State<'_, AppState>) -> String {
    let project_state_guard = match state.engine.project_state.read() {
        Ok(guard) => guard,
        Err(_) => return "{}".to_string(), // Lock was poisoned, return empty state
    };
    let mut project_state = project_state_guard.clone();
    project_state.is_playing = state.engine.is_playing();
    project_state.bpm = state.engine.get_bpm();
    project_state.master_volume = state.engine.get_master_volume();

    serde_json::to_string(&project_state).unwrap_or_else(|_| "{}".to_string())
}

/// トラックを追加する
#[tauri::command]
pub fn add_track(name: String, state: State<'_, AppState>) -> Result<u32, String> {
    info!("Project: Add track '{}'", name);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let new_id = project_state.tracks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let track = crate::state::Track::new(new_id, name);
    project_state.tracks.push(track);
    Ok(new_id as u32)
}

/// トラックを削除する
#[tauri::command]
pub fn remove_track(track_id: usize, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Remove track {}", track_id);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    project_state.tracks.retain(|t| t.id != track_id);
    Ok(())
}


/// オーディオクリップを追加する
#[tauri::command]
pub fn add_audio_clip(track_id: usize, name: String, start_pos: f32, length: f32, state: State<'_, AppState>) -> Result<usize, String> {
    info!("Project: Add audio clip '{}' to track {}", name, track_id);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let new_id = track.clips.iter().map(|c| c.id).max().unwrap_or(0) + 1;
        let clip = crate::state::clip::AudioClip {
            id: new_id,
            name,
            start_pos,
            length,
            waveform_summary: Vec::new(),
        };
        track.clips.push(clip);
        Ok(new_id)
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// オーディオクリップを削除する
#[tauri::command]
pub fn remove_audio_clip(track_id: usize, clip_id: usize, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Remove audio clip {} from track {}", clip_id, track_id);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        track.clips.retain(|c| c.id != clip_id);
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// オーディオクリップを移動する
#[tauri::command]
pub fn move_audio_clip(track_id: usize, clip_id: usize, new_start_pos: f32, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Move audio clip {} in track {} to {}", clip_id, track_id, new_start_pos);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        if let Some(clip) = track.clips.iter_mut().find(|c| c.id == clip_id) {
            clip.start_pos = new_start_pos;
            Ok(())
        } else {
            Err(format!("Clip {} not found in track {}", clip_id, track_id))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップを追加する
#[tauri::command]
pub fn add_midi_clip(track_id: usize, name: String, start_beat: f64, length_beats: f64, state: State<'_, AppState>) -> Result<usize, String> {
    info!("Project: Add midi clip '{}' to track {}", name, track_id);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let new_id = track.midi_clips.iter().map(|c| c.id).max().unwrap_or(0) + 1;
        let clip = crate::state::clip::MidiClip {
            id: new_id,
            name,
            start_beat,
            length_beats,
            sequence: crate::midi::sequence::Sequence::new(),
        };
        track.midi_clips.push(clip);
        Ok(new_id)
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップを削除する
#[tauri::command]
pub fn remove_midi_clip(track_id: usize, clip_id: usize, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Remove midi clip {} from track {}", clip_id, track_id);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        track.midi_clips.retain(|c| c.id != clip_id);
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップを移動する
#[tauri::command]
pub fn move_midi_clip(track_id: usize, clip_id: usize, new_start_beat: f64, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Move midi clip {} in track {} to {}", clip_id, track_id, new_start_beat);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        if let Some(clip) = track.midi_clips.iter_mut().find(|c| c.id == clip_id) {
            clip.start_beat = new_start_beat;
            Ok(())
        } else {
            Err(format!("Clip {} not found in track {}", clip_id, track_id))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップのノート情報を更新する
#[tauri::command]
pub fn update_midi_clip_notes(track_id: usize, clip_id: usize, notes: Vec<crate::midi::sequence::NoteEvent>, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Update midi clip notes for clip {} in track {}", clip_id, track_id);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        if let Some(clip) = track.midi_clips.iter_mut().find(|c| c.id == clip_id) {
            clip.sequence.notes = notes;

            // Sequence の next_note_id は private フィールドなので直接変更できない。
            // しかし Sequence 構造体に `clear()` して `add_note_event()` で入れ直すか、
            // もしくは `notes` フィールドは pub なので、Sequence自体を新しいものに置き換える。

            // 新しい Sequence を作り、notes を設定し、
            // 次のIDが適切になるようにするため、add_note_eventを使う
            let mut new_seq = crate::midi::sequence::Sequence::new();
            for note in clip.sequence.notes.clone() {
                new_seq.add_note_event(note);
            }
            clip.sequence = new_seq;

            Ok(())
        } else {
            Err(format!("Clip {} not found in track {}", clip_id, track_id))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}
