#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod core;
pub mod engine;
pub mod mcp;
pub mod midi;
pub mod plugin;
pub mod state;
pub mod ui;
pub mod util;

pub use app::OpenDawApp;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
thread_local! {
    // スレッドローカルでMcpCommandSenderを保持し、Svelte側から関数を呼び出せるようにする
    static MCP_TX: RefCell<Option<crate::mcp::channel::McpCommandSender>> = RefCell::new(None);
}

// プレイヘッド位置のグローバル保存用
use std::sync::atomic::{AtomicU64, Ordering};
static PLAYHEAD_POS: AtomicU64 = AtomicU64::new(0);

pub fn set_playhead_pos(pos: f64) {
    PLAYHEAD_POS.store(pos.to_bits(), Ordering::Relaxed);
}

#[cfg(target_arch = "wasm32")]
use std::sync::Mutex;

#[cfg(target_arch = "wasm32")]
thread_local! {
    pub static EGUI_CTX: RefCell<Option<egui::Context>> = RefCell::new(None);
}

#[cfg(target_arch = "wasm32")]
lazy_static::lazy_static! {
    static ref TRACKS_JSON: Mutex<String> = Mutex::new("[]".to_string());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn set_tracks_json(json: String) {
    if let Ok(mut lock) = TRACKS_JSON.lock() {
        *lock = json;
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn request_repaint() {
    EGUI_CTX.with(|ctx| {
        if let Some(c) = &*ctx.borrow() {
            c.request_repaint();
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) {
    // ログパニックフックの初期化
    console_error_panic_hook::set_once();
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    // MCPのチャンネル作成
    let (mcp_tx, mcp_rx) = mcp::channel::create_mcp_channel(100);
    // WASM関数のために送信機を保持
    MCP_TX.with(|tx| *tx.borrow_mut() = Some(mcp_tx));

    let document = web_sys::window().expect("no global `window` exists").document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id(canvas_id)
        .expect("failed to find canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("element is not a canvas");

    wasm_bindgen_futures::spawn_local(async move {
        let result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(move |cc| Ok(Box::new(OpenDawApp::new(cc, Some(mcp_rx))))),
            )
            .await;

        if let Err(e) = result {
            log::error!("Failed to start eframe: {:?}", e);
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn play() {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let _ = sender.send(crate::mcp::channel::McpCommand::Play);
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn stop() {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let _ = sender.send(crate::mcp::channel::McpCommand::Stop);
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn toggle_loop() {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let _ = sender.send(crate::mcp::channel::McpCommand::ToggleLoop);
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn set_master_volume(volume: f32) {
    // 現在のMcpCommandはマスターボリュームの直接変更をサポートしていないため、
    // 将来の拡張用としてプレースホルダー実装にしておきます。
    log::info!("set_master_volume called from Svelte: {}", volume);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_playhead_pos() -> f64 {
    f64::from_bits(PLAYHEAD_POS.load(Ordering::Relaxed))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn select_track(id: i32) {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let track_id = if id < 0 { None } else { Some(id as usize) };
            let _ = sender.send(crate::mcp::channel::McpCommand::SelectTrack(track_id));
        }
    });
    request_repaint();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn toggle_mute(id: usize) {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let _ = sender.send(crate::mcp::channel::McpCommand::ToggleMute(id));
        }
    });
    request_repaint();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn toggle_solo(id: usize) {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let _ = sender.send(crate::mcp::channel::McpCommand::ToggleSolo(id));
        }
    });
    request_repaint();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn toggle_record_arm(id: usize) {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let _ = sender.send(crate::mcp::channel::McpCommand::ToggleRecordArm(id));
        }
    });
    request_repaint();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn add_track() {
    MCP_TX.with(|tx| {
        if let Some(sender) = tx.borrow().as_ref() {
            let _ = sender.send(crate::mcp::channel::McpCommand::AddTrack);
        }
    });
    request_repaint();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_tracks_json() -> String {
    if let Ok(lock) = TRACKS_JSON.lock() {
        lock.clone()
    } else {
        "[]".to_string()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    fn tauri_invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

#[cfg(target_arch = "wasm32")]
pub fn notify_clip_moved(track_id: usize, clip_id: usize, new_start_pos: f32) {
    #[derive(serde::Serialize)]
    struct MoveClipArgs {
        track_id: usize,
        clip_id: usize,
        start_pos: f32,
    }

    let args = MoveClipArgs {
        track_id,
        clip_id,
        start_pos: new_start_pos,
    };

    if let Ok(js_value) = serde_wasm_bindgen::to_value(&args) {
        let _ = tauri_invoke("move_audio_clip", js_value);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn notify_midi_clip_moved(track_id: usize, clip_id: usize, new_start_beat: f64) {
    #[derive(serde::Serialize)]
    struct MoveMidiClipArgs {
        track_id: usize,
        clip_id: usize,
        new_start_beat: f64,
    }

    let args = MoveMidiClipArgs {
        track_id,
        clip_id,
        new_start_beat,
    };

    if let Ok(js_value) = serde_wasm_bindgen::to_value(&args) {
        let _ = tauri_invoke("move_midi_clip", js_value);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn notify_update_midi_clip_notes(track_id: usize, clip_id: usize, notes: &Vec<crate::midi::sequence::NoteEvent>) {
    #[derive(serde::Serialize)]
    struct UpdateMidiClipNotesArgs<'a> {
        track_id: usize,
        clip_id: usize,
        notes: &'a Vec<crate::midi::sequence::NoteEvent>,
    }

    let args = UpdateMidiClipNotesArgs {
        track_id,
        clip_id,
        notes,
    };

    if let Ok(js_value) = serde_wasm_bindgen::to_value(&args) {
        let _ = tauri_invoke("update_midi_clip_notes", js_value);
    }
}
