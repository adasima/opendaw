//! オーディオエクスポートモジュール
//!
//! ミキサー出力をWAVファイルとして書き出す（オフラインレンダリング）機能を提供します。

use crate::engine::mixer::{TrackMixData, mix_tracks};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// トラックをミキシングして指定されたWAVファイルにエクスポートします。
///
/// `path` は出力先ファイルのパスです。
/// `tracks` はエクスポート対象のトラックデータの配列です。
/// `total_samples` は処理する総サンプル数（1チャンネルあたりのフレーム数）です。
/// `sample_rate` はサンプリングレートです（例: 44100）。
/// 出力は常に16ビットステレオになります。
pub fn export_project_to_wav<P: AsRef<Path>>(
    path: P,
    tracks: &mut [TrackMixData<'_>],
    total_samples: usize,
    sample_rate: u32,
) -> Result<(), String> {
    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let file = File::create(path).map_err(|e| format!("ファイルの作成に失敗しました: {}", e))?;
    let buf_writer = BufWriter::new(file);
    let mut writer = WavWriter::new(buf_writer, spec)
        .map_err(|e| format!("WavWriterの作成に失敗しました: {}", e))?;

    const CHUNK_FRAMES: usize = 1024;
    let mut processed_samples = 0;

    // 各トラックの元のサンプルスライスを保存しておく
    let original_samples: Vec<&[f32]> = tracks.iter().map(|t| t.samples).collect();

    // バッファを一度だけ確保する
    let mut mix_buf = vec![0.0; CHUNK_FRAMES * 2]; // ステレオ出力のため * 2

    while processed_samples < total_samples {
        let remaining_samples = total_samples - processed_samples;
        let current_chunk_frames = remaining_samples.min(CHUNK_FRAMES);

        let mix_slice = &mut mix_buf[..current_chunk_frames * 2];

        // 各トラックのサンプルスライスを、現在のチャンク分だけに設定する
        for (i, track) in tracks.iter_mut().enumerate() {
            let samples = original_samples[i];
            let channels = track.channels as usize;
            let start = (processed_samples * channels).min(samples.len());
            let end = ((processed_samples + current_chunk_frames) * channels).min(samples.len());
            track.samples = &samples[start..end];
        }

        // ミックス処理
        mix_tracks(mix_slice, 2, tracks);

        // WAVに書き込み (f32 -> i16 変換)
        for &sample in mix_slice.iter() {
            let s_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            writer
                .write_sample(s_i16)
                .map_err(|e| format!("サンプルの書き込みに失敗しました: {}", e))?;
        }

        processed_samples += current_chunk_frames;
    }

    writer
        .finalize()
        .map_err(|e| format!("WAVファイルの保存に失敗しました: {}", e))?;

    // トラックのサンプルスライスを元に戻す（呼び出し元に影響を与えないため）
    for (i, track) in tracks.iter_mut().enumerate() {
        track.samples = original_samples[i];
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_export_wav() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = "test_export.wav";
        let samples1 = vec![0.5, 0.5, 0.5, 0.5]; // 4 frames
        let samples2 = vec![-0.5, -0.5, -0.5, -0.5]; // 4 frames

        let track1 = TrackMixData {
            samples: &samples1,
            channels: 1,
            volume: 1.0,
            pan: -1.0, // Left
            is_muted: false,
            is_solo: false,
            effects: &mut [],
            oscillator: None,
        };

        let track2 = TrackMixData {
            samples: &samples2,
            channels: 1,
            volume: 1.0,
            pan: 1.0, // Right
            is_muted: false,
            is_solo: false,
            effects: &mut [],
            oscillator: None,
        };

        let mut tracks = [track1, track2];

        let result = export_project_to_wav(file_path, &mut tracks, 4, 44100);
        assert!(result.is_ok());

        // 出力されたファイルを確認
        assert!(Path::new(file_path).exists());
        let md = fs::metadata(file_path)?;
        assert!(md.len() > 0);

        // テスト用の一時ファイルを削除
        let _ = fs::remove_file(file_path);

        Ok(())
    }
}
