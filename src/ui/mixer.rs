use eframe::egui;

/// Mixer & Effects パネルを描画します。
pub fn draw_mixer_panel(ui: &mut egui::Ui, app: &mut crate::app::AuraDawApp) {
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

            // 各トラックのミキサーコントロールを水平スクロール領域に表示
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for track in &mut app.state.tracks {
                        ui.group(|ui| {
                            ui.set_width(120.0);
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
                                });
                            });
                        });
                    }
                });
            });
        });
}
