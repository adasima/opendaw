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
                if ui.button("+ Add Synth Track").clicked() {
                    let next_num = app.state.next_track_id;
                    app.state.add_track(format!("Synth {}", next_num));
                    if let Some(track) = app.state.tracks.last_mut() {
                        track.synth.is_enabled = true;
                    }
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

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::app::AuraDawApp;


    #[test]
    fn test_add_synth_track_logic() {
        // UIスレッドを模倣して、トラック追加ロジックの動作を確認する
        let mut app = AuraDawApp::default();

        let initial_count = app.state.tracks.len();

        // Synthトラック追加時のロジック
        let next_num = app.state.next_track_id;
        app.state.add_track(format!("Synth {}", next_num));
        if let Some(track) = app.state.tracks.last_mut() {
            track.synth.is_enabled = true;
        }

        assert_eq!(app.state.tracks.len(), initial_count + 1);
        if let Some(last_track) = app.state.tracks.last() {
            assert!(last_track.name.starts_with("Synth"));
            assert!(last_track.synth.is_enabled);
        } else {
            panic!("Synth track was not added");
        }
    }
}
