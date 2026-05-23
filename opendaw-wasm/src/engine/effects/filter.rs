//! フィルターエフェクト
//!
//! 基本的なローパス/ハイパスフィルタ等を提供するバイカッドフィルタの実装です。

use crate::engine::effects::AudioEffect;
use std::f32::consts::PI;

/// フィルターの種類
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    /// ローパスフィルター
    LowPass,
    /// ハイパスフィルター
    HighPass,
}

/// バイカッドフィルタによるエフェクト実装
pub struct BiquadFilter {
    filter_type: FilterType,
    cutoff_freq: f32,
    sample_rate: f32,
    q_factor: f32,
    enabled: bool,

    // フィルタ係数
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    // 状態変数 (ステレオ対応のためチャンネル数分必要ですが、
    // ここでは簡易的に最大2チャンネル分を保持します)
    z1: [f32; 2],
    z2: [f32; 2],
}

impl Default for BiquadFilter {
    fn default() -> Self {
        Self::new(FilterType::LowPass, 1000.0, 44100.0)
    }
}

impl BiquadFilter {
    /// 新しいバイカッドフィルタを作成します
    pub fn new(filter_type: FilterType, cutoff_freq: f32, sample_rate: f32) -> Self {
        let mut filter = Self {
            filter_type,
            cutoff_freq,
            sample_rate,
            q_factor: std::f32::consts::FRAC_1_SQRT_2, // バタワース特性
            enabled: true,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            z1: [0.0; 2],
            z2: [0.0; 2],
        };
        filter.calculate_coefficients();
        filter
    }

    /// カットオフ周波数を設定します
    pub fn set_cutoff(&mut self, freq: f32) {
        self.cutoff_freq = freq.clamp(20.0, self.sample_rate / 2.0);
        self.calculate_coefficients();
    }

    /// フィルタの種類を設定します
    pub fn set_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
        self.calculate_coefficients();
    }

    /// サンプルレートを設定します
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.calculate_coefficients();
    }

    fn calculate_coefficients(&mut self) {
        let omega = 2.0 * PI * self.cutoff_freq / self.sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * self.q_factor);

        match self.filter_type {
            FilterType::LowPass => {
                let a0 = 1.0 + alpha;
                self.b0 = ((1.0 - cos_omega) / 2.0) / a0;
                self.b1 = (1.0 - cos_omega) / a0;
                self.b2 = ((1.0 - cos_omega) / 2.0) / a0;
                self.a1 = (-2.0 * cos_omega) / a0;
                self.a2 = (1.0 - alpha) / a0;
            }
            FilterType::HighPass => {
                let a0 = 1.0 + alpha;
                self.b0 = ((1.0 + cos_omega) / 2.0) / a0;
                self.b1 = -(1.0 + cos_omega) / a0;
                self.b2 = ((1.0 + cos_omega) / 2.0) / a0;
                self.a1 = (-2.0 * cos_omega) / a0;
                self.a2 = (1.0 - alpha) / a0;
            }
        }
    }
}

impl AudioEffect for BiquadFilter {
    fn process(&mut self, buffer: &mut [f32], channels: u16) {
        if !self.enabled {
            return;
        }

        let process_channels = channels.min(2) as usize; // 状態変数は最大2チャンネルまで保持
        let actual_channels = channels as usize;

        if actual_channels == 0 {
            return;
        }

        // Transposed Direct Form 2 を使用した正確な実装
        for chunk in buffer.chunks_exact_mut(actual_channels) {
            // 処理可能なチャンネル（最大2）のみを処理
            for (ch, sample) in chunk.iter_mut().take(process_channels).enumerate() {
                let input = *sample;
                let output = self.b0 * input + self.z1[ch];
                self.z1[ch] = self.b1 * input - self.a1 * output + self.z2[ch];
                self.z2[ch] = self.b2 * input - self.a2 * output;
                *sample = output;
            }
        }
    }

    fn name(&self) -> &str {
        match self.filter_type {
            FilterType::LowPass => "LowPass Filter",
            FilterType::HighPass => "HighPass Filter",
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biquad_filter_new() {
        let filter = BiquadFilter::new(FilterType::LowPass, 1000.0, 44100.0);
        assert_eq!(filter.filter_type, FilterType::LowPass);
        assert_eq!(filter.cutoff_freq, 1000.0);
        assert!(filter.is_enabled());
        assert_eq!(filter.name(), "LowPass Filter");
    }

    #[test]
    fn test_biquad_filter_set_cutoff() {
        let mut filter = BiquadFilter::default();
        let old_b0 = filter.b0;

        filter.set_cutoff(2000.0);
        assert_eq!(filter.cutoff_freq, 2000.0);
        assert_ne!(filter.b0, old_b0); // 係数が再計算されたことを確認
    }

    #[test]
    fn test_biquad_filter_process() {
        let mut filter = BiquadFilter::new(FilterType::LowPass, 1000.0, 44100.0);
        let mut buffer = vec![1.0, -1.0, 0.5, -0.5];

        filter.process(&mut buffer, 2);

        // 出力が変化していることを確認
        assert_ne!(buffer[0], 1.0);
        assert_ne!(buffer[1], -1.0);
    }
}
