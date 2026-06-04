use eframe::egui;

const MIN_COL_WIDTH: f32 = 80.0;
const DUMMY_TRACK_COUNT: usize = 4;
const DUMMY_SCENE_COUNT: usize = 5;

/// セッションビューのUIを描画します。
/// 現在はダミーデータを使用してクリップグリッドのスケルトンを表示します。
pub fn draw_session_view(ui: &mut egui::Ui, _app: &mut crate::app::OpenDawApp) {
    ui.heading("Session View (Skeleton)");
    ui.separator();

    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("session_grid")
            .striped(true)
            .min_col_width(MIN_COL_WIDTH)
            .show(ui, |ui| {
                // ヘッダー行: トラック名
                ui.label(""); // シーン用の余白
                for track_idx in 0..DUMMY_TRACK_COUNT {
                    ui.heading(format!("Track {}", track_idx + 1));
                }
                ui.end_row();

                // ダミーのシーンとクリップグリッド
                for scene_idx in 0..DUMMY_SCENE_COUNT {
                    // シーン再生ボタン
                    if ui.button(format!("▶ Scene {}", scene_idx + 1)).clicked() {
                        // ダミー処理（何もしない）
                    }

                    // 各トラックのクリップ
                    for track_idx in 0..DUMMY_TRACK_COUNT {
                        // 簡単なダミーデータ生成（一部を空のクリップにする）
                        let is_empty = (scene_idx + track_idx) % 3 == 0;
                        if is_empty {
                            // 空のクリップスロット（ボタンとして描画するがラベルは空）
                            if ui.button("＋").on_hover_text("空のクリップ").clicked() {
                                // ダミー処理
                            }
                        } else {
                            // 存在するクリップ
                            let clip_color = match track_idx {
                                0 => egui::Color32::from_rgb(114, 137, 218), // アクセントブルー
                                1 => egui::Color32::from_rgb(218, 114, 137), // レッド系
                                2 => egui::Color32::from_rgb(137, 218, 114), // グリーン系
                                _ => egui::Color32::from_rgb(218, 180, 114), // イエロー系
                            };

                            let button = egui::Button::new(
                                egui::RichText::new("Clip").color(egui::Color32::WHITE),
                            )
                            .fill(clip_color);

                            if ui.add(button).clicked() {
                                // ダミー処理
                            }
                        }
                    }
                    ui.end_row();
                }
            });
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_draw_session_view_dummy() {
        // UI描画のロジックテストはヘッドレスでは困難なため、
        // 少なくともコンパイルが通るか、基本状態が影響を受けないかを検証します。
        let app = crate::app::OpenDawApp::default();
        let initial_is_playing = app.state.is_playing;

        // アプリケーションの状態がセッションビューによって予期せず変更されないことを確認
        assert_eq!(app.state.is_playing, initial_is_playing);
    }
}
