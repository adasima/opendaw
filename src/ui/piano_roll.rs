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

    // 現在選択されているトラックのインデックスを取得（ここではUIで選択させる仕組みがないため、最初のシンセトラックを使用）
    let mut target_track_idx = None;
    for (i, track) in app.state.tracks.iter().enumerate() {
        if track.synth.is_enabled {
            target_track_idx = Some(i);
            break;
        }
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
                // target_track_idx がある場合、そのトラックの最初のMidiClipに追加を試みる。なければactive_sequence。
                if let Some(idx) = target_track_idx {
                    let track = &mut app.state.tracks[idx];
                    if track.midi_clips.is_empty() {
                        track.midi_clips.push(crate::state::clip::MidiClip::new(0, "Midi Clip", 0.0, VISIBLE_BEATS as f64));
                    }
                    let clip = &mut track.midi_clips[0];
                    let exists = clip.sequence.notes.iter().any(|n| n.pitch == pitch && n.start_beat == beat);
                    if !exists {
                        clip.sequence.add_note(pitch, DEFAULT_VELOCITY, beat, DEFAULT_DURATION);
                    }
                } else {
                    let exists = app.state.active_sequence.notes.iter().any(|n| n.pitch == pitch && n.start_beat == beat);
                    if !exists {
                        app.state.active_sequence.add_note(pitch, DEFAULT_VELOCITY, beat, DEFAULT_DURATION);
                    }
                }
            }

            // 右クリックでノート削除
            if response.secondary_clicked() {
                if let Some(idx) = target_track_idx {
                    let track = &mut app.state.tracks[idx];
                    if let Some(clip) = track.midi_clips.get_mut(0) {
                        let note_to_remove = clip.sequence.notes.iter().find(|n| {
                            n.pitch == pitch && n.start_beat <= beat && n.start_beat + n.duration_beats > beat
                        }).map(|n| n.id);
                        if let Some(id) = note_to_remove {
                            clip.sequence.remove_note(id);
                        }
                    }
                } else {
                    let note_to_remove = app.state.active_sequence.notes.iter().find(|n| {
                        n.pitch == pitch && n.start_beat <= beat && n.start_beat + n.duration_beats > beat
                    }).map(|n| n.id);

                    if let Some(id) = note_to_remove {
                        app.state.active_sequence.remove_note(id);
                    }
                }
            }
        }
    }

    // 既存ノートの描画
    let mut notes_to_draw = Vec::new();

    if let Some(idx) = target_track_idx {
        let track = &app.state.tracks[idx];
        if let Some(clip) = track.midi_clips.first() {
            notes_to_draw.extend(clip.sequence.notes.iter().cloned());
        }
    } else {
        notes_to_draw.extend(app.state.active_sequence.notes.iter().cloned());
    }

    for note in notes_to_draw {
        if (MIN_PITCH..=MAX_PITCH).contains(&note.pitch) {
            let pitch_index = MAX_PITCH.saturating_sub(note.pitch) as f32;
            let y = rect.top() + pitch_index * PITCH_HEIGHT;
            let x = rect.left() + (note.start_beat as f32) * BEAT_WIDTH;
            let w = (note.duration_beats as f32) * BEAT_WIDTH;

            let note_rect =
                egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(w, PITCH_HEIGHT));

            // ノートを描画（少し縮小して見やすくする）
            let display_rect = note_rect.shrink(NOTE_SHRINK);
            painter.rect_filled(
                display_rect,
                NOTE_CORNER_RADIUS,
                egui::Color32::from_rgb(100, 150, 250), // 青系の色
            );
            painter.rect_stroke(
                display_rect,
                NOTE_CORNER_RADIUS,
                egui::Stroke::new(NOTE_STROKE_WIDTH, egui::Color32::from_rgb(50, 100, 200)),
                egui::StrokeKind::Middle,
            );
        }
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
