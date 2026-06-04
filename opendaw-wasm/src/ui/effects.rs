use crate::app::OpenDawApp;
use crate::state::track::{EffectSetting, EffectType};
use eframe::egui;

/// エフェクトチェーンの編集ウィンドウを描画します。
pub fn draw_effects_window(ctx: &egui::Context, app: &mut OpenDawApp) {
    let track_id = match app.opened_effect_track_id {
        Some(id) => id,
        None => return,
    };

    let track_index = app.state.tracks.iter().position(|t| t.id == track_id);
    let track = match track_index {
        Some(idx) => &mut app.state.tracks[idx],
        None => {
            app.opened_effect_track_id = None;
            return;
        }
    };

    let mut is_open = true;

    egui::Window::new(format!("Effects - {}", track.name))
        .open(&mut is_open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add Gain").clicked() {
                    let new_id = track.effects.iter().map(|e| e.id).max().unwrap_or(0) + 1;
                    track.add_effect(EffectSetting::new(new_id, EffectType::Gain));
                }
                if ui.button("Add Filter").clicked() {
                    let new_id = track.effects.iter().map(|e| e.id).max().unwrap_or(0) + 1;
                    track.add_effect(EffectSetting::new(new_id, EffectType::Filter));
                }
                if ui.button("Add Delay").clicked() {
                    let new_id = track.effects.iter().map(|e| e.id).max().unwrap_or(0) + 1;
                    track.add_effect(EffectSetting::new(
                        new_id,
                        EffectType::Delay {
                            time_ms: 300.0,
                            feedback: 0.3,
                            mix: 0.5,
                        },
                    ));
                }
            });

            ui.separator();

            let mut action = None;

            let effects_len = track.effects.len();
            for i in 0..effects_len {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let effect_id = track.effects[i].id;

                        ui.checkbox(&mut track.effects[i].is_enabled, "");

                        let effect_name = match track.effects[i].effect_type {
                            EffectType::Gain => "Gain",
                            EffectType::Filter => "Filter",
                            EffectType::Delay { .. } => "Delay",
                        };
                        ui.label(effect_name);

                        let effect = &mut track.effects[i];
                        match &mut effect.effect_type {
                            EffectType::Gain => {}
                            EffectType::Filter => {}
                            _ => {
                                if let crate::state::track::EffectType::Delay {
                                    time_ms,
                                    feedback,
                                    mix,
                                } = &mut effect.effect_type
                                {
                                    ui.add(
                                        egui::Slider::new(time_ms, 1.0..=2000.0).text("Time (ms)"),
                                    );
                                    ui.add(
                                        egui::Slider::new(feedback, 0.0..=0.99).text("Feedback"),
                                    );
                                    ui.add(egui::Slider::new(mix, 0.0..=1.0).text("Mix"));
                                }
                            }
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("X").on_hover_text("Remove").clicked() {
                                action = Some(("remove", effect_id));
                            }
                            if i < effects_len - 1
                                && ui.button("↓").on_hover_text("Move Down").clicked()
                            {
                                action = Some(("down", i));
                            }
                            if i > 0 && ui.button("↑").on_hover_text("Move Up").clicked() {
                                action = Some(("up", i));
                            }
                        });
                    });
                });
            }

            if let Some((act, val)) = action {
                match act {
                    "remove" => track.remove_effect(val),
                    "up" => track.move_effect(val, val - 1),
                    "down" => track.move_effect(val, val + 1),
                    _ => {}
                }
            }
        });

    if !is_open {
        app.opened_effect_track_id = None;
    }
}
