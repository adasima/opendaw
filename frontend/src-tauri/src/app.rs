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
pub fn set_track_midi_routing(track_id: u32, device: String, channel: u8, _state: State<'_, AppState>) {
    info!("MIDI Route: Set track {} to device '{}' channel {}", track_id, device, channel);
    // 将来的にはここでエンジン等に設定を反映する
    // state.engine.set_track_midi_route(track_id, device, channel);
}
