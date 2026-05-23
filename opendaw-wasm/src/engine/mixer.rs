//! オーディオミキシングモジュール
//!
//! 複数のトラックから入力されたオーディオサンプルを合算し、
//! ボリューム、パン、ミュート、ソロを適用して最終的な出力を生成します。

use crate::engine::effects::AudioEffect;

/// トラックからミキサーに渡されるデータ
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
    /// 適用するエフェクトのチェーン
    pub effects: &'a mut [&'a mut dyn AudioEffect],
    /// オシレーター（シンセサイザー）
    pub oscillator: Option<&'a mut crate::engine::synth::Oscillator>,
    /// アクティブなノートの周波数
    pub active_notes: [f32; crate::engine::channel::MAX_ACTIVE_NOTES],
    /// アクティブなノートの数
    pub active_note_count: usize,
}

/// 複数のトラックのサンプルをミックスし、指定されたバッファに出力します。
///
/// `out_buffer` は出力先のバッファで、合算されたサンプルが上書きされます。
/// `out_channels` は出力のチャンネル数です（現在は2（ステレオ）のみサポート）。
/// `tracks` はミキシング対象のトラックのリストです。
pub fn mix_tracks(out_buffer: &mut [f32], out_channels: u16, tracks: &mut [TrackMixData<'_>]) {
    // 出力バッファを0クリア
    for sample in out_buffer.iter_mut() {
        *sample = 0.0;
    }

    // ステレオ出力以外は現在未対応として処理をスキップ
    if out_channels != 2 {
        return;
    }

    let has_solo = tracks.iter().any(|t| t.is_solo);

    // 最大処理フレームサイズ。スタックに乗る固定バッファとして2048サンプル分(1024フレームのステレオ)を用意
    const CHUNK_FRAMES: usize = 1024;
    let mut temp_buf = [0.0; CHUNK_FRAMES * 2];

    for track in tracks.iter_mut() {
        // ソロトラックが存在する場合、ソロでないトラックはミュートされる
        if has_solo && !track.is_solo {
            continue;
        }

        if track.is_muted || track.volume == 0.0 {
            continue;
        }

        // モノラルまたはステレオ以外は現在未対応としてスキップ
        if track.channels != 1 && track.channels != 2 {
            continue;
        }

        // パンの適用 (Constant Power Panning ではなく、シンプルなリニアパンニングを仮実装)
        // -1.0(左) 〜 1.0(右) の範囲を 0.0 〜 1.0 に正規化
        let p = (track.pan + 1.0) / 2.0;
        let left_gain = (1.0 - p) * track.volume;
        let right_gain = p * track.volume;

        let frames = out_buffer.len() / 2;
        let process_frames = frames.min(track.samples.len() / track.channels as usize);

        let mut i = 0;
        while i < process_frames {
            let frames_to_process = (process_frames - i).min(CHUNK_FRAMES);
            let actual_samples = frames_to_process * track.channels as usize;

            let start_idx = i * track.channels as usize;
            let end_idx = start_idx + actual_samples;

            // 入力サンプルを一時バッファにコピー
            temp_buf[..actual_samples].copy_from_slice(&track.samples[start_idx..end_idx]);

            // オシレーターのサンプルを加算
            if let Some(osc) = track.oscillator.as_mut() {
                if track.active_note_count > 0 {
                    osc.set_frequency(track.active_notes[0]);
                    osc.set_active(true);
                } else {
                    osc.set_active(false);
                }
                osc.process_add(&mut temp_buf[..actual_samples], track.channels);
            }

            // エフェクトの適用
            for effect in track.effects.iter_mut() {
                effect.process(&mut temp_buf[..actual_samples], track.channels);
            }

            // ミックス処理
            if track.channels == 1 {
                for (j, sample) in temp_buf.iter().enumerate().take(frames_to_process) {
                    let out_idx = (i + j) * 2;
                    out_buffer[out_idx] += sample * left_gain;
                    out_buffer[out_idx + 1] += sample * right_gain;
                }
            } else if track.channels == 2 {
                for j in 0..frames_to_process {
                    let left_sample = temp_buf[j * 2];
                    let right_sample = temp_buf[j * 2 + 1];
                    let out_idx = (i + j) * 2;
                    out_buffer[out_idx] += left_sample * left_gain * 2.0; // ゲインの補正
                    out_buffer[out_idx + 1] += right_sample * right_gain * 2.0; // ゲインの補正
                }
            }

            i += frames_to_process;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_tracks_empty() {
        let mut out = vec![0.0; 4]; // 2 frames stereo
        mix_tracks(&mut out, 2, &mut []);
        assert_eq!(out, vec![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    #[allow(unused_mut)]
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
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track]);

        // Pan 0.0 -> left_gain = 0.5, right_gain = 0.5
        // input 0.5 * gain 0.5 = 0.25
        assert_eq!(out, vec![0.25, 0.25, 0.25, 0.25]);
    }

    #[test]
    #[allow(unused_mut)]
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
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track]);

        // Pan -1.0 -> left_gain = 1.0, right_gain = 0.0
        assert_eq!(out, vec![1.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    #[allow(unused_mut)]
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
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track]);

        // Pan 0.0 -> left_gain = 0.5, right_gain = 0.5
        // Stereo processing scales by 2.0 to compensate, so multiplier is 1.0.
        assert_eq!(out, vec![0.5, 0.2, 0.5, 0.2]);
    }

    #[test]
    #[allow(unused_mut)]
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
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track]);
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
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        let samples2 = vec![0.5, 0.5];
        let track2 = TrackMixData {
            samples: &samples2,
            channels: 1,
            volume: 1.0,
            pan: 1.0,
            is_muted: false,
            is_solo: true, // Solo
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track1, track2]);

        // Track 1 should be ignored. Track 2 pan=1.0 (Right)
        assert_eq!(out, vec![0.0, 0.5, 0.0, 0.5]);
    }

    #[test]
    #[allow(unused_mut)]
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
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track]);
        // pan 0.0 (0.5) * volume 0.5 = 0.25
        assert_eq!(out, vec![0.25, 0.25]);
    }

    // ゲインエフェクトのモック
    struct MockGainEffect {
        gain: f32,
    }

    impl AudioEffect for MockGainEffect {
        fn process(&mut self, buffer: &mut [f32], _channels: u16) {
            for sample in buffer.iter_mut() {
                *sample *= self.gain;
            }
        }
        fn name(&self) -> &str {
            "Mock Gain"
        }
        fn is_enabled(&self) -> bool {
            true
        }
        fn set_enabled(&mut self, _enabled: bool) {}
    }

    #[test]
    fn test_mix_tracks_with_effects() {
        let mut out = vec![0.0; 4];
        let samples = vec![1.0, 1.0];

        let mut effect = MockGainEffect { gain: 0.5 };
        let mut effects: [&mut dyn AudioEffect; 1] = [&mut effect];

        let track = TrackMixData {
            samples: &samples,
            channels: 1,
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            effects: &mut effects,
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track]);

        // track pan=0.0 -> left_gain=0.5, right_gain=0.5
        // input=1.0 -> effect_gain=0.5 -> 0.5
        // output = 0.5 * gain(0.5) = 0.25
        assert_eq!(out, vec![0.25, 0.25, 0.25, 0.25]);
    }

    #[test]
    fn test_mix_tracks_with_oscillator() {
        let mut out = vec![0.0; 4];
        let samples = vec![0.0, 0.0]; // 無音のサンプル

        let mut osc = crate::engine::synth::Oscillator::new(44100.0);
        osc.set_frequency(1.0);
        // active_notes[0] で周波数が上書きされるため 1.0 を設定
        osc.set_active(true);
        // sample 1: sin(0) = 0.0, sample 2: sin(2π * 1 / 44100) ≈ 0.000142...

        let mut active_notes = [0.0; crate::engine::channel::MAX_ACTIVE_NOTES];
        active_notes[0] = 1.0;
        let track = TrackMixData {
            samples: &samples,
            channels: 1,
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            active_notes,
            active_note_count: 1,
            effects: &mut [],
            oscillator: Some(&mut osc),
        };

        mix_tracks(&mut out, 2, &mut [track]);

        // オシレーターの出力がミックスされているか確認
        assert_eq!(out[0], 0.0); // sample 1 L (0.0 * 0.5)
        assert_eq!(out[1], 0.0); // sample 1 R (0.0 * 0.5)
        assert!(out[2] > 0.0); // sample 2 L (sin(...) * 0.5)
        assert!(out[3] > 0.0); // sample 2 R (sin(...) * 0.5)
    }

    #[test]
    fn test_mix_tracks_invalid_channels() {
        let mut out = vec![0.0; 4];
        let samples = vec![1.0, 1.0, 1.0];
        let track = TrackMixData {
            samples: &samples,
            channels: 0, // 0チャンネルはエラーにならないようにスキップされる
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track]);
        assert_eq!(out, vec![0.0, 0.0, 0.0, 0.0]);

        let track2 = TrackMixData {
            samples: &samples,
            channels: 3, // 3チャンネル以上も現在未対応なのでスキップされる
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            effects: &mut [],
            oscillator: None,
            active_notes: [0.0; crate::engine::channel::MAX_ACTIVE_NOTES],
            active_note_count: 0,
        };

        mix_tracks(&mut out, 2, &mut [track2]);
        assert_eq!(out, vec![0.0, 0.0, 0.0, 0.0]);
    }
}

/// マスターバッファにメトロノーム音をミックスします。
pub fn mix_metronome(
    out_buffer: &mut [f32],
    channels: u16,
    metronome: &mut crate::engine::metronome::Metronome,
    playhead_samples: usize,
    bpm: f32,
    is_enabled: bool,
) {
    metronome.process(out_buffer, channels, playhead_samples, bpm, is_enabled);
}

#[cfg(test)]
mod metronome_mixer_tests {
    use super::*;

    #[test]
    fn test_mix_metronome() {
        let mut out = vec![0.0; 4];
        let mut metro = crate::engine::metronome::Metronome::new(44100.0);
        mix_metronome(&mut out, 2, &mut metro, 0, 120.0, true);
        // metronome output added
        // It should modify the out buffer
    }
}
