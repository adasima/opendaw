pub mod app;
pub mod engine;
pub mod midi;
pub mod state;

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
            app::save_project,
            app::load_project,
            app::play,
            app::pause,
            app::stop,
            app::set_bpm,
            app::set_master_volume,
            app::get_midi_devices,
            app::set_track_midi_routing,
            app::get_project_state,
            app::add_track,
            app::remove_track,
            app::add_audio_clip,
            app::remove_audio_clip,
            app::move_audio_clip,
            app::add_midi_clip,
            app::remove_midi_clip,
            app::move_midi_clip,
            app::update_midi_clip_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
