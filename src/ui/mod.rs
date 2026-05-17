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

/// メイン画面のUIを描画します。
pub fn draw_main_ui(app: &mut crate::app::AuraDawApp, ui: &mut egui::Ui) {
    #[allow(deprecated)]
    egui::TopBottomPanel::bottom("mixer_panel")
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("Mixer & Effects");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Master Volume");
                ui.add(egui::Slider::new(&mut app.master_volume, 0.0..=1.0));

                let mute_icon = if app.is_muted { "🔇" } else { "🔊" };
                if ui.button(mute_icon).on_hover_text("Mute/Unmute").clicked() {
                    app.toggle_mute();
                }
            });
        });

    #[allow(deprecated)]
    egui::SidePanel::left("tracks_panel")
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("Tracks");
            ui.separator();
            ui.label("Track 1 - Vocals");
            ui.label("Track 2 - Synth");
        });

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
