use crate::app::AuraDawApp;
use eframe::egui;

const PIANO_ROLL_BG_ROUNDING: f32 = 4.0;
const BEAT_WIDTH: f32 = 40.0;
const PITCH_HEIGHT: f32 = 12.0;
const MIN_PITCH: u8 = 48; // C3
const MAX_PITCH: u8 = 72; // C5
const VISIBLE_BEATS: f32 = 16.0;
const DEFAULT_VELOCITY: u8 = 100;
const DEFAULT_DURATION: f64 = 1.0;
const GRID_LINE_WIDTH: f32 = 1.0;
const NOTE_CORNER_RADIUS: f32 = 2.0;
const NOTE_STROKE_WIDTH: f32 = 2.0;
const NOTE_SHRINK: f32 = 1.0;

/// ピアノロールエディタ領域のUIを描画し、マウスクリックによるノートの追加・削除を処理します。
pub fn draw_piano_roll(ui: &mut egui::Ui, app: &mut AuraDawApp) {
    ui.heading("Piano Roll View");

    let num_pitches = (MAX_PITCH - MIN_PITCH + 1) as f32;
    let height = num_pitches * PITCH_HEIGHT;
    let width = VISIBLE_BEATS * BEAT_WIDTH;

    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, height),
        egui::Sense::click_and_drag(), // 左クリック、右クリック検知用
    );

    let painter = ui.painter();

    // 背景の描画
    painter.rect_filled(
        rect,
        PIANO_ROLL_BG_ROUNDING,
        egui::Color32::from_rgba_premultiplied(30, 32, 40, 200),
    );

    // グリッドの描画
    for i in 0..=(VISIBLE_BEATS as i32) {
        let x = rect.left() + (i as f32) * BEAT_WIDTH;
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(GRID_LINE_WIDTH, egui::Color32::from_gray(50)),
        );
    }
    for i in 0..=(num_pitches as i32) {
        let y = rect.top() + (i as f32) * PITCH_HEIGHT;
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(GRID_LINE_WIDTH, egui::Color32::from_gray(50)),
        );
    }

    // インタラクションの処理
    if let Some(hover_pos) = response.hover_pos() {
        let rel_x = hover_pos.x - rect.left();
        let rel_y = hover_pos.y - rect.top();

        let beat = (rel_x / BEAT_WIDTH).floor() as f64;
        let pitch_index = (rel_y / PITCH_HEIGHT).floor() as u8;
        let pitch = MAX_PITCH.saturating_sub(pitch_index); // 上がMAX_PITCH、下がMIN_PITCH

        if beat >= 0.0 && beat < (VISIBLE_BEATS as f64) && (MIN_PITCH..=MAX_PITCH).contains(&pitch)
        {
            // 左クリックでノート追加
            if response.clicked() {
                // 同じ位置にノートがないか確認
                let exists = app
                    .state
                    .active_sequence
                    .notes
                    .iter()
                    .any(|n| n.pitch == pitch && n.start_beat == beat);
                if !exists {
                    app.state.active_sequence.add_note(
                        pitch,
                        DEFAULT_VELOCITY,
                        beat,
                        DEFAULT_DURATION,
                    );
                }
            }

            // 右クリックでノート削除
            if response.secondary_clicked() {
                let note_to_remove = app
                    .state
                    .active_sequence
                    .notes
                    .iter()
                    .find(|n| {
                        n.pitch == pitch
                            && n.start_beat <= beat
                            && n.start_beat + n.duration_beats > beat
                    })
                    .map(|n| n.id);

                if let Some(id) = note_to_remove {
                    app.state.active_sequence.remove_note(id);
                }
            }
        }
    }

    // 既存ノートの描画とインタラクション
    let mut note_moved = None;
    let mut note_resized = None;
    let mut note_deleted = None;

    for note in &app.state.active_sequence.notes {
        if (MIN_PITCH..=MAX_PITCH).contains(&note.pitch) {
            let pitch_index = MAX_PITCH.saturating_sub(note.pitch) as f32;
            let y = rect.top() + pitch_index * PITCH_HEIGHT;
            let x = rect.left() + (note.start_beat as f32) * BEAT_WIDTH;
            let w = (note.duration_beats as f32) * BEAT_WIDTH;

            let note_rect =
                egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(w, PITCH_HEIGHT));

            let display_rect = note_rect.shrink(NOTE_SHRINK);

            // ノートごとのインタラクション
            let note_id = ui.id().with("note").with(note.id);
            let note_response = ui.interact(display_rect, note_id, egui::Sense::click_and_drag());

            // 右端のドラッグ判定
            let resize_margin = 6.0;
            let is_edge_hovered = note_response.hover_pos()
                .map(|p| p.x > display_rect.right() - resize_margin)
                .unwrap_or(false);

            if is_edge_hovered {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }

            // ドラッグ中のリサイズ状態を保持
            let resize_state_id = note_id.with("is_resizing");
            let mut is_resizing = ui.data_mut(|d| d.get_temp::<bool>(resize_state_id).unwrap_or(false));

            if note_response.drag_started()
                && note_response
                    .interact_pointer_pos()
                    .map(|pos| pos.x > display_rect.right() - resize_margin)
                    .unwrap_or(false)
            {
                is_resizing = true;
                ui.data_mut(|d| d.insert_temp(resize_state_id, true));
            }

            if note_response.drag_stopped() {
                is_resizing = false;
                ui.data_mut(|d| d.insert_temp(resize_state_id, false));
            }

            if note_response.dragged() {
                if is_resizing {
                    // リサイズ処理 (長さ変更)
                    let delta_x = note_response.drag_delta().x;
                    let delta_beats = (delta_x / BEAT_WIDTH) as f64;
                    let new_duration = (note.duration_beats + delta_beats).max(0.25); // 最低0.25拍
                    note_resized = Some((note.id, new_duration));
                } else {
                    // 移動処理 (位置・ピッチ変更)
                    let delta_x = note_response.drag_delta().x;
                    let delta_beats = (delta_x / BEAT_WIDTH) as f64;
                    let new_start = (note.start_beat + delta_beats).max(0.0);

                    // Y方向は絶対座標を使って新しいピッチを計算
                    let new_pitch = if let Some(pos) = note_response.interact_pointer_pos() {
                        let rel_y = pos.y - rect.top();
                        let pitch_index = (rel_y / PITCH_HEIGHT).floor() as u8;
                        MAX_PITCH.saturating_sub(pitch_index).clamp(MIN_PITCH, MAX_PITCH)
                    } else {
                        note.pitch
                    };

                    note_moved = Some((note.id, new_pitch, new_start));
                }
            }

            // ノート上での右クリックで削除
            if note_response.secondary_clicked() {
                note_deleted = Some(note.id);
            }

            // 描画状態の決定
            let fill_color = if note_response.dragged() {
                egui::Color32::from_rgb(150, 200, 255)
            } else if note_response.hovered() {
                egui::Color32::from_rgb(120, 180, 255)
            } else {
                egui::Color32::from_rgb(100, 150, 250)
            };

            painter.rect_filled(
                display_rect,
                NOTE_CORNER_RADIUS,
                fill_color,
            );
            painter.rect_stroke(
                display_rect,
                NOTE_CORNER_RADIUS,
                egui::Stroke::new(NOTE_STROKE_WIDTH, egui::Color32::from_rgb(50, 100, 200)),
                egui::StrokeKind::Middle,
            );
        }
    }

    // 状態の更新
    if let Some((id, pitch, start_beat)) = note_moved {
        app.state.active_sequence.move_note(id, pitch, start_beat);
    }
    if let Some((id, dur)) = note_resized {
        app.state.active_sequence.resize_note(id, dur);
    }
    if let Some(id) = note_deleted {
        app.state.active_sequence.remove_note(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_piano_roll_compiles() {
        // UI描画関数がシグネチャ通りに呼び出せるかの簡単なテスト
        let _f = draw_piano_roll as fn(&mut egui::Ui, &mut AuraDawApp);
    }

    #[test]
    fn test_piano_roll_constants() {
        // 定数の整合性テスト
        assert!(MIN_PITCH < MAX_PITCH);
        assert!(VISIBLE_BEATS > 0.0);
        assert!(BEAT_WIDTH > 0.0);
        assert!(PITCH_HEIGHT > 0.0);
    }
}
