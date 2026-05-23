use eframe::egui;

#[allow(deprecated)]
pub fn render(ctx: &egui::Context) {
    // カスタムタイトルバー (Top)
    egui::TopBottomPanel::top("title_bar")
        .exact_size(32.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("OpenDAW Genesis");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("X").clicked() {
                        // 閉じるアクション用
                    }
                    if ui.button("□").clicked() {
                        // 最大化アクション用
                    }
                    if ui.button("-").clicked() {
                        // 最小化アクション用
                    }
                });
            });
        });

    // 左パネル (トラック一覧)
    egui::SidePanel::left("track_panel")
        .default_size(220.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Tracks");
            ui.separator();
            ui.label("Track 1");
            ui.label("Track 2");
        });

    // 右パネル (AIエージェント)
    egui::SidePanel::right("ai_panel")
        .default_size(280.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("AI Agent");
            ui.separator();
            ui.label("Hello! How can I help you compose today?");
        });

    // 中央下部 (ミキサー / ピアノロール)
    egui::TopBottomPanel::bottom("mixer_panel")
        .default_size(250.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Mixer / Piano Roll");
            ui.separator();
            ui.label("Mixer and piano roll controls will be displayed here.");
        });

    // 中央上部 (タイムライン) - egui::CentralPanelは残りのすべてのスペースを埋めるため最後に定義
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Timeline");
        ui.separator();
        ui.label("Timeline arrangement area.");
    });
}
