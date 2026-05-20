use crate::app::AuraDawApp;
use eframe::egui;

/// トラックパネルのID
const TRACKS_PANEL_ID: &str = "tracks_panel";

/// トラック一覧パネルを描画します。
///
/// 左側に配置されるリサイズ可能なパネルで、現在のプロジェクトに
/// 存在する各トラック（ボーカル、シンセなど）のリストを表示します。
pub fn draw_tracks_panel(ui: &mut egui::Ui, app: &mut AuraDawApp) {
    #[allow(deprecated)]
    egui::SidePanel::left(TRACKS_PANEL_ID)
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Tracks");
                if ui.button("+ Add Track").clicked() {
                    let next_num = app.state.next_track_id;
                    app.state.add_track(format!("Track {}", next_num));
                }
            });
            ui.separator();

            let mut track_to_remove = None;

            for track in &app.state.tracks {
                ui.horizontal(|ui| {
                    ui.label(format!("ID: {} - {}", track.id, track.name));
                    if ui.button("X").clicked() {
                        track_to_remove = Some(track.id);
                    }
                });
            }

            if let Some(id) = track_to_remove {
                app.state.remove_track(id);
            }
        });
}
