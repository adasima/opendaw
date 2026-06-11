use egui::{PointerButton, Pos2, Rect, Sense, Vec2};

pub mod grid;
pub mod keyboard;
pub mod note;

pub struct PianoRoll {
    // notes are now in app.state.active_sequence
    pub pitch_bend_curve: Vec<(u32, f32)>, // (tick, bend_value -1.0 to 1.0)

    pub pan: Vec2,
    pub pixels_per_tick: f32,
    pub key_height: f32,
    pub dragging_note: Option<(usize, Vec2)>, // note_id, click offset
    pub resizing_note: Option<usize>,         // note_id for resizing duration
    pub ticks_per_beat: u32,
}

impl Default for PianoRoll {
    fn default() -> Self {
        Self {
            pitch_bend_curve: vec![],
            pan: Vec2::new(0.0, 60.0 * 20.0), // Center around C4 (pitch 60)
            pixels_per_tick: 0.1,
            key_height: 20.0,
            dragging_note: None,
            resizing_note: None,
            ticks_per_beat: 480,
        }
    }
}

pub fn draw_piano_roll(ui: &mut egui::Ui, app: &mut crate::app::OpenDawApp) {
    ui.group(|ui| {
        ui.heading("Piano Roll & ARA2 / SV2 Editor");
        let mut piano_roll = std::mem::take(&mut app.piano_roll);
        piano_roll.show(ui, app);
        app.piano_roll = piano_roll;
    });
}

