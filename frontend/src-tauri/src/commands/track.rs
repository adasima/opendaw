use log::info;

use tauri::State;

use crate::app::AppState;

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

/// トラックに対するMIDIデバイスとチャンネルのルーティングを設定する
#[tauri::command]
pub fn set_track_midi_routing(track_id: u32, device: String, channel: u8, state: State<'_, AppState>) {
    info!("MIDI Route: Set track {} to device '{}' channel {}", track_id, device, channel);
    state.engine.set_track_midi_route(track_id, device, channel);
}

/// トラックのボリュームを設定する
#[tauri::command]
pub fn set_track_volume(track_id: u32, volume: f64, _state: State<'_, AppState>) {
    info!("Mixer: Set track {} volume to {}", track_id, volume);
    let mut proj = _state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = proj.tracks.iter_mut().find(|t| t.id == track_id as usize) {
        track.volume = volume as f32;
    }
    // _state.engine.set_track_volume(track_id, volume);
}

/// トラックのパンを設定する
#[tauri::command]
pub fn set_track_pan(track_id: u32, pan: f64, _state: State<'_, AppState>) {
    info!("Mixer: Set track {} pan to {}", track_id, pan);
    let mut proj = _state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    if let Some(track) = proj.tracks.iter_mut().find(|t| t.id == track_id as usize) {
        track.pan = pan as f32;
    }
    // _state.engine.set_track_pan(track_id, pan);
}

/// トラックを追加する
#[tauri::command]
pub fn add_track(name: String, state: State<'_, AppState>) -> Result<u32, String> {
    info!("Project: Add track '{}'", name);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    let new_id = project_state.tracks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let track = crate::state::Track::new(new_id, name);
    project_state.tracks.push(track);
    state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
        Ok(new_id as u32)
}

/// トラックを削除する
#[tauri::command]
pub fn remove_track(track_id: usize, state: State<'_, AppState>) -> Result<(), String> {
    info!("Project: Remove track {}", track_id);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    project_state.tracks.retain(|t| t.id != track_id);
    state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
        Ok(())
}

/// トラックの出力ルーティングを設定する
#[tauri::command]
pub fn set_track_output_routing(track_id: usize, target: Option<usize>, state: State<'_, AppState>) -> Result<(), String> {
    info!("Routing: Set track {} output to {:?}", track_id, target);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        track.output_routing = target;
        state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// オートメーションポイントを追加・更新する
#[tauri::command]
pub fn update_automation_point(track_id: usize, param_name: String, time: f64, value: f32, state: State<'_, AppState>) -> Result<(), String> {
    info!("Automation: Update point on track {} ({}) at {} = {}", track_id, param_name, time, value);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();

    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let auto_track = match track.automations.iter_mut().find(|a| a.parameter_name == param_name) {
            Some(a) => a,
            None => {
                track.automations.push(crate::state::AutomationTrack {
                    parameter_name: param_name.clone(),
                    points: Vec::new(),
                });
                track.automations.last_mut().unwrap()
            }
        };

        // Update if existing at approximately same time, else insert
        if let Some(existing) = auto_track.points.iter_mut().find(|p| (p.time - time).abs() < 0.001) {
            existing.value = value;
        } else {
            let new_id = auto_track.points.iter().map(|p| p.id).max().unwrap_or(0) + 1;
            auto_track.points.push(crate::state::AutomationPoint {
                id: new_id,
                time,
                value,
            });
            auto_track.points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        }

        state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// オートメーションポイントを削除する
#[tauri::command]
pub fn remove_automation_point(track_id: usize, param_name: String, point_id: usize, state: State<'_, AppState>) -> Result<(), String> {
    info!("Automation: Remove point {} on track {} ({})", point_id, track_id, param_name);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();

    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        if let Some(auto_track) = track.automations.iter_mut().find(|a| a.parameter_name == param_name) {
            auto_track.points.retain(|p| p.id != point_id);
            state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
            Ok(())
        } else {
            Err(format!("Automation track {} not found", param_name))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// オートメーションの表示状態を設定する
#[tauri::command]
pub fn set_automation_visibility(track_id: usize, visible: bool, selected_param: Option<String>, state: State<'_, AppState>) -> Result<(), String> {
    info!("Automation: Set visibility on track {} to {} ({:?})", track_id, visible, selected_param);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();

    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        track.automation_visible = visible;
        track.selected_automation = selected_param;
        state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// トラックにセンドルーティングを追加する
#[tauri::command]
pub fn add_track_send(track_id: usize, target_track_id: usize, amount: f32, state: State<'_, AppState>) -> Result<(), String> {
    info!("Routing: Add send from track {} to {} (amount: {})", track_id, target_track_id, amount);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        // Prevent duplicate sends to the same target
        if !track.sends.iter().any(|s| s.target_track_id == target_track_id) {
            track.sends.push(crate::state::SendRouting {
                target_track_id,
                amount,
            });
            state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
        }
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// トラックのセンドルーティングの送信量を設定する
#[tauri::command]
pub fn set_track_send_amount(track_id: usize, target_track_id: usize, amount: f32, state: State<'_, AppState>) -> Result<(), String> {
    info!("Routing: Set send amount from track {} to {} (amount: {})", track_id, target_track_id, amount);
    let mut project_state = state.engine.project_state.write().unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        if let Some(send) = track.sends.iter_mut().find(|s| s.target_track_id == target_track_id) {
            send.amount = amount;
            state.engine.history.write().unwrap_or_else(|e| e.into_inner()).save_snapshot(&project_state_snapshot);
            Ok(())
        } else {
            Err(format!("Send from track {} to {} not found", track_id, target_track_id))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}