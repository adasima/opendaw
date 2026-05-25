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
    state.engine.play();
}

/// 再生を一時停止する
#[tauri::command]
pub fn pause(state: State<'_, AppState>) {
    info!("Transport: Pause");
    state.engine.pause();
}

/// 再生を停止する
#[tauri::command]
pub fn stop(state: State<'_, AppState>) {
    info!("Transport: Stop");
    state.engine.stop();
}

/// BPMを設定する
#[tauri::command]
pub fn set_bpm(bpm: f64, state: State<'_, AppState>) {
    info!("Transport: Set BPM to {}", bpm);
    state.engine.set_bpm(bpm);
}

/// マスターボリュームを設定する
#[tauri::command]
pub fn set_master_volume(volume: f64, state: State<'_, AppState>) {
    info!("Mixer: Set Master Volume to {}", volume);
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
    // state.engine.set_track_volume(track_id, volume);
}

/// トラックのパンを設定する
#[tauri::command]
pub fn set_track_pan(track_id: u32, pan: f64, _state: State<'_, AppState>) {
    info!("Mixer: Set track {} pan to {}", track_id, pan);
    // state.engine.set_track_pan(track_id, pan);
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
