use crate::app::AuraDawApp;
use eframe::egui;

/// AIエージェントパネルを描画します。
pub fn draw_ai_agent_panel(ui: &mut egui::Ui, _app: &mut AuraDawApp) {
    #[allow(deprecated)]
    egui::SidePanel::right("ai_agent_panel")
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("AI Agent & CLI");
            ui.separator();
            ui.label("Agent is ready.");
            ui.text_edit_singleline(&mut "".to_string());
        });
}
