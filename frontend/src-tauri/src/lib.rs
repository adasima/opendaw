pub mod app;
pub mod engine;

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
            app::play,
            app::pause,
            app::stop,
            app::set_bpm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
