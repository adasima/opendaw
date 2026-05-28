use egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2};
use super::PianoRoll;

pub fn draw_keyboard(ui: &mut Ui, piano_roll: &PianoRoll, keyboard_rect: Rect, keys_width: f32) {
    let kb_painter = ui.painter().with_clip_rect(keyboard_rect);
    kb_painter.rect_filled(keyboard_rect, 0.0, Color32::from_gray(30));

    for p in 0..=127 {
        let y = keyboard_rect.min.y + (127 - p) as f32 * piano_roll.key_height - piano_roll.pan.y;
        if y + piano_roll.key_height < keyboard_rect.min.y || y > keyboard_rect.max.y {
            continue; // Skip off-screen
        }

        let is_black = matches!(p % 12, 1 | 3 | 6 | 8 | 10);
        let color = if is_black { Color32::from_gray(20) } else { Color32::from_gray(200) };
        let key_rect = Rect::from_min_size(Pos2::new(keyboard_rect.min.x, y), Vec2::new(keys_width, piano_roll.key_height));

        kb_painter.rect_filled(key_rect, 0.0, color);
        kb_painter.rect_stroke(key_rect, 0.0, Stroke::new(1.0, Color32::from_gray(50)), egui::StrokeKind::Inside);

        if p % 12 == 0 {
            kb_painter.text(
                key_rect.min + Vec2::new(5.0, 2.0),
                egui::Align2::LEFT_TOP,
                {
                    #[allow(clippy::unnecessary_cast)]
                    let octave = (p as i32 / 12) - 1;
                    format!("C{}", octave)
                },
                egui::FontId::proportional(12.0),
                if is_black { Color32::WHITE } else { Color32::BLACK }
            );
        }
    }
}
