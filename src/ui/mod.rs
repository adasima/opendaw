pub mod mixer;
pub mod timeline;
pub mod tracks;
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

/// メイン画面のUIを描画します。
pub fn draw_main_ui(app: &mut crate::app::AuraDawApp, ui: &mut egui::Ui) {
    crate::ui::mixer::draw_mixer_panel(ui, app);

    crate::ui::tracks::draw_tracks_panel(ui, app);

    #[allow(deprecated)]
    egui::SidePanel::right("ai_agent_panel")
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("AI Agent & CLI");
            ui.separator();
            ui.label("Agent is ready.");
            ui.text_edit_singleline(&mut "".to_string());
        });

    #[allow(deprecated)]
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("Main Timeline & Visualizer");

        crate::ui::transport::draw_transport(ui, app);
        ui.separator();

        crate::ui::timeline::draw_timeline(ui, app);
    });
}
