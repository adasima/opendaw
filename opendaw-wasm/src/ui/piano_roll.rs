use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2, PointerButton};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Note {
    pub id: usize,
    pub pitch: u8,
    pub start_tick: u32,
    pub duration: u32,
}

pub struct PianoRoll {
    pub notes: Vec<Note>,
    next_id: usize,
    pan: Vec2,
    pixels_per_tick: f32,
    key_height: f32,
    dragging_note: Option<(usize, Vec2)>, // note_id, click offset
    ticks_per_beat: u32,
}

impl Default for PianoRoll {
    fn default() -> Self {
        Self {
            notes: vec![
                Note { id: 0, pitch: 60, start_tick: 0, duration: 480 },
                Note { id: 1, pitch: 64, start_tick: 480, duration: 480 },
                Note { id: 2, pitch: 67, start_tick: 960, duration: 480 },
            ],
            next_id: 3,
            pan: Vec2::new(0.0, 60.0 * 20.0), // Center around C4 (pitch 60)
            pixels_per_tick: 0.1,
            key_height: 20.0,
            dragging_note: None,
            ticks_per_beat: 480,
        }
    }
}

impl PianoRoll {
    pub fn show(&mut self, ui: &mut egui::Ui) -> egui::Response {
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
                        self.pixels_per_tick = (self.pixels_per_tick * (1.0 + zoom_delta)).clamp(0.01, 10.0);
                    }
                }
            }
            
            // Release drag
            if !i.pointer.button_down(PointerButton::Primary) {
                self.dragging_note = None;
            }
        });
        
        // Clamp Pan
        let max_pan_y = 128.0 * self.key_height - rect.height();
        self.pan.y = self.pan.y.clamp(0.0, max_pan_y.max(0.0));
        self.pan.x = self.pan.x.max(0.0);

        let keyboard_rect = Rect::from_min_max(rect.min, Pos2::new(rect.min.x + keys_width, rect.max.y));
        let grid_rect = Rect::from_min_max(Pos2::new(rect.min.x + keys_width, rect.min.y), rect.max);
        
        let mut pointer_pos = None;
        ui.input(|i| pointer_pos = i.pointer.hover_pos());
        
        // Interaction Logic
        if let Some(pos) = pointer_pos {
            if grid_rect.contains(pos) {
                let grid_pos = pos - grid_rect.min + self.pan;
                let hover_tick = (grid_pos.x / self.pixels_per_tick).max(0.0) as u32;
                let hover_pitch = (127.0 - grid_pos.y / self.key_height).clamp(0.0, 127.0) as u8;

                if response.drag_started_by(PointerButton::Primary) {
                    // Try to pick a note (reverse order so top note is picked)
                    let mut clicked_note = None;
                    for note in self.notes.iter().rev() {
                        let note_rect = self.note_rect(note, grid_rect.min);
                        if note_rect.contains(pos) {
                            clicked_note = Some(note.id);
                            let offset = pos - note_rect.min;
                            self.dragging_note = Some((note.id, offset));
                            break;
                        }
                    }
                    
                    if clicked_note.is_none() {
                        // Add a new note
                        let snap_tick = (hover_tick / (self.ticks_per_beat / 4)) * (self.ticks_per_beat / 4);
                        let new_note = Note {
                            id: self.next_id,
                            pitch: hover_pitch,
                            start_tick: snap_tick,
                            duration: self.ticks_per_beat / 4, // 16th note default duration
                        };
                        self.next_id += 1;
                        self.notes.push(new_note);
                        
                        let note_rect = self.note_rect(&new_note, grid_rect.min);
                        self.dragging_note = Some((new_note.id, pos - note_rect.min));
                    }
                } else if response.clicked_by(PointerButton::Secondary) {
                    // Delete note on right click
                    
                    let mut to_remove = Vec::new();
                    for n in &self.notes {
                        if self.note_rect(n, grid_rect.min).contains(pos) {
                            to_remove.push(n.id);
                        }
                    }
                    self.notes.retain(|n| !to_remove.contains(&n.id));

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
                    new_tick = (new_tick / (self.ticks_per_beat / 4)) * (self.ticks_per_beat / 4); // snap to 16th
                    
                    let new_pitch = (127.0 - grid_pos.y / self.key_height).clamp(0.0, 127.0) as u8;
                    
                    if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
                        note.start_tick = new_tick;
                        note.pitch = new_pitch;
                    }
                }
            }
        }

        // ================= Drawing Phase =================

        // 1. Draw Keyboard Background & Keys
        let kb_painter = ui.painter().with_clip_rect(keyboard_rect);
        kb_painter.rect_filled(keyboard_rect, 0.0, Color32::from_gray(30));
        
        for p in 0..=127 {
            let y = keyboard_rect.min.y + (127 - p) as f32 * self.key_height - self.pan.y;
            if y + self.key_height < keyboard_rect.min.y || y > keyboard_rect.max.y {
                continue; // Skip off-screen
            }
            
            let is_black = matches!(p % 12, 1 | 3 | 6 | 8 | 10);
            let color = if is_black { Color32::from_gray(20) } else { Color32::from_gray(200) };
            let key_rect = Rect::from_min_size(Pos2::new(keyboard_rect.min.x, y), Vec2::new(keys_width, self.key_height));
            
            kb_painter.rect_filled(key_rect, 0.0, color);
            kb_painter.rect_stroke(key_rect, 0.0, Stroke::new(1.0, Color32::from_gray(50)), egui::StrokeKind::Inside);
            
            if p % 12 == 0 {
                kb_painter.text(
                    key_rect.min + Vec2::new(5.0, 2.0),
                    egui::Align2::LEFT_TOP,
                    format!("C{}", (p as i32 / 12) - 1),
                    egui::FontId::proportional(12.0),
                    if is_black { Color32::WHITE } else { Color32::BLACK }
                );
            }
        }

        // 2. Draw Grid Background & Lines
        let grid_painter = ui.painter().with_clip_rect(grid_rect);
        grid_painter.rect_filled(grid_rect, 0.0, Color32::from_gray(40));
        
        // Horizontal key lanes
        for p in 0..=127 {
            let y = grid_rect.min.y + (127 - p) as f32 * self.key_height - self.pan.y;
            if y + self.key_height < grid_rect.min.y || y > grid_rect.max.y {
                continue;
            }
            let is_black = matches!(p % 12, 1 | 3 | 6 | 8 | 10);
            if is_black {
                grid_painter.rect_filled(
                    Rect::from_min_size(Pos2::new(grid_rect.min.x, y), Vec2::new(grid_rect.width(), self.key_height)),
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
        let min_tick = (self.pan.x / self.pixels_per_tick).max(0.0) as u32;
        let max_tick = min_tick + (grid_rect.width() / self.pixels_per_tick) as u32;
        
        let snap_step = self.ticks_per_beat / 4;
        let mut t = (min_tick / snap_step) * snap_step;
        while t <= max_tick {
            let x = grid_rect.min.x + t as f32 * self.pixels_per_tick - self.pan.x;
            let is_beat = t % self.ticks_per_beat == 0;
            let is_bar = t % (self.ticks_per_beat * 4) == 0;
            
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

        // 3. Draw Notes
        for note in &self.notes {
            let note_rect = self.note_rect(note, grid_rect.min);
            if note_rect.max.x < grid_rect.min.x || note_rect.min.x > grid_rect.max.x ||
               note_rect.max.y < grid_rect.min.y || note_rect.min.y > grid_rect.max.y {
                continue; // Skip off-screen
            }
            
            let is_dragged = self.dragging_note.map(|(id, _)| id == note.id).unwrap_or(false);
            let fill_color = if is_dragged {
                Color32::from_rgb(150, 220, 255) // Brighter when dragged
            } else {
                Color32::from_rgb(80, 180, 250)
            };
            
            let display_rect = note_rect.shrink(1.0);
            grid_painter.rect_filled(display_rect, 2.0, fill_color);
            grid_painter.rect_stroke(display_rect, 2.0, Stroke::new(1.0, Color32::from_rgb(30, 100, 180)), egui::StrokeKind::Inside);
        }
        
        response
    }
    
    // Helper to calculate screen coordinates for a note
    fn note_rect(&self, note: &Note, grid_min: Pos2) -> Rect {
        let x = grid_min.x + note.start_tick as f32 * self.pixels_per_tick - self.pan.x;
        let y = grid_min.y + (127 - note.pitch) as f32 * self.key_height - self.pan.y;
        let w = note.duration as f32 * self.pixels_per_tick;
        let h = self.key_height;
        Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h))
    }
}
