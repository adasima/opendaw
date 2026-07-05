pub mod importer;
pub mod exporter;
pub mod midi_exporter;
pub mod sequence;
pub mod midi_parser;
pub mod audio_engine;
pub mod commands;
pub mod state;
pub mod error;
pub mod constants;

use tauri::Manager;
use tauri::Emitter;
use std::sync::{Arc, Mutex};
use crate::state::AtomicAudioState;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize Shared State
    // Initialize Audio State (Lock-Free)
    // Create empty sequence for initial state
    let empty_sequence = sequence::Sequence::new(120.0);
    let audio_state = Arc::new(AtomicAudioState::new(empty_sequence));

    // Initialize Audio Engine
    let audio_engine = audio_engine::AudioEngine::new(audio_state.clone())
        .expect("Failed to initialize audio engine");

    let meter_receiver = audio_engine.meter_receiver.clone();
    let playhead_receiver = audio_engine.playhead_receiver.clone();
    let command_sender = audio_engine.command_sender.clone();
    
    let audio_engine_state = Arc::new(Mutex::new(audio_engine));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let app_handle_ph = app.handle().clone();
            
            std::thread::spawn(move || {
                while let Ok(meter_data) = meter_receiver.recv() {
                    let _ = app_handle.emit("meter-update", meter_data);
                }
            });

            std::thread::spawn(move || {
                while let Ok(pos) = playhead_receiver.recv() {
                    let _ = app_handle_ph.emit("playhead-update", pos);
                }
            });
            Ok(())
        })
        .manage(audio_state) // Manage SharedAudioState
        .manage(command_sender) // Manage Sender<AudioCommand>
        .manage(audio_engine_state) // Manage Arc<Mutex<AudioEngine>>
        .invoke_handler(tauri::generate_handler![
            commands::get_audio_state,
            commands::play,
            commands::pause,
            commands::stop,
            commands::seek,
            commands::set_looping,
            commands::set_loop_region,
            commands::get_sequence,
            commands::update_sequence,
            commands::set_adsr,
            commands::get_midi_info,
            commands::import_midi,
            commands::add_audio_track,
            commands::export_project,
            commands::set_track_volume,
            commands::set_track_pan,
            commands::set_track_mute,
            commands::set_track_solo,
            commands::export_midi,
            commands::new_project,
            commands::open_locales_folder,
            commands::load_custom_locale,
            commands::get_output_devices,
            commands::set_output_device,
            commands::start_recording,
            commands::stop_recording,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
