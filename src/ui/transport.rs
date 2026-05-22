use crate::app::AuraDawApp;
use eframe::egui;
use ringbuf::traits::Producer;

const BPM_MIN: f64 = 20.0;
const BPM_MAX: f64 = 300.0;
const TIME_SCALER: f32 = 0.1;

/// トランスポートコントロール（再生、停止、ループ、BPMなど）を描画します。
pub fn draw_transport(ui: &mut egui::Ui, app: &mut AuraDawApp) {
    ui.horizontal(|ui| {
        let play_icon = if app.state.is_playing { "⏸" } else { "▶" };

        let record_icon = if app.state.is_recording { "⏺ (On)" } else { "⏺" };
        let record_color = if app.state.is_recording {
            egui::Color32::RED
        } else {
            ui.visuals().text_color()
        };
        if ui.add(egui::Button::new(egui::RichText::new(record_icon).color(record_color))).on_hover_text("Record").clicked() {
            app.state.toggle_recording();
        }
        if ui.button(play_icon).on_hover_text("Play/Pause").clicked() {
            app.state.toggle_playback();
            if let Some(ui_channels) = &mut app.ui_channels {
                let send_result =
                    ui_channels
                        .0
                        .try_push(crate::engine::channel::UiToAudioMsg::SetPlaying(
                            app.state.is_playing,
                        ));
                if send_result.is_err() {
                    log::warn!("Failed to send SetPlaying message: channel full");
                }
            }
        }
        if ui.button("⏹").on_hover_text("Stop").clicked() {
            app.state.stop_playback();
            if let Some(ui_channels) = &mut app.ui_channels {
                let send_result = ui_channels
                    .0
                    .try_push(crate::engine::channel::UiToAudioMsg::SetPlaying(false));
                if send_result.is_err() {
                    log::warn!("Failed to send SetPlaying message: channel full");
                }
            }
        }

        let loop_icon = if app.state.is_looping {
            "🔁 (On)"
        } else {
            "🔁 (Off)"
        };
        if ui.button(loop_icon).on_hover_text("Toggle Loop").clicked() {
            app.state.toggle_loop();
        }

        let metronome_icon = if app.state.is_metronome_enabled {
            "⏱ (On)"
        } else {
            "⏱ (Off)"
        };
        if ui.button(metronome_icon).on_hover_text("Toggle Metronome").clicked() {
            app.state.toggle_metronome();
        }

        ui.separator();

        // BPMコントロール
        ui.add(
            egui::DragValue::new(&mut app.state.bpm)
                .range(BPM_MIN..=BPM_MAX)
                .prefix("BPM: "),
        );

        ui.separator();

        // 仮想的なタイム表示 (MM:SS.ms などに見立てる)
        ui.label(format!(
            "Time: {:.1}s",
            app.state.playhead_pos * TIME_SCALER
        ));
    });
}
