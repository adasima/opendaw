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

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .expect("failed to find canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

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
}
