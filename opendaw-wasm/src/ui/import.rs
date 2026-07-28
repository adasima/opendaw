use crate::app::OpenDawApp;
use eframe::egui;
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
use ringbuf::traits::Producer;

/// オーディオインポート用のUI（ボタン等）を描画します。
#[allow(unused_variables)]
pub fn draw_import_ui(ui: &mut egui::Ui, app: &mut OpenDawApp) {
    if ui
        .button("📁 Import Audio")
        .on_hover_text("WAVファイルなどをインポートします")
        .clicked()
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let picked_file = FileDialog::new().add_filter("Audio", &["wav"]).pick_file();

            if let Some(path) = picked_file {
                // パスからファイル名を取得
                let file_name = extract_file_name(&path);

                app.state.add_track(file_name.clone());

                let track_idx = app.state.tracks.len() - 1;
                let track_id = app.state.tracks[track_idx].id;

                match crate::engine::audio_file::load_wav(&path) {
                    Ok(buffer) => {
                        let length = buffer.samples.len() as f32
                            / buffer.channels as f32
                            / buffer.sample_rate as f32;
                        let summary: Vec<f32> =
                            buffer.samples.iter().step_by(100).copied().collect();

                        let mut clip =
                            crate::state::clip::AudioClip::new(0, file_name, 0.0, length);
                        clip.set_waveform_summary(summary);

                        app.state.tracks[track_idx].clips.push(clip);

                        if let Some(ui_channels) = &mut app.ui_channels {
                            let _ = ui_channels.0.try_push(
                                crate::engine::channel::UiToAudioMsg::AddRecordedClip(
                                    track_id,
                                    0,
                                    std::sync::Arc::new(buffer.samples),
                                ),
                            );
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to load audio file {:?}: {}", path, e);
                    }
                }
            }
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
