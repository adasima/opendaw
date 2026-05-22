use crate::app::AuraDawApp;
use eframe::egui;

const BG_ROUNDING: f32 = 4.0;
const TRACK_HEIGHT: f32 = 80.0;
const TIMELINE_PERCENT_MAX: f32 = 100.0;
const CLIP_MARGIN_Y: f32 = 4.0;
const CLIP_PADDING: f32 = 4.0;
const CLIP_BG_ROUNDING: f32 = 4.0;
const TEXT_SIZE: f32 = 12.0;
const WAVEFORM_STROKE_WIDTH: f32 = 1.0;
const PLAYHEAD_STROKE_WIDTH: f32 = 2.0;


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
        let percentage = (relative_x / rect.width()) * TIMELINE_PERCENT_MAX;
        app.state.seek_to(percentage);
    }

    let painter = ui.painter();
    painter.rect_filled(
        rect,
        BG_ROUNDING,
        egui::Color32::from_rgba_premultiplied(18, 19, 24, 255),
    );

    for (i, track) in app.state.tracks.iter().enumerate() {
        let track_top = rect.top() + (i as f32) * TRACK_HEIGHT;
        let track_rect = egui::Rect::from_min_max(
            egui::pos2(rect.left(), track_top),
            egui::pos2(rect.right(), track_top + TRACK_HEIGHT),
        );

        let bg_color = if i % 2 == 0 {
            egui::Color32::from_rgba_premultiplied(30, 32, 38, 180)
        } else {
            egui::Color32::from_rgba_premultiplied(22, 24, 28, 180)
        };
        painter.rect_filled(track_rect, 0.0, bg_color);

        for clip in &track.clips {
            let clip_x = rect.left() + (rect.width() / TIMELINE_PERCENT_MAX) * clip.start_pos;
            let clip_w = (rect.width() / TIMELINE_PERCENT_MAX) * clip.length;
            let clip_rect = egui::Rect::from_min_size(
                egui::pos2(clip_x, track_rect.top() + CLIP_MARGIN_Y),
                egui::vec2(clip_w, TRACK_HEIGHT - (CLIP_MARGIN_Y * 2.0)),
            );

            painter.rect_filled(
                clip_rect,
                CLIP_BG_ROUNDING,
                egui::Color32::from_rgba_premultiplied(50, 60, 90, 200),
            );

            painter.text(
                clip_rect.left_top() + egui::vec2(CLIP_PADDING, CLIP_PADDING),
                egui::Align2::LEFT_TOP,
                &clip.name,
                egui::FontId::proportional(TEXT_SIZE),
                egui::Color32::WHITE,
            );

            if !clip.waveform_summary.is_empty() {
                let center_y = clip_rect.center().y;
                let max_h = clip_rect.height() / 2.0 - CLIP_PADDING;
                let step = clip_rect.width() / clip.waveform_summary.len() as f32;
                for (j, &val) in clip.waveform_summary.iter().enumerate() {
                    let wx = clip_rect.left() + j as f32 * step;
                    let h = val.clamp(0.0, 1.0) * max_h;
                    painter.line_segment(
                        [egui::pos2(wx, center_y - h), egui::pos2(wx, center_y + h)],
                        egui::Stroke::new(WAVEFORM_STROKE_WIDTH, egui::Color32::from_rgb(114, 137, 218)),
                    );
                }
            }
        }
    }

    // プレイヘッド（縦線）の描画
    let playhead_x = rect.left() + (rect.width() / TIMELINE_PERCENT_MAX) * app.state.playhead_pos;
    painter.line_segment(
        [
            egui::pos2(playhead_x, rect.top()),
            egui::pos2(playhead_x, rect.bottom()),
        ],
        egui::Stroke::new(PLAYHEAD_STROKE_WIDTH, egui::Color32::RED),
    );
}
