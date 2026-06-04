//! メトロノームモジュール
//!
//! BPMと再生位置に基づいてクリック音を生成する。
//! リアルタイムオーディオコールバックから安全に呼び出せるよう、
//! アロケーションやブロッキング処理を持たない。

use std::f32::consts::PI;

const DEFAULT_FREQUENCY: f32 = 1000.0;
const ACCENT_FREQUENCY: f32 = 1500.0;
const CLICK_DURATION_SEC: f32 = 0.05;
const BEATS_PER_BAR: f32 = 4.0;

/// メトロノームのクリック音を生成する構造体
pub struct Metronome {
    sample_rate: f32,
    phase: f32,
    active_frames: usize,
    frequency: f32,
}

impl Default for Metronome {
    fn default() -> Self {
        Self::new(44100.0)
    }
}

impl Metronome {
    /// 新しいMetronomeを作成する
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            active_frames: 0,
            frequency: DEFAULT_FREQUENCY,
        }
    }

    /// サンプリングレートを設定する
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    /// 1フレーム分のサンプル（モノラル）を生成する
    fn next_sample(&mut self) -> f32 {
        if self.active_frames == 0 {
            return 0.0;
        }

        // 簡単なエンベロープ（線形減衰）
        let max_frames = (self.sample_rate * CLICK_DURATION_SEC) as usize; // クリック音
        let env = (self.active_frames as f32) / (max_frames as f32);

        let osc_val = self.phase.sin();

        // 位相を進める
        let phase_increment = 2.0 * PI * self.frequency / self.sample_rate;
        self.phase += phase_increment;
        if self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        self.active_frames -= 1;

        osc_val * env * 0.5 // 音量調整
    }

    /// 指定されたバッファにメトロノーム音をミックスする。
    /// current_sample_pos は再生開始からの累積サンプル数
    pub fn process(
        &mut self,
        buffer: &mut [f32],
        channels: u16,
        current_sample_pos: usize,
        bpm: f32,
        is_enabled: bool,
    ) {
        if !is_enabled || channels == 0 || bpm <= 0.0 {
            return;
        }

        let frames = buffer.len() / (channels as usize);
        let samples_per_beat = (self.sample_rate * 60.0) / bpm;
        let max_frames = (self.sample_rate * CLICK_DURATION_SEC) as usize;

        for i in 0..frames {
            let pos = current_sample_pos + i;

            // ビートの先頭かどうかを判定
            // (pos % samples_per_beat) が 0 になるタイミングでトリガー
            // 浮動小数点の誤差を考慮し、現在のサンプル位置が新しいビートの開始境界をまたいだかをチェックする
            let current_beat = (pos as f32 / samples_per_beat).floor();
            let prev_beat = if pos > 0 {
                ((pos - 1) as f32 / samples_per_beat).floor()
            } else {
                -1.0
            };

            if current_beat > prev_beat {
                // トリガー
                self.active_frames = max_frames;
                self.phase = 0.0;

                // 1拍目は高い音、それ以外は普通の音
                if current_beat % BEATS_PER_BAR == 0.0 {
                    self.frequency = ACCENT_FREQUENCY;
                } else {
                    self.frequency = DEFAULT_FREQUENCY;
                }
            }

            let sample = self.next_sample();

            for c in 0..(channels as usize) {
                buffer[i * (channels as usize) + c] += sample;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metronome_creation() {
        let metro = Metronome::new(48000.0);
        assert_eq!(metro.sample_rate, 48000.0);
        assert_eq!(metro.active_frames, 0);
    }

    #[test]
    fn test_metronome_trigger() {
        let mut metro = Metronome::new(44100.0);
        let mut buffer = vec![0.0; 4]; // 2 frames stereo

        // Disable state
        metro.process(&mut buffer, 2, 0, 120.0, false);
        assert_eq!(buffer, vec![0.0, 0.0, 0.0, 0.0]);

        // Enable state, pos=0 triggers the first beat
        metro.process(&mut buffer, 2, 0, 120.0, true);
        assert!(buffer[0] > 0.0 || buffer[0] == 0.0); // Sine phase 0 is 0.0, next is > 0
        assert_eq!(metro.active_frames, (44100.0 * 0.05) as usize - 2);
    }
}
