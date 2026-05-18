//! オーディオミキシングモジュール
//!
//! 複数のトラックから入力されたオーディオサンプルを合算し、
//! ボリューム、パン、ミュート、ソロを適用して最終的な出力を生成します。

/// トラックからミキサーに渡されるデータ
#[derive(Debug, Clone)]
pub struct TrackMixData<'a> {
    /// オーディオサンプルのスライス
    pub samples: &'a [f32],
    /// チャンネル数（1: モノラル、2: ステレオ）
    pub channels: u16,
    /// トラックのボリューム（0.0〜）
    pub volume: f32,
    /// トラックのパン（-1.0〜1.0）
    pub pan: f32,
    /// ミュート状態
    pub is_muted: bool,
    /// ソロ状態
    pub is_solo: bool,
}

/// 複数のトラックのサンプルをミックスし、指定されたバッファに出力します。
///
/// `out_buffer` は出力先のバッファで、合算されたサンプルが上書きされます。
/// `out_channels` は出力のチャンネル数です（現在は2（ステレオ）のみサポート）。
/// `tracks` はミキシング対象のトラックのリストです。
pub fn mix_tracks(
    out_buffer: &mut [f32],
    out_channels: u16,
    tracks: &[TrackMixData<'_>],
) {
    // 出力バッファを0クリア
    for sample in out_buffer.iter_mut() {
        *sample = 0.0;
    }

    // ステレオ出力以外は現在未対応として処理をスキップ
    if out_channels != 2 {
        return;
    }

    let has_solo = tracks.iter().any(|t| t.is_solo);

    for track in tracks {
        // ソロトラックが存在する場合、ソロでないトラックはミュートされる
        if has_solo && !track.is_solo {
            continue;
        }

        if track.is_muted || track.volume == 0.0 {
            continue;
        }

        // パンの適用 (Constant Power Panning ではなく、シンプルなリニアパンニングを仮実装)
        // -1.0(左) 〜 1.0(右) の範囲を 0.0 〜 1.0 に正規化
        let p = (track.pan + 1.0) / 2.0;
        let left_gain = (1.0 - p) * track.volume;
        let right_gain = p * track.volume;

        if track.channels == 1 {
            // モノラルトラックの場合: 同じサンプルを左右のチャンネルにパンを適用して加算
            let frames = out_buffer.len() / 2;
            let process_frames = frames.min(track.samples.len());

            for i in 0..process_frames {
                let sample = track.samples[i];
                out_buffer[i * 2] += sample * left_gain;
                out_buffer[i * 2 + 1] += sample * right_gain;
            }
        } else if track.channels == 2 {
            // ステレオトラックの場合: 左は左、右は右のサンプルにパンを適用して加算
            let frames = out_buffer.len() / 2;
            let process_frames = frames.min(track.samples.len() / 2);

            for i in 0..process_frames {
                let left_sample = track.samples[i * 2];
                let right_sample = track.samples[i * 2 + 1];

                out_buffer[i * 2] += left_sample * left_gain * 2.0; // ゲインの補正
                out_buffer[i * 2 + 1] += right_sample * right_gain * 2.0; // ゲインの補正
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_tracks_empty() {
        let mut out = vec![0.0; 4]; // 2 frames stereo
        mix_tracks(&mut out, 2, &[]);
        assert_eq!(out, vec![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_mix_tracks_mono_center() {
        let mut out = vec![0.0; 4]; // 2 frames stereo
        let samples = vec![0.5, 0.5];
        let track = TrackMixData {
            samples: &samples,
            channels: 1,
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
        };

        mix_tracks(&mut out, 2, &[track]);

        // Pan 0.0 -> left_gain = 0.5, right_gain = 0.5
        // input 0.5 * gain 0.5 = 0.25
        assert_eq!(out, vec![0.25, 0.25, 0.25, 0.25]);
    }

    #[test]
    fn test_mix_tracks_mono_pan_left() {
        let mut out = vec![0.0; 4];
        let samples = vec![1.0, 1.0];
        let track = TrackMixData {
            samples: &samples,
            channels: 1,
            volume: 1.0,
            pan: -1.0, // Left
            is_muted: false,
            is_solo: false,
        };

        mix_tracks(&mut out, 2, &[track]);

        // Pan -1.0 -> left_gain = 1.0, right_gain = 0.0
        assert_eq!(out, vec![1.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_mix_tracks_stereo() {
        let mut out = vec![0.0; 4];
        let samples = vec![0.5, 0.2, 0.5, 0.2]; // L, R, L, R
        let track = TrackMixData {
            samples: &samples,
            channels: 2,
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
        };

        mix_tracks(&mut out, 2, &[track]);

        // Pan 0.0 -> left_gain = 0.5, right_gain = 0.5
        // Stereo processing scales by 2.0 to compensate, so multiplier is 1.0.
        assert_eq!(out, vec![0.5, 0.2, 0.5, 0.2]);
    }

    #[test]
    fn test_mix_tracks_mute() {
        let mut out = vec![0.0; 4];
        let samples = vec![1.0, 1.0];
        let track = TrackMixData {
            samples: &samples,
            channels: 1,
            volume: 1.0,
            pan: 0.0,
            is_muted: true, // Muted
            is_solo: false,
        };

        mix_tracks(&mut out, 2, &[track]);
        assert_eq!(out, vec![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_mix_tracks_solo() {
        let mut out = vec![0.0; 4];
        let samples1 = vec![1.0, 1.0];
        let track1 = TrackMixData {
            samples: &samples1,
            channels: 1,
            volume: 1.0,
            pan: -1.0,
            is_muted: false,
            is_solo: false, // Not solo
        };

        let samples2 = vec![0.5, 0.5];
        let track2 = TrackMixData {
            samples: &samples2,
            channels: 1,
            volume: 1.0,
            pan: 1.0,
            is_muted: false,
            is_solo: true, // Solo
        };

        mix_tracks(&mut out, 2, &[track1, track2]);

        // Track 1 should be ignored. Track 2 pan=1.0 (Right)
        assert_eq!(out, vec![0.0, 0.5, 0.0, 0.5]);
    }

    #[test]
    fn test_mix_tracks_volume() {
        let mut out = vec![0.0; 2];
        let samples = vec![1.0];
        let track = TrackMixData {
            samples: &samples,
            channels: 1,
            volume: 0.5, // Half volume
            pan: 0.0,
            is_muted: false,
            is_solo: false,
        };

        mix_tracks(&mut out, 2, &[track]);
        // pan 0.0 (0.5) * volume 0.5 = 0.25
        assert_eq!(out, vec![0.25, 0.25]);
    }
}
