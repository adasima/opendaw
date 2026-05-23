#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseビルド時にコンソールを隠す

mod app;
pub mod core;
pub mod engine;
pub mod mcp;
pub mod midi;
pub mod state;
pub mod ui;
pub mod util;
pub mod plugin;
use app::AuraDawApp;

const MCP_CHANNEL_CAPACITY: usize = 100;

fn main() -> eframe::Result<()> {
    let (mcp_tx, mcp_rx) = crate::mcp::channel::create_mcp_channel(MCP_CHANNEL_CAPACITY);

    // 別のスレッドでTokioランタイムを起動し、eframe/winitはメインスレッドで実行する
    std::thread::spawn(move || {
        match tokio::runtime::Runtime::new() {
            Ok(rt) => {
                rt.block_on(async move {
                    println!("Tokio background runtime started. Waiting for connections...");
                    let mut mcp_server = crate::mcp::McpServer::new();
                    mcp_server.transport_handler.sender = Some(mcp_tx.clone());
                    mcp_server.tracks_handler.sender = Some(mcp_tx);

                    tokio::spawn(async move {
                        mcp_server.run().await;
                    });
                    // ランタイムが終了しないように待機
                    if let Err(e) = tokio::signal::ctrl_c().await {
                        eprintln!("Failed to wait for ctrl-c: {}", e);
                    }
                    println!("Tokio runtime shutting down.");
                });
            }
            Err(e) => {
                eprintln!("Failed to start Tokio runtime: {}", e);
            }
        }
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Next-Gen AI DAW - AuraDAW")
            .with_transparent(true), // 可能なら透過を有効化（グラスモーフィズム用）
        ..Default::default()
    };

    eframe::run_native(
        "AuraDAW",
        native_options,
        Box::new(move |cc| Ok(Box::new(AuraDawApp::new(cc, Some(mcp_rx))))),
    )
}
