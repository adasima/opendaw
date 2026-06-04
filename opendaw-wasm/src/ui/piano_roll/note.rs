use super::PianoRoll;
use egui::{Color32, Pos2, Rect, Stroke, Ui};

pub fn draw_notes(
    ui: &mut Ui,
    app: &crate::app::OpenDawApp,
    piano_roll: &PianoRoll,
    grid_rect: Rect,
) {
    let grid_painter = ui.painter().with_clip_rect(grid_rect);

    // 3. Draw Notes and Lyrics
    for note in &app.state.active_sequence.notes {
        let note_rect = piano_roll.note_rect(note, grid_rect.min);
        if note_rect.max.x < grid_rect.min.x
            || note_rect.min.x > grid_rect.max.x
            || note_rect.max.y < grid_rect.min.y
            || note_rect.min.y > grid_rect.max.y
        {
            continue; // Skip off-screen
        }

        let is_dragged = piano_roll
            .dragging_note
            .map(|(id, _)| id == note.id)
            .unwrap_or_default()
            || piano_roll.resizing_note == Some(note.id);
        let fill_color = if is_dragged {
            Color32::from_rgb(150, 220, 255) // Brighter when dragged
        } else {
            Color32::from_rgb(80, 180, 250)
        };

        let display_rect = note_rect.shrink(1.0);
        grid_painter.rect_filled(display_rect, 2.0, fill_color);
        grid_painter.rect_stroke(
            display_rect,
            2.0,
            Stroke::new(1.0, Color32::from_rgb(30, 100, 180)),
            egui::StrokeKind::Inside,
        );

        // Draw Lyrics (ARA2 / SV2 feature) - Not supported on NoteEvent yet
    }

    // 4. Draw Pitch Bend Curve (ARA2 / SV2 feature)
    if !piano_roll.pitch_bend_curve.is_empty() {
        let mut points = Vec::new();
        for &(tick, value) in &piano_roll.pitch_bend_curve {
            let x = grid_rect.min.x + tick as f32 * piano_roll.pixels_per_tick - piano_roll.pan.x;
            // Draw curve at the bottom of the grid
            let base_y = grid_rect.max.y - 40.0;
            let amplitude = 30.0;
            let y = base_y - (value * amplitude);
            points.push(Pos2::new(x, y));
        }

        if points.len() > 1 {
            // Draw a line for the pitch bend curve
            grid_painter.add(egui::Shape::line(
                points.clone(),
                Stroke::new(2.0, Color32::from_rgba_premultiplied(255, 100, 100, 200)),
            ));

            // Draw points on the curve
            for &p in &points {
                if grid_rect.contains(p) {
                    grid_painter.circle_filled(p, 3.0, Color32::from_rgb(255, 50, 50));
                }
            }
        }
    }
}
