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
