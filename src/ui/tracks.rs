use eframe::egui;
use crate::app::AuraDawApp;

/// トラックパネルのID
const TRACKS_PANEL_ID: &str = "tracks_panel";

/// トラック一覧パネルを描画します。
///
/// 左側に配置されるリサイズ可能なパネルで、現在のプロジェクトに
/// 存在する各トラック（ボーカル、シンセなど）のリストを表示します。
pub fn draw_tracks_panel(ui: &mut egui::Ui, _app: &mut AuraDawApp) {
    #[allow(deprecated)]
    egui::SidePanel::left(TRACKS_PANEL_ID)
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("Tracks");
            ui.separator();
            ui.label("Track 1 - Vocals");
            ui.label("Track 2 - Synth");
        });
}
