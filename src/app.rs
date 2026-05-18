use eframe::egui;
use crate::state::DawState;

#[derive(Default)]
pub struct AuraDawApp {
    pub state: DawState,
}

impl AuraDawApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // カスタムフォントやスタイルなどをここで設定
        crate::ui::setup_custom_style(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for AuraDawApp {
    // Eframe 0.34
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // キーボードショートカット: スペースキーで再生/停止
        let focused = ctx.memory(|mem| mem.focused());
        if focused.is_none() && ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.state.toggle_playback();
        }

        // 再生中の場合、プレイヘッドを進行させて再描画を要求
        if self.state.is_playing {
            self.state.tick_playback();
            ctx.request_repaint();
        }

        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
             crate::ui::draw_main_ui(self, ui);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::ui::draw_main_ui(self, ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        // App構造体の初期化が正常にできるか確認
        // eframe::CreationContextをモックするのは難しいため、
        // Default::default() で状態が初期化されることのみを確認します。
        let app = AuraDawApp::default();
        assert!(!app.state.is_playing);
        assert_eq!(app.state.playhead_pos, 0.0);
    }
}
