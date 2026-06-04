use eframe::egui;

/// プラグインブラウザパネルを描画します。
/// ダミーデータを用いてプラグイン一覧のスケルトンを表示します。
pub fn draw_browser_panel(ui: &mut egui::Ui) {
    ui.heading("Plugin Browser");
    ui.separator();

    let dummy_plugins = vec![
        "Serum (Dummy)",
        "Vital (Dummy)",
        "FabFilter Pro-Q 3 (Dummy)",
        "Valhalla VintageVerb (Dummy)",
    ];

    egui::ScrollArea::vertical().show(ui, |ui| {
        for plugin in dummy_plugins {
            ui.horizontal(|ui| {
                ui.label(plugin);
                if ui.button("Load").clicked() {
                    // ダミーのロード処理
                }
            });
            ui.separator();
        }
    });
}

#[cfg(test)]
mod tests {
    // UIロジックのため、ダミーテストのみ
    #[test]
    fn test_dummy() {
        assert!(true);
    }
}
