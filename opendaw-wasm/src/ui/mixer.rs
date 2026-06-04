use eframe::egui;

/// Mixer & Effects パネルを描画します。
pub fn draw_mixer_panel(ui: &mut egui::Ui, app: &mut crate::app::OpenDawApp) {
    #[allow(deprecated)]
    egui::TopBottomPanel::bottom("mixer_panel")
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.heading("Mixer & Effects");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Master Volume");
                ui.add(egui::Slider::new(&mut app.state.master_volume, 0.0..=1.0));

                let mute_icon = if app.state.is_muted { "🔇" } else { "🔊" };
                if ui.button(mute_icon).on_hover_text("Mute/Unmute").clicked() {
                    app.state.toggle_mute();
                }
            });

            ui.separator();

            let mut opened_effect_id = None;

            // 各トラックのミキサーコントロールを水平スクロール領域に表示
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for track in &mut app.state.tracks {
                        ui.group(|ui| {
                            const PANEL_WIDTH: f32 = 200.0;
                            ui.set_width(PANEL_WIDTH);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(&track.name).strong());
                                ui.separator();

                                ui.label("Volume");
                                ui.add(egui::Slider::new(&mut track.volume, 0.0..=2.0).text("Vol"));

                                ui.label("Pan");
                                ui.add(egui::Slider::new(&mut track.pan, -1.0..=1.0).text("L/R"));

                                ui.horizontal(|ui| {
                                    let mut mute_text = egui::RichText::new("M");
                                    if track.is_muted {
                                        mute_text = mute_text.color(egui::Color32::RED);
                                    }
                                    if ui.button(mute_text).on_hover_text("Mute").clicked() {
                                        track.toggle_mute();
                                    }

                                    let mut solo_text = egui::RichText::new("S");
                                    if track.is_solo {
                                        solo_text = solo_text.color(egui::Color32::YELLOW);
                                    }
                                    if ui.button(solo_text).on_hover_text("Solo").clicked() {
                                        track.toggle_solo();
                                    }

                                    if ui.button("FX").on_hover_text("Effects").clicked() {
                                        opened_effect_id = Some(track.id);
                                    }
                                });

                                ui.separator();
                                ui.checkbox(&mut track.synth.is_enabled, "Synth");
                                if track.synth.is_enabled {
                                    ui.label("Freq (Hz)");
                                    ui.add(
                                        egui::Slider::new(&mut track.synth.frequency, 20.0..=20000.0).logarithmic(true)
                                    );

                                    ui.label("Waveform");
                                    egui::ComboBox::from_id_source(format!("waveform_{}", track.id))
                                        .selected_text(match track.synth.waveform {
                                            crate::state::track_plugin::Waveform::Sine => "Sine",
                                            crate::state::track_plugin::Waveform::Square => "Square",
                                            crate::state::track_plugin::Waveform::Sawtooth => "Sawtooth",
                                        })
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut track.synth.waveform, crate::state::track_plugin::Waveform::Sine, "Sine");
                                            ui.selectable_value(&mut track.synth.waveform, crate::state::track_plugin::Waveform::Square, "Square");
                                            ui.selectable_value(&mut track.synth.waveform, crate::state::track_plugin::Waveform::Sawtooth, "Sawtooth");
                                        });

                                    ui.label("ADSR");
                                    const MAX_ATTACK: f32 = 2.0;
                                    const MAX_DECAY: f32 = 2.0;
                                    const MAX_SUSTAIN: f32 = 1.0;
                                    const MAX_RELEASE: f32 = 5.0;
                                    ui.horizontal(|ui| {
                                        ui.add(egui::Slider::new(&mut track.synth.adsr.attack, 0.0..=MAX_ATTACK).text("A"));
                                        ui.add(egui::Slider::new(&mut track.synth.adsr.decay, 0.0..=MAX_DECAY).text("D"));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add(egui::Slider::new(&mut track.synth.adsr.sustain, 0.0..=MAX_SUSTAIN).text("S"));
                                        ui.add(egui::Slider::new(&mut track.synth.adsr.release, 0.0..=MAX_RELEASE).text("R"));
                                    });
                                }
                            });
                        });
                    }
                });
            });

            if let Some(id) = opened_effect_id {
                app.opened_effect_track_id = Some(id);
            }
        });
}

#[cfg(test)]
mod tests {
    use crate::app::OpenDawApp;

    #[test]
    fn test_mixer_synth_toggle() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = OpenDawApp::default();
        app.state.add_track("Test Track");

        let track = app.state.tracks.last_mut().ok_or("Track not found")?;
        assert!(!track.synth.is_enabled);

        // シンセサイザーを有効にする
        track.toggle_synth();
        assert!(track.synth.is_enabled);
        track.set_synth_frequency(440.0);
        assert_eq!(track.synth.frequency, 440.0);
        Ok(())
    }
}