impl PianoRoll {
    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut crate::app::OpenDawApp) -> egui::Response {
        let keys_width = 60.0;
        let response = ui.allocate_response(ui.available_size(), Sense::click_and_drag());
        let rect = response.rect;

        // Panning and Zooming logic
        ui.input(|i| {
            if response.hovered() {
                // Scroll wheel panning
                self.pan.y -= i.smooth_scroll_delta.y;
                self.pan.x -= i.smooth_scroll_delta.x;

                // Middle click panning
                if i.pointer.button_down(PointerButton::Middle) {
                    self.pan -= i.pointer.delta();
                }

                // Zooming (Ctrl + Scroll)
                if i.modifiers.ctrl {
                    let zoom_delta = i.smooth_scroll_delta.y * 0.01;
                    if zoom_delta != 0.0 {
                        self.pixels_per_tick =
                            (self.pixels_per_tick * (1.0 + zoom_delta)).clamp(0.01, 10.0);
                    }
                }
            }

            // Release drag
            if !i.pointer.button_down(PointerButton::Primary) {
                let modified = self.dragging_note.is_some() || self.resizing_note.is_some();
                self.dragging_note = None;
                self.resizing_note = None;

                if modified {
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let (Some(track_id), Some(clip_id)) =
                            (app.selected_track_id, app.selected_clip_id)
                        {
                            crate::notify_update_midi_clip_notes(
                                track_id,
                                clip_id,
                                &app.state.active_sequence.notes,
                            );
                        }
                    }
                }
            }
        });

        // Clamp Pan
        let max_pan_y = 128.0 * self.key_height - rect.height();
        self.pan.y = self.pan.y.clamp(0.0, max_pan_y.max(0.0));
        self.pan.x = self.pan.x.max(0.0);

        let keyboard_rect =
            Rect::from_min_max(rect.min, Pos2::new(rect.min.x + keys_width, rect.max.y));
        let grid_rect =
            Rect::from_min_max(Pos2::new(rect.min.x + keys_width, rect.min.y), rect.max);

        let mut pointer_pos = None;
        ui.input(|i| pointer_pos = i.pointer.hover_pos());

        // Interaction Logic
        #[allow(clippy::collapsible_if)]
        if let Some(pos) = pointer_pos {
            if grid_rect.contains(pos) {
                let grid_pos = pos - grid_rect.min + self.pan;
                let hover_tick = (grid_pos.x / self.pixels_per_tick).max(0.0) as u32;
                let hover_pitch = (127.0 - grid_pos.y / self.key_height).clamp(0.0, 127.0) as u8;

                let target_pitch_raw = 127 - (grid_pos.y / self.key_height).floor() as i32;
                let target_pitch = if (0..=127).contains(&target_pitch_raw) {
                    Some(target_pitch_raw as u8)
                } else {
                    None
                };
                // Optimization: Calculate beat once for X bounds checking.
                // We add some padding (e.g. 10.0 pixels / pixels_per_tick converted to beats)
                // so we don't miss clicks on visual borders / resize handles.
                let beat_padding = 15.0 / self.pixels_per_tick as f64 / self.ticks_per_beat as f64;
                let hover_beat = hover_tick as f64 / self.ticks_per_beat as f64;

                // Change cursor on hover
                if let Some(target_pitch) = target_pitch {
                    for note in app.state.active_sequence.notes.iter().rev() {
                        if (note.pitch as i32 - target_pitch as i32).abs() <= 1 {
                            if hover_beat >= note.start_beat - beat_padding && hover_beat <= note.start_beat + note.duration_beats + beat_padding {
                                let note_rect = self.note_rect(note, grid_rect.min);
                                if note_rect.contains(pos) {
                                    if pos.x > note_rect.max.x - 10.0 {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }

                if response.drag_started_by(PointerButton::Primary) {
                    // Try to pick a note (reverse order so top note is picked)
                    let mut clicked_note = None;
                    if let Some(target_pitch) = target_pitch {
                        for note in app.state.active_sequence.notes.iter().rev() {
                            if (note.pitch as i32 - target_pitch as i32).abs() <= 1 {
                                if hover_beat >= note.start_beat - beat_padding && hover_beat <= note.start_beat + note.duration_beats + beat_padding {
                                    let note_rect = self.note_rect(note, grid_rect.min);
                                    if note_rect.contains(pos) {
                                        clicked_note = Some(note.id);
                                        if pos.x > note_rect.max.x - 10.0 {
                                            self.resizing_note = Some(note.id);
                                        } else {
                                            let offset = pos - note_rect.min;
                                            self.dragging_note = Some((note.id, offset));
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if clicked_note.is_none() {
                        // The fallback still uses X bounds (beat) to avoid 100k allocations,
                        // but removes strict pitch bounds to allow picking tall notes overlapping from other rows.
                        for note in app.state.active_sequence.notes.iter().rev() {
                            if hover_beat >= note.start_beat - beat_padding && hover_beat <= note.start_beat + note.duration_beats + beat_padding {
                                let note_rect = self.note_rect(note, grid_rect.min);
                                if note_rect.contains(pos) {
                                    clicked_note = Some(note.id);
                                    if pos.x > note_rect.max.x - 10.0 {
                                        self.resizing_note = Some(note.id);
                                    } else {
                                        let offset = pos - note_rect.min;
                                        self.dragging_note = Some((note.id, offset));
                                    }
                                    break;
                                }
                            }
                        }
                    }

                    if clicked_note.is_none() {
                        // Add a new note
                        let mut snap_tick = hover_tick;
                        if app.state.is_grid_enabled {
                            let snap_step = self.ticks_per_beat / app.state.grid_resolution;
                            snap_tick = (hover_tick / snap_step) * snap_step;
                        }

                        let id = app.state.active_sequence.add_note(
                            hover_pitch,
                            100,
                            snap_tick as f64 / self.ticks_per_beat as f64,
                            0.25,
                        );
                        if let Some(last_note) = app.state.active_sequence.get_note(id) {
                            let note_rect = self.note_rect(last_note, grid_rect.min);
                            self.dragging_note = Some((last_note.id, pos - note_rect.min));
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            if let (Some(track_id), Some(clip_id)) =
                                (app.selected_track_id, app.selected_clip_id)
                            {
                                crate::notify_update_midi_clip_notes(
                                    track_id,
                                    clip_id,
                                    &app.state.active_sequence.notes,
                                );
                            }
                        }
                    }
                } else if response.clicked_by(PointerButton::Secondary) {
                    // Delete note on right click

                    let initial_len = app.state.active_sequence.notes.len();
                    app.state.active_sequence.notes.retain(|n| {
                        if let Some(target_pitch) = target_pitch {
                            if (n.pitch as i32 - target_pitch as i32).abs() <= 1 {
                                if hover_beat >= n.start_beat - beat_padding && hover_beat <= n.start_beat + n.duration_beats + beat_padding {
                                    !self.note_rect(n, grid_rect.min).contains(pos)
                                } else {
                                    true
                                }
                            } else {
                                true
                            }
                        } else {
                            if hover_beat >= n.start_beat - beat_padding && hover_beat <= n.start_beat + n.duration_beats + beat_padding {
                                !self.note_rect(n, grid_rect.min).contains(pos)
                            } else {
                                true
                            }
                        }
                    });
                    let removed_any = app.state.active_sequence.notes.len() < initial_len;

                    if removed_any {
                        #[cfg(target_arch = "wasm32")]
                        {
                            if let (Some(track_id), Some(clip_id)) =
                                (app.selected_track_id, app.selected_clip_id)
                            {
                                crate::notify_update_midi_clip_notes(
                                    track_id,
                                    clip_id,
                                    &app.state.active_sequence.notes,
                                );
                            }
                        }
                    }
                }
            }
        }

        // Dragging processing
        if response.dragged_by(PointerButton::Primary) {
            if let Some((id, offset)) = self.dragging_note {
                if let Some(pos) = pointer_pos {
                    let new_pos = pos - offset;
                    let grid_pos = new_pos - grid_rect.min + self.pan;

                    let mut new_tick = (grid_pos.x / self.pixels_per_tick).max(0.0) as u32;
                    if app.state.is_grid_enabled {
                        let snap_step = self.ticks_per_beat / app.state.grid_resolution;
                        new_tick = (new_tick / snap_step) * snap_step;
                    }

                    let new_pitch = (127.0 - grid_pos.y / self.key_height).clamp(0.0, 127.0) as u8;

                    if let Some(note) = app.state.active_sequence.get_note_mut(id) {
                        note.start_beat = new_tick as f64 / self.ticks_per_beat as f64;
                        note.pitch = new_pitch;
                    }
                }
            } else if let Some(id) = self.resizing_note {
                #[allow(clippy::collapsible_if)]
                if let Some(pos) = pointer_pos {
                    let grid_pos = pos - grid_rect.min + self.pan;
                    let mut new_end_tick = (grid_pos.x / self.pixels_per_tick).max(0.0) as u32;
                    if app.state.is_grid_enabled {
                        let snap_step = self.ticks_per_beat / app.state.grid_resolution;
                        new_end_tick = (new_end_tick / snap_step) * snap_step;
                    }

                    if let Some(note) = app.state.active_sequence.get_note_mut(id) {
                        if (new_end_tick as f64 / self.ticks_per_beat as f64) > note.start_beat {
                            note.duration_beats = (new_end_tick as f64
                                / self.ticks_per_beat as f64)
                                - note.start_beat;
                        } else {
                            note.duration_beats = 0.25; // min 16th note
                        }
                    }
                }
            }
        }

        // ================= Drawing Phase =================

        keyboard::draw_keyboard(ui, self, keyboard_rect, keys_width);
        grid::draw_grid(ui, app, self, grid_rect);
        note::draw_notes(ui, app, self, grid_rect);

        response
    }

    // Helper to calculate screen coordinates for a note
    pub fn note_rect(&self, note: &crate::midi::sequence::NoteEvent, grid_min: Pos2) -> Rect {
        let x = grid_min.x
            + (note.start_beat * self.ticks_per_beat as f64) as f32 * self.pixels_per_tick
            - self.pan.x;
        let y = grid_min.y + (127 - note.pitch) as f32 * self.key_height - self.pan.y;
        let w = (note.duration_beats * self.ticks_per_beat as f64) as f32 * self.pixels_per_tick;
        let h = self.key_height;
        Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h))
    }
}
