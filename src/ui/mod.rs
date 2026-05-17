pub mod timeline;
pub mod transport;

use eframe::egui;

/// カスタムスタイル（ダークテーマ・グラスモーフィズム風）を設定します。
pub fn setup_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();

    // Discordにインスパイアされたダークテーマ/グラスモーフィズム風のスタイル調整
    style.visuals = egui::Visuals::dark();
    style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(18, 19, 24, 230); // 半透明の暗い背景
    style.visuals.panel_fill = egui::Color32::from_rgb(18, 19, 24);

    ctx.set_global_style(style);
}
