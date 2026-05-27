use log::info;

use tauri::State;

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