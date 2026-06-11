pub mod app;
pub mod commands;
pub mod engine;
pub mod midi;
pub mod state;

use engine::EngineHandle;
use std::sync::Arc;

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
            commands::project::save_project,
            commands::project::load_project,
            commands::transport::play,
            commands::transport::pause,
            commands::transport::stop,
            commands::transport::set_bpm,
            commands::track::set_master_volume,
            commands::project::set_grid_settings,
            commands::track::get_midi_devices,
            commands::track::set_track_midi_routing,
            commands::project::get_project_state,
            commands::track::add_track,
            commands::track::remove_track,
            commands::clip::add_audio_clip,
            commands::clip::remove_audio_clip,
            commands::clip::move_audio_clip,
            commands::clip::add_midi_clip,
            commands::clip::remove_midi_clip,
            commands::clip::move_midi_clip,
            commands::clip::update_midi_clip_notes,
            commands::project::undo,
            commands::project::redo,
            commands::plugin::load_plugin_to_track,
            commands::track::set_track_output_routing,
            commands::track::add_track_send,
            commands::track::set_track_send_amount,
            commands::track::update_automation_point,
            commands::track::remove_automation_point,
            commands::track::set_automation_visibility
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
