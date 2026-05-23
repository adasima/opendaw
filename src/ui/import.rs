use crate::app::OpenDawApp;
use eframe::egui;
use rfd::FileDialog;

/// オーディオインポート用のUI（ボタン等）を描画します。
pub fn draw_import_ui(ui: &mut egui::Ui, app: &mut OpenDawApp) {
    if ui
        .button("📁 Import Audio")
        .on_hover_text("WAVファイルなどをインポートします")
        .clicked()
    {
        let picked_file = FileDialog::new().add_filter("Audio", &["wav"]).pick_file();

        if let Some(path) = picked_file {
            // パスからファイル名を取得
            let file_name = extract_file_name(&path);

            // TODO: ファイルの読み込み処理（オーディオエンジンへの送信）は Phase 3/4 にて拡張
            // 今回はトラックの追加のみを実施
            app.state.add_track(file_name);
        }
    }
}

/// 指定されたパスからファイル名（拡張子なし）を抽出します。
pub fn extract_file_name(path: &std::path::Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_extract_file_name() {
        let path = Path::new("/path/to/audio_file.wav");
        assert_eq!(extract_file_name(path), "audio_file");

        let path_no_ext = Path::new("/path/to/audio_file");
        assert_eq!(extract_file_name(path_no_ext), "audio_file");

        let path_empty = Path::new("");
        assert_eq!(extract_file_name(path_empty), "");
    }
}
