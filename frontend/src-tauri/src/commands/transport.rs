use tauri::State;
use log::info;
use crate::app::AppState;

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
