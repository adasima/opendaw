use crate::app::AuraDawApp;
use eframe::egui;

const WAVEFORM_HEIGHT: f32 = 50.0;
const WAVEFORM_SEGMENTS: usize = 100;
const WAVEFORM_SPEED_SCALER: f32 = 0.1;
const BG_ROUNDING: f32 = 4.0;

/// メインタイムライン領域（波形描画、プレイヘッドなど）を描画します。
pub fn draw_timeline(ui: &mut egui::Ui, app: &mut AuraDawApp) {
    // 波形のプレースホルダー領域
    let (rect, response) =
        ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

    if let Some(pos) = response
        .interact_pointer_pos()
        .filter(|_| response.clicked() || response.dragged())
    {
        let relative_x = pos.x - rect.left();
        let percentage = (relative_x / rect.width()) * 100.0;
        app.state.seek_to(percentage);
    }

    // 簡単な波形描画のモック
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        BG_ROUNDING,
        egui::Color32::from_rgba_premultiplied(22, 24, 28, 180),
    );

    let center_y = rect.center().y;
    for i in 0..WAVEFORM_SEGMENTS {
        let x = rect.left() + (rect.width() / WAVEFORM_SEGMENTS as f32) * i as f32;
        let height = (i as f32 * WAVEFORM_SPEED_SCALER).sin().abs() * WAVEFORM_HEIGHT;
        painter.line_segment(
            [
                egui::pos2(x, center_y - height),
                egui::pos2(x, center_y + height),
            ],
            egui::Stroke::new(2.0, egui::Color32::from_rgb(114, 137, 218)), // アクセントカラー
        );
    }

    // プレイヘッド（縦線）の描画
    let playhead_x = rect.left() + (rect.width() / 100.0) * app.state.playhead_pos;
    painter.line_segment(
        [
            egui::pos2(playhead_x, rect.top()),
            egui::pos2(playhead_x, rect.bottom()),
        ],
        egui::Stroke::new(2.0, egui::Color32::RED),
    );
}
