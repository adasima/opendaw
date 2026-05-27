use log::info;

use tauri::State;

use crate::app::AppState;

/// Undo (取り消し) を実行する
#[tauri::command]
pub fn undo(state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Undo");
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let mut history = state.engine.history.write().unwrap_or_else(|e| e.into_inner());

    if let Some(previous_state) = history.undo(&project_state) {
        *project_state = previous_state.clone();

        // エンジン側の値も更新する
        state.engine.set_bpm(project_state.bpm);
        state.engine.set_master_volume(project_state.master_volume);

        if project_state.is_playing {
            state.engine.play();
        } else {
            state.engine.pause();
        }
    }

    Ok(())
}

/// Redo (やり直し) を実行する
#[tauri::command]
pub fn redo(state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Redo");
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let mut history = state.engine.history.write().unwrap_or_else(|e| e.into_inner());

    if let Some(next_state) = history.redo(&project_state) {
        *project_state = next_state.clone();

        // エンジン側の値も更新する
        state.engine.set_bpm(project_state.bpm);
        state.engine.set_master_volume(project_state.master_volume);

        if project_state.is_playing {
            state.engine.play();
        } else {
            state.engine.pause();
        }
    }

    Ok(())
}

/// グリッド設定を更新する
#[tauri::command]
pub fn set_grid_settings(is_enabled: bool, resolution: u32, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Set Grid Settings: enabled={}, resolution={}", is_enabled, resolution);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    project_state.grid_settings.is_enabled = is_enabled;
    project_state.grid_settings.resolution = resolution;
    Ok(())
}

/// プロジェクトの現在の状態をJSONとして取得する
#[tauri::command]
pub fn get_project_state(state: State<'_, AppState>) -> String {
    let project_state_guard = state.engine.project_state.read().unwrap_or_else(|e| e.into_inner());
    let mut project_state = project_state_guard.clone();
    project_state.is_playing = state.engine.is_playing();
    project_state.bpm = state.engine.get_bpm();
    project_state.master_volume = state.engine.get_master_volume();

    serde_json::to_string(&project_state).unwrap_or_else(|_| "{}".to_string())
}

/// プロジェクトの状態をファイルに保存する
#[tauri::command]
pub fn save_project(path: String, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Save project to {}", path);
    let project_state_guard = state.engine.project_state.read().unwrap_or_else(|e| e.into_inner());

    let mut project_state = project_state_guard.clone();
    project_state.is_playing = state.engine.is_playing();
    project_state.bpm = state.engine.get_bpm();
    project_state.master_volume = state.engine.get_master_volume();

    let json = serde_json::to_string_pretty(&project_state)
        .map_err(|e| format!("Serialization error: {}", e))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("File write error: {}", e))?;

    Ok(())
}

/// ファイルからプロジェクトの状態を読み込む
#[tauri::command]
pub fn load_project(path: String, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Load project from {}", path);
    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("File read error: {}", e))?;

    let new_state: crate::state::ProjectState = serde_json::from_str(&json)
        .map_err(|e| format!("Deserialization error: {}", e))?;

    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    *project_state = new_state.clone();

    // エンジン側の値も更新する
    state.engine.set_bpm(project_state.bpm);
    state.engine.set_master_volume(project_state.master_volume);

    if project_state.is_playing {
        state.engine.play();
    } else {
        state.engine.pause();
    }

    Ok(())
}