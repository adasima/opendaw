use egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2};
use super::PianoRoll;

pub fn draw_grid(ui: &mut Ui, app: &crate::app::OpenDawApp, piano_roll: &PianoRoll, grid_rect: Rect) {
    let grid_painter = ui.painter().with_clip_rect(grid_rect);
    grid_painter.rect_filled(grid_rect, 0.0, Color32::from_gray(40));

    // Horizontal key lanes
    for p in 0..=127 {
        let y = grid_rect.min.y + (127 - p) as f32 * piano_roll.key_height - piano_roll.pan.y;
        if y + piano_roll.key_height < grid_rect.min.y || y > grid_rect.max.y {
            continue;
        }
        let is_black = matches!(p % 12, 1 | 3 | 6 | 8 | 10);
        if is_black {
            grid_painter.rect_filled(
                Rect::from_min_size(Pos2::new(grid_rect.min.x, y), Vec2::new(grid_rect.width(), piano_roll.key_height)),
                0.0,
                Color32::from_gray(35)
            );
        }
        grid_painter.line_segment(
            [Pos2::new(grid_rect.min.x, y), Pos2::new(grid_rect.max.x, y)],
            Stroke::new(1.0, Color32::from_gray(50))
        );
    }

    // Vertical beat/bar lines
    let min_tick = (piano_roll.pan.x / piano_roll.pixels_per_tick).max(0.0) as u32;
    let max_tick = min_tick + (grid_rect.width() / piano_roll.pixels_per_tick) as u32;

    let snap_step = if app.state.is_grid_enabled {
        piano_roll.ticks_per_beat / app.state.grid_resolution
    } else {
        piano_roll.ticks_per_beat / 4 // fallback to visual default
    };

    let mut t = (min_tick / snap_step) * snap_step;
    while t <= max_tick {
        let x = grid_rect.min.x + t as f32 * piano_roll.pixels_per_tick - piano_roll.pan.x;
        #[allow(clippy::manual_is_multiple_of)]
        let is_beat = t % piano_roll.ticks_per_beat == 0;
        #[allow(clippy::manual_is_multiple_of)]
        let is_bar = t % (piano_roll.ticks_per_beat * 4) == 0;

        let stroke = if is_bar {
            Stroke::new(2.0, Color32::from_gray(100))
        } else if is_beat {
            Stroke::new(1.0, Color32::from_gray(80))
        } else {
            Stroke::new(1.0, Color32::from_gray(50))
        };

        grid_painter.line_segment(
            [Pos2::new(x, grid_rect.min.y), Pos2::new(x, grid_rect.max.y)],
            stroke
        );
        t += snap_step;
    }
}
