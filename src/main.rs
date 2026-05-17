#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseビルド時にコンソールを隠す

pub mod ui;
pub mod engine;
pub mod state;
pub mod midi;
pub mod util;
mod app;
use app::AuraDawApp;

fn main() -> eframe::Result<()> {
    // 別のスレッドでTokioランタイムを起動し、eframe/winitはメインスレッドで実行する
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // ここで将来的にバックグラウンドのAPIサーバーなどを起動
            println!("Tokio background runtime started. Waiting for connections...");
            // ランタイムが終了しないように待機
            tokio::signal::ctrl_c().await.unwrap();
            println!("Tokio runtime shutting down.");
        });
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
        Box::new(|cc| Ok(Box::new(AuraDawApp::new(cc)))),
    )
}
