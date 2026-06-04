use crate::app::OpenDawApp;
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
const AUTOMATION_LANE_HEIGHT: f32 = 60.0;

/// メインタイムライン領域（波形描画、プレイヘッドなど）を描画します。
pub fn draw_timeline(ui: &mut egui::Ui, app: &mut OpenDawApp) {
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

    let mut all_modified_clips = Vec::new();
    let mut all_modified_midi_clips = Vec::new();
    let mut all_modified_auto_points = Vec::new();
    let mut is_dragging_any = false;
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        BG_ROUNDING,
        egui::Color32::from_rgba_premultiplied(18, 19, 24, 255),
    );

    let mut current_y = rect.top();

    for (i, track) in app.state.tracks.iter().enumerate() {
        let track_top = current_y;
        let track_rect = egui::Rect::from_min_max(
            egui::pos2(rect.left(), track_top),
            egui::pos2(rect.right(), track_top + TRACK_HEIGHT),
        );
        current_y += TRACK_HEIGHT;

        let mut bg_color = if i % 2 == 0 {
            egui::Color32::from_rgba_premultiplied(30, 32, 38, 180)
        } else {
            egui::Color32::from_rgba_premultiplied(22, 24, 28, 180)
        };

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(hover_y) = crate::get_drag_hover_y() {
                // If hover_y falls within this track's vertical bounds, draw a highlight
                if hover_y >= track_top && hover_y < track_top + TRACK_HEIGHT {
                    bg_color = egui::Color32::from_rgba_premultiplied(70, 80, 100, 220); // Highlight color
                }
            }
        }
        painter.rect_filled(track_rect, 0.0, bg_color);

        for clip in &track.clips {
            let clip_x = rect.left() + (rect.width() / TIMELINE_PERCENT_MAX) * clip.start_pos;
            let clip_w = (rect.width() / TIMELINE_PERCENT_MAX) * clip.length;
            let clip_rect = egui::Rect::from_min_size(
                egui::pos2(clip_x, track_rect.top() + CLIP_MARGIN_Y),
                egui::vec2(clip_w, TRACK_HEIGHT - (CLIP_MARGIN_Y * 2.0)),
            );

            let clip_id = ui.make_persistent_id(format!("clip_{}_{}", track.id, clip.id));
            let clip_response = ui.interact(clip_rect, clip_id, egui::Sense::drag());

            if clip_response.dragged() {
                is_dragging_any = true;
                let drag_delta_x = clip_response.drag_delta().x;
                let delta_percent = (drag_delta_x / rect.width()) * TIMELINE_PERCENT_MAX;
                let mut new_pos = clip.start_pos + delta_percent;

                if app.state.is_grid_enabled {
                    let snap_step = 100.0 / (app.state.grid_resolution as f32 * 4.0);
                    new_pos = (new_pos / snap_step).round() * snap_step;
                }
                all_modified_clips.push((track.id, clip.id, new_pos));
            }

            if clip_response.drag_stopped() {
                #[cfg(target_arch = "wasm32")]
                crate::notify_clip_moved(track.id, clip.id, clip.start_pos);
            }

            let bg_color = if clip_response.hovered() || clip_response.dragged() {
                egui::Color32::from_rgba_premultiplied(70, 80, 120, 200)
            } else {
                egui::Color32::from_rgba_premultiplied(50, 60, 90, 200)
            };

            painter.rect_filled(clip_rect, CLIP_BG_ROUNDING, bg_color);

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
                    let val_f32: f32 = val;
                    let h = val_f32.clamp(0.0, 1.0) * max_h;
                    painter.line_segment(
                        [egui::pos2(wx, center_y - h), egui::pos2(wx, center_y + h)],
                        egui::Stroke::new(
                            WAVEFORM_STROKE_WIDTH,
                            egui::Color32::from_rgb(114, 137, 218),
                        ),
                    );
                }
            }
        }

        for clip in &track.midi_clips {
            let clip_x =
                rect.left() + (rect.width() / TIMELINE_PERCENT_MAX) * clip.start_beat as f32;
            let clip_w = (rect.width() / TIMELINE_PERCENT_MAX) * clip.length_beats as f32;
            let clip_rect = egui::Rect::from_min_size(
                egui::pos2(clip_x, track_rect.top() + CLIP_MARGIN_Y),
                egui::vec2(clip_w, TRACK_HEIGHT - (CLIP_MARGIN_Y * 2.0)),
            );

            let clip_id = ui.make_persistent_id(format!("midi_clip_{}_{}", track.id, clip.id));
            let clip_response = ui.interact(clip_rect, clip_id, egui::Sense::click_and_drag());

            if clip_response.clicked() {
                app.selected_track_id = Some(track.id);
                app.selected_clip_id = Some(clip.id);
                app.state.active_sequence = clip.sequence.clone();
            }

            if clip_response.dragged() {
                is_dragging_any = true;
                let drag_delta_x = clip_response.drag_delta().x;
                let delta_percent = (drag_delta_x / rect.width()) * TIMELINE_PERCENT_MAX;
                let mut new_pos = clip.start_beat + delta_percent as f64;

                if app.state.is_grid_enabled {
                    let snap_step = 100.0 / (app.state.grid_resolution as f64 * 4.0);
                    new_pos = (new_pos / snap_step).round() * snap_step;
                }
                all_modified_midi_clips.push((track.id, clip.id, new_pos));
            }

            if clip_response.drag_stopped() {
                #[cfg(target_arch = "wasm32")]
                crate::notify_midi_clip_moved(track.id, clip.id, clip.start_beat);
            }

            let bg_color = if clip_response.hovered() || clip_response.dragged() {
                egui::Color32::from_rgba_premultiplied(120, 80, 150, 200)
            } else {
                egui::Color32::from_rgba_premultiplied(90, 60, 110, 200)
            };

            painter.rect_filled(clip_rect, CLIP_BG_ROUNDING, bg_color);

            painter.text(
                clip_rect.left_top() + egui::vec2(CLIP_PADDING, CLIP_PADDING),
                egui::Align2::LEFT_TOP,
                &clip.name,
                egui::FontId::proportional(TEXT_SIZE),
                egui::Color32::WHITE,
            );
        }

        // Draw Automation Lane if visible
        if track.automation_visible
            && let Some(param_name) = &track.selected_automation {
                let auto_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.left(), current_y),
                    egui::pos2(rect.right(), current_y + AUTOMATION_LANE_HEIGHT),
                );
                current_y += AUTOMATION_LANE_HEIGHT;

                let bg_color = egui::Color32::from_rgba_premultiplied(20, 25, 35, 180);
                painter.rect_filled(auto_rect, 0.0, bg_color);

                // Draw lane separator
                painter.line_segment(
                    [egui::pos2(auto_rect.left(), auto_rect.top()), egui::pos2(auto_rect.right(), auto_rect.top())],
                    egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(100, 100, 100, 100)),
                );

                if let Some(auto_track) = track.automations.iter().find(|a| a.parameter_name == *param_name) {
                    let mut prev_point: Option<egui::Pos2> = None;

                    for point in &auto_track.points {
                        let px = rect.left() + (rect.width() / TIMELINE_PERCENT_MAX) * point.time as f32;
                        // value is 0.0 to 1.0, map to y (bottom to top)
                        let py = auto_rect.bottom() - (auto_rect.height() * point.value);
                        let p_pos = egui::pos2(px, py);

                        if let Some(prev) = prev_point {
                            painter.line_segment(
                                [prev, p_pos],
                                egui::Stroke::new(2.0, egui::Color32::from_rgb(114, 137, 218)),
                            );
                        }
                        prev_point = Some(p_pos);

                        let point_rect = egui::Rect::from_center_size(p_pos, egui::vec2(8.0, 8.0));
                        let point_id = ui.make_persistent_id(format!("auto_{}_{}_{}", track.id, param_name, point.id));
                        let point_response = ui.interact(point_rect, point_id, egui::Sense::drag());

                        if point_response.dragged() {
                            is_dragging_any = true;
                            let drag_delta = point_response.drag_delta();
                            let delta_time = (drag_delta.x / rect.width()) * TIMELINE_PERCENT_MAX;
                            let delta_val = -(drag_delta.y / auto_rect.height());

                            let new_time = (point.time as f32 + delta_time).clamp(0.0, TIMELINE_PERCENT_MAX);
                            let new_val = (point.value + delta_val).clamp(0.0, 1.0);

                            all_modified_auto_points.push((track.id, param_name.clone(), point.id, new_time as f64, new_val));
                        }

                        if point_response.drag_stopped() {
                            #[cfg(target_arch = "wasm32")]
                            crate::notify_update_automation_point(track.id, param_name.clone(), point.time, point.value);
                        }

                        let p_color = if point_response.hovered() || point_response.dragged() {
                            egui::Color32::WHITE
                        } else {
                            egui::Color32::from_rgb(114, 137, 218)
                        };
                        painter.circle_filled(p_pos, 4.0, p_color);
                    }
                }

                // Add new point on click in empty space
                let lane_id = ui.make_persistent_id(format!("auto_lane_{}_{}", track.id, param_name));
                let lane_response = ui.interact(auto_rect, lane_id, egui::Sense::click());
                if lane_response.clicked()
                    && let Some(pos) = lane_response.interact_pointer_pos() {
                        let click_time = ((pos.x - rect.left()) / rect.width()) * TIMELINE_PERCENT_MAX;
                        let click_val = 1.0 - ((pos.y - auto_rect.top()) / auto_rect.height());

                        #[cfg(target_arch = "wasm32")]
                        crate::notify_update_automation_point(track.id, param_name.clone(), click_time as f64, click_val);
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = click_time;
                            let _ = click_val;
                        }
                    }
            }
    }

    for (t_id, param_name, point_id, new_time, new_val) in all_modified_auto_points {
        if let Some(track) = app.state.tracks.iter_mut().find(|t| t.id == t_id)
            && let Some(auto_track) = track.automations.iter_mut().find(|a| a.parameter_name == param_name) {
                if let Some(point) = auto_track.points.iter_mut().find(|p| p.id == point_id) {
                    point.time = new_time;
                    point.value = new_val;
                }
                auto_track.points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
            }
    }

    for (t_id, clip_id, new_pos) in all_modified_clips {
        #[allow(clippy::collapsible_if)]
        if let Some(track) = app.state.tracks.iter_mut().find(|t| t.id == t_id) {
            if let Some(clip) = track.clips.iter_mut().find(|c| c.id == clip_id) {
                clip.start_pos = new_pos.max(0.0);
            }
        }
    }

    for (t_id, clip_id, new_pos) in all_modified_midi_clips {
        #[allow(clippy::collapsible_if)]
        if let Some(track) = app.state.tracks.iter_mut().find(|t| t.id == t_id) {
            if let Some(clip) = track.midi_clips.iter_mut().find(|c| c.id == clip_id) {
                clip.start_beat = new_pos.max(0.0);
            }
        }
    }

    // プレイヘッド（縦線）の描画
    app.is_dragging_clip = is_dragging_any;

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
