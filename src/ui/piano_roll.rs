use crate::app::AuraDawApp;
use eframe::egui;

const PIANO_ROLL_BG_ROUNDING: f32 = 4.0;

/// ピアノロールエディタ領域のスケルトンUIを描画します。
pub fn draw_piano_roll(ui: &mut egui::Ui, _app: &mut AuraDawApp) {
    ui.heading("Piano Roll View");

    // プレースホルダー領域
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 150.0),
        egui::Sense::hover()
    );

    let painter = ui.painter();
    painter.rect_filled(
        rect,
        PIANO_ROLL_BG_ROUNDING,
        egui::Color32::from_rgba_premultiplied(30, 32, 40, 200), // ダークな背景色
    );

    // 中央にテキストを配置
    let center = rect.center();
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        "Piano Roll Area (Coming Soon)",
        egui::FontId::proportional(16.0),
        egui::Color32::GRAY,
    );
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_draw_piano_roll_compiles() {
        // UI描画関数がシグネチャ通りに呼び出せるかの簡単なテスト
        // 実際の描画テストはeguiのContextが必要なため、関数が存在しコンパイルが通ることを確認
        let _f = draw_piano_roll as fn(&mut egui::Ui, &mut AuraDawApp);
    }
}
