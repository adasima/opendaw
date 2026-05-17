use eframe::egui;

pub struct AuraDawApp {
    pub is_playing: bool,
    pub is_looping: bool,
    pub playhead_pos: f32,
    pub master_volume: f32,
    pub is_muted: bool,
}

impl Default for AuraDawApp {
    fn default() -> Self {
        Self {
            is_playing: false,
            is_looping: true,
            playhead_pos: 0.0,
            master_volume: 0.8,
            is_muted: false,
        }
    }
}

impl AuraDawApp {
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // カスタムフォントやスタイルなどをここで設定
        setup_custom_style(&cc.egui_ctx);
        Self::default()
    }

    pub fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn stop_playback(&mut self) {
        self.is_playing = false;
        self.playhead_pos = 0.0;
    }

    pub fn toggle_loop(&mut self) {
        self.is_looping = !self.is_looping;
    }

    pub fn tick_playback(&mut self) {
        if self.is_playing {
            self.playhead_pos += 1.0;
            // 画面端まで行ったらループさせるか停止する処理
            if self.playhead_pos > 100.0 {
                self.playhead_pos = 0.0;
                if !self.is_looping {
                    self.is_playing = false;
                }
            }
        }
    }
}

fn setup_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();

    // Discordにインスパイアされたダークテーマ/グラスモーフィズム風のスタイル調整
    style.visuals = egui::Visuals::dark();
    style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(18, 19, 24, 230); // 半透明の暗い背景
    style.visuals.panel_fill = egui::Color32::from_rgb(18, 19, 24);

    ctx.set_global_style(style);
}

impl eframe::App for AuraDawApp {
    // Eframe 0.34
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 再生中の場合、プレイヘッドを進行させて再描画を要求
        if self.is_playing {
            self.tick_playback();
            ctx.request_repaint();
        }

        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
             self.ui(ui, frame);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        #[allow(deprecated)]
        egui::TopBottomPanel::bottom("mixer_panel")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.heading("Mixer & Effects");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Master Volume");
                    ui.add(egui::Slider::new(&mut self.master_volume, 0.0..=1.0));

                    let mute_icon = if self.is_muted { "🔇" } else { "🔊" };
                    if ui.button(mute_icon).on_hover_text("Mute/Unmute").clicked() {
                        self.toggle_mute();
                    }

                });
            });

        #[allow(deprecated)]
        egui::SidePanel::left("tracks_panel")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.heading("Tracks");
                ui.separator();
                ui.label("Track 1 - Vocals");
                ui.label("Track 2 - Synth");
            });

        #[allow(deprecated)]
        egui::SidePanel::right("ai_agent_panel")
            .resizable(true)
            .show_inside(ui, |ui| {
                ui.heading("AI Agent & CLI");
                ui.separator();
                ui.label("Agent is ready.");
                ui.text_edit_singleline(&mut "".to_string());
            });

        #[allow(deprecated)]
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Main Timeline & Visualizer");

            // トランスポートコントロールの追加
            ui.horizontal(|ui| {
                let play_icon = if self.is_playing { "⏸" } else { "▶" };
                if ui.button(play_icon).on_hover_text("Play/Pause").clicked() {
                    self.toggle_playback();
                }
                if ui.button("⏹").on_hover_text("Stop").clicked() {
                    self.stop_playback();
                }

                let loop_icon = if self.is_looping { "🔁 (On)" } else { "🔁 (Off)" };
                if ui.button(loop_icon).on_hover_text("Toggle Loop").clicked() {
                    self.toggle_loop();
                }
            });
            ui.separator();

            // 波形のプレースホルダー領域
            let (rect, _response) = ui.allocate_exact_size(
                ui.available_size(),
                egui::Sense::hover(),
            );

            // 簡単な波形描画のモック
            let painter = ui.painter();
            painter.rect_filled(rect, 4.0, egui::Color32::from_rgba_premultiplied(22, 24, 28, 180));

            let center_y = rect.center().y;
            for i in 0..100 {
                let x = rect.left() + (rect.width() / 100.0) * i as f32;
                let height = (i as f32 * 0.1).sin().abs() * 50.0;
                painter.line_segment(
                    [egui::pos2(x, center_y - height), egui::pos2(x, center_y + height)],
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(114, 137, 218)) // アクセントカラー
                );
            }

            // プレイヘッド（縦線）の描画
            let playhead_x = rect.left() + (rect.width() / 100.0) * self.playhead_pos;
            painter.line_segment(
                [egui::pos2(playhead_x, rect.top()), egui::pos2(playhead_x, rect.bottom())],
                egui::Stroke::new(2.0, egui::Color32::RED)
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_playback() {
        let mut app = AuraDawApp::default();
        assert!(!app.is_playing);

        app.toggle_playback();
        assert!(app.is_playing);

        app.toggle_playback();
        assert!(!app.is_playing);
    }

    #[test]
    fn test_stop_playback() {
        let mut app = AuraDawApp::default();
        app.is_playing = true;
        app.playhead_pos = 50.0;

        app.stop_playback();
        assert!(!app.is_playing);
        assert_eq!(app.playhead_pos, 0.0);
    }

    #[test]
    fn test_toggle_loop() {
        let mut app = AuraDawApp::default();
        assert!(app.is_looping); // 💡 初期値は Default::default() により true

        app.toggle_loop();
        assert!(!app.is_looping);

        app.toggle_loop();
        assert!(app.is_looping);
    }

    #[test]
    fn test_playback_end_behavior_with_loop() {
        let mut app = AuraDawApp::default();
        app.is_playing = true;
        app.is_looping = true;
        app.playhead_pos = 100.0; // 次の tick で 100.0 を超える

        app.tick_playback();

        assert!(app.is_playing);
        assert_eq!(app.playhead_pos, 0.0);
    }

    #[test]
    fn test_playback_end_behavior_without_loop() {
        let mut app = AuraDawApp::default();
        app.is_playing = true;
        app.is_looping = false;
        app.playhead_pos = 100.0; // 次の tick で 100.0 を超える

        app.tick_playback();

        assert!(!app.is_playing);
        assert_eq!(app.playhead_pos, 0.0);
    }

    #[test]
    fn test_toggle_mute() {
        let mut app = AuraDawApp::default();
        assert!(!app.is_muted);

        app.toggle_mute();
        assert!(app.is_muted);

        app.toggle_mute();
        assert!(!app.is_muted);
    }
}
