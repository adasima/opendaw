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
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) {
    // ログパニックフックの初期化
    console_error_panic_hook::set_once();
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    // MCPのチャンネル作成（WASMでは非同期処理やスレッド制限があるため、ここではダミーあるいはRxだけ渡す）
    let (_mcp_tx, mcp_rx) = mcp::channel::create_mcp_channel(100);

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
