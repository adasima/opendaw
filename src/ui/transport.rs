use crate::app::AuraDawApp;
use eframe::egui;

const BPM_MIN: f64 = 20.0;
const BPM_MAX: f64 = 300.0;
const TIME_SCALER: f32 = 0.1;

/// トランスポートコントロール（再生、停止、ループ、BPMなど）を描画します。
pub fn draw_transport(ui: &mut egui::Ui, app: &mut AuraDawApp) {
    ui.horizontal(|ui| {
        let play_icon = if app.is_playing { "⏸" } else { "▶" };
        if ui.button(play_icon).on_hover_text("Play/Pause").clicked() {
            app.toggle_playback();
        }
        if ui.button("⏹").on_hover_text("Stop").clicked() {
            app.stop_playback();
        }

        let loop_icon = if app.is_looping {
            "🔁 (On)"
        } else {
            "🔁 (Off)"
        };
        if ui.button(loop_icon).on_hover_text("Toggle Loop").clicked() {
            app.toggle_loop();
        }

        ui.separator();

        // BPMコントロール
        ui.add(
            egui::DragValue::new(&mut app.bpm)
                .range(BPM_MIN..=BPM_MAX)
                .prefix("BPM: "),
        );

        ui.separator();

        // 仮想的なタイム表示 (MM:SS.ms などに見立てる)
        ui.label(format!("Time: {:.1}s", app.playhead_pos * TIME_SCALER));
    });
}
