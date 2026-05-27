pub mod app;
pub mod engine;
pub mod midi;
pub mod state;
pub mod commands;

use std::sync::Arc;
use engine::EngineHandle;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Arc::new(EngineHandle::new());

    tauri::Builder::default()
        .manage(app::AppState {
            engine: Arc::clone(&engine),
        })
        .setup(|app| {
            app.handle().plugin(tauri_plugin_dialog::init())?;
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
                .invoke_handler(tauri::generate_handler![
            crate::commands::project::save_project,
            crate::commands::project::load_project,
            crate::commands::project::undo,
            crate::commands::project::redo,
            crate::commands::project::set_grid_settings,
            crate::commands::project::get_project_state,
            crate::commands::transport::play,
            crate::commands::transport::pause,
            crate::commands::transport::stop,
            crate::commands::transport::set_bpm,
            crate::commands::transport::set_master_volume,
            crate::commands::transport::get_midi_devices,
            crate::commands::track::add_track,
            crate::commands::track::remove_track,
            crate::commands::track::set_track_midi_routing,
            crate::commands::track::set_track_volume,
            crate::commands::track::set_track_pan,
            crate::commands::clip::add_audio_clip,
            crate::commands::clip::remove_audio_clip,
            crate::commands::clip::move_audio_clip,
            crate::commands::clip::add_midi_clip,
            crate::commands::clip::remove_midi_clip,
            crate::commands::clip::move_midi_clip,
            crate::commands::clip::update_midi_clip_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
