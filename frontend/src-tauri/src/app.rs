use log::info;

/// 再生を開始する
#[tauri::command]
pub fn play() {
    info!("Transport: Play");
}

/// 再生を一時停止する
#[tauri::command]
pub fn pause() {
    info!("Transport: Pause");
}

/// 再生を停止する
#[tauri::command]
pub fn stop() {
    info!("Transport: Stop");
}

/// BPMを設定する
#[tauri::command]
pub fn set_bpm(bpm: f64) {
    info!("Transport: Set BPM to {}", bpm);
}
