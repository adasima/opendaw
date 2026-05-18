use eframe::egui;

/// Mixer & Effects パネルを描画します。
pub fn draw_mixer_panel(ui: &mut egui::Ui, app: &mut crate::app::AuraDawApp) {
    #[allow(deprecated)]
    egui::TopBottomPanel::bottom("mixer_panel")
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("Mixer & Effects");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Master Volume");
                ui.add(egui::Slider::new(&mut app.state.master_volume, 0.0..=1.0));

                let mute_icon = if app.state.is_muted { "🔇" } else { "🔊" };
                if ui.button(mute_icon).on_hover_text("Mute/Unmute").clicked() {
                    app.state.toggle_mute();
                }
            });
        });
}
