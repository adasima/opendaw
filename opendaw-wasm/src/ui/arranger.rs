use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2, PointerButton};
use crate::state::DawState;

pub struct ArrangerState {
    pub pan: Vec2,
    pub pixels_per_tick: f32,
    pub track_height: f32,
    pub ticks_per_beat: u32,
}

impl Default for ArrangerState {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            pixels_per_tick: 0.1,
            track_height: 80.0,
            ticks_per_beat: 480,
        }
    }
}

pub fn draw_arranger(ui: &mut egui::Ui, app: &mut crate::app::OpenDawApp) {
    let state = &mut app.state;
    // (将来的にApp構造体にArrangerStateを持たせますが、今回はモックとして関数内で状態を作ります)
    let mut arranger = ArrangerState::default();

    let response = ui.allocate_response(ui.available_size(), Sense::click_and_drag());
    let rect = response.rect;

    // ヘッダー（ルーラー）の高さ
    let ruler_height = 24.0;
    let ruler_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), ruler_height));
    let track_area_rect = Rect::from_min_max(Pos2::new(rect.min.x, rect.min.y + ruler_height), rect.max);

    let grid_painter = ui.painter().with_clip_rect(rect);

    // 背景
    grid_painter.rect_filled(rect, 0.0, Color32::from_gray(20));
    
    // ルーラー背景
    grid_painter.rect_filled(ruler_rect, 0.0, Color32::from_gray(35));
    grid_painter.line_segment(
        [Pos2::new(ruler_rect.min.x, ruler_rect.max.y), Pos2::new(ruler_rect.max.x, ruler_rect.max.y)],
        Stroke::new(1.0, Color32::from_gray(60))
    );

    // 縦線（拍・小節）の描画
    let min_tick = (arranger.pan.x / arranger.pixels_per_tick).max(0.0) as u32;
    let max_tick = min_tick + (rect.width() / arranger.pixels_per_tick) as u32;
    
    let snap_step = arranger.ticks_per_beat; // 1拍ごとに線
    let mut t = (min_tick / snap_step) * snap_step;
    while t <= max_tick {
        let x = rect.min.x + t as f32 * arranger.pixels_per_tick - arranger.pan.x;
        let is_bar = t % (arranger.ticks_per_beat * 4) == 0;
        
        let stroke = if is_bar {
            Stroke::new(1.5, Color32::from_gray(70))
        } else {
            Stroke::new(1.0, Color32::from_gray(40))
        };
        
        // トラックエリアのグリッド線
        grid_painter.line_segment(
            [Pos2::new(x, track_area_rect.min.y), Pos2::new(x, track_area_rect.max.y)],
            stroke
        );

        // ルーラーの目盛り
        if is_bar {
            let bar_number = (t / (arranger.ticks_per_beat * 4)) + 1;
            grid_painter.text(
                Pos2::new(x + 4.0, ruler_rect.min.y + 4.0),
                egui::Align2::LEFT_TOP,
                format!("{}", bar_number),
                egui::FontId::proportional(12.0),
                Color32::from_gray(180)
            );
        }
        
        t += snap_step;
    }

    // トラックレーンのモック描画
    let num_mock_tracks = 4;
    for i in 0..num_mock_tracks {
        let y = track_area_rect.min.y + (i as f32 * arranger.track_height) - arranger.pan.y;
        if y > track_area_rect.max.y { break; }
        
        // レーンの区切り線
        grid_painter.line_segment(
            [Pos2::new(track_area_rect.min.x, y + arranger.track_height), Pos2::new(track_area_rect.max.x, y + arranger.track_height)],
            Stroke::new(1.0, Color32::from_gray(50))
        );

        // モックのクリップ描画
        if i == 0 { // トラック1にクリップ
            let clip_x1 = rect.min.x + 480.0 * arranger.pixels_per_tick - arranger.pan.x;
            let clip_x2 = rect.min.x + 1920.0 * arranger.pixels_per_tick - arranger.pan.x;
            let clip_rect = Rect::from_min_max(Pos2::new(clip_x1, y + 4.0), Pos2::new(clip_x2, y + arranger.track_height - 4.0));
            
            grid_painter.rect_filled(clip_rect, 4.0, Color32::from_rgb(80, 140, 200));
            grid_painter.rect_stroke(clip_rect, 4.0, Stroke::new(1.0, Color32::from_rgb(100, 180, 250)), egui::StrokeKind::Inside);
            grid_painter.text(
                clip_rect.min + Vec2::new(6.0, 6.0),
                egui::Align2::LEFT_TOP,
                "Audio Clip",
                egui::FontId::proportional(12.0),
                Color32::WHITE
            );
        }
    }

    // プレイヘッドの描画
    let playhead_x = rect.min.x + (state.playhead_pos * arranger.ticks_per_beat as f32) * arranger.pixels_per_tick - arranger.pan.x;
    if playhead_x >= rect.min.x && playhead_x <= rect.max.x {
        // 白い縦線
        grid_painter.line_segment(
            [Pos2::new(playhead_x, rect.min.y), Pos2::new(playhead_x, rect.max.y)],
            Stroke::new(2.0, Color32::from_rgb(255, 255, 255))
        );
        // 上部の逆三角形
        let triangle_points = vec![
            Pos2::new(playhead_x - 6.0, ruler_rect.min.y),
            Pos2::new(playhead_x + 6.0, ruler_rect.min.y),
            Pos2::new(playhead_x, ruler_rect.min.y + 12.0),
        ];
        grid_painter.add(egui::Shape::convex_polygon(
            triangle_points,
            Color32::from_rgb(255, 255, 255),
            Stroke::NONE
        ));
    }
}
