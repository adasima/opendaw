use crate::app::AppState;
use tauri::State;

/// プラグインをトラックにロードするコマンド
#[tauri::command]
pub fn load_plugin_to_track(
    track_id: usize,
    plugin_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut project_state = state
        .engine
        .project_state
        .write()
        .map_err(|_| "Failed to lock project state")?;

    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        track.plugins.push(plugin_id);
    }

    Ok(())
}
