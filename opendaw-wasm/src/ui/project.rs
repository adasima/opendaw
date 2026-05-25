use crate::app::OpenDawApp;
#[cfg(not(target_arch = "wasm32"))]
use crate::state::ProjectState;
use eframe::egui;
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;

/// プロジェクトの保存および読み込み用のUIを描画します。
#[allow(unused_variables)]
pub fn draw_project_ui(ui: &mut egui::Ui, _app: &mut OpenDawApp) {
    ui.horizontal(|ui| {
        if ui
            .button("💾 Save Project")
            .on_hover_text("現在のプロジェクトを保存します")
            .clicked()
        {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let picked_file = FileDialog::new()
                    .add_filter("Aura Project", &["aura"])
                    .save_file();

                if let Some(path) = picked_file {
                    let project_state = ProjectState::new(_app.state.clone());
                    if let Err(e) = project_state.save_to_file(&path) {
                        log::error!("Failed to save project: {}", e);
                    }
                }
            }
        }

        if ui
            .button("📂 Load Project")
            .on_hover_text("プロジェクトを読み込みます")
            .clicked()
        {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let picked_file = FileDialog::new()
                    .add_filter("Aura Project", &["aura"])
                    .pick_file();

                if let Some(path) = picked_file {
                    match ProjectState::load_from_file(&path) {
                        Ok(project_state) => {
                            _app.state = project_state.daw_state;
                            // オーディオエンジン等への同期が必要な場合はここに追加
                        }
                        Err(e) => {
                            log::error!("Failed to load project: {}", e);
                        }
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    // use super::*;

    // Since draw_project_ui relies on egui::Ui and rfd which interact with the system,
    // it's difficult to write standard unit tests for it without significant mocking.
    // However, we verify that the module compiles correctly.

    #[test]
    fn test_draw_project_ui_compiles() {
        // Just a dummy test to ensure the module is included in tests.
        assert!(true);
    }
}
