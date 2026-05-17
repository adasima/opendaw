use eframe::egui;

#[derive(Default)]
pub struct AuraDawApp {
    // 状態などをここに保持します
}

impl AuraDawApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // カスタムフォントやスタイルなどをここで設定
        setup_custom_style(&cc.egui_ctx);
        Self::default()
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
                    let mut vol = 0.8;
                    ui.add(egui::Slider::new(&mut vol, 0.0..=1.0));
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
        });
    }
}
