//! ディレイエフェクト
//!
//! オーディオ信号を遅延させて再生するエフェクトです。

use crate::engine::effects::AudioEffect;

pub const MAX_DELAY_SAMPLES: usize = 48000;

/// ディレイエフェクトの実装
pub struct DelayEffect {
    /// 遅延時間（ミリ秒）
    pub time_ms: f32,
    /// フィードバック係数 (0.0 = フィードバックなし, 1.0 = 無限ループ)
    pub feedback: f32,
    /// 原音とディレイ音のミックス割合 (0.0 = 原音のみ, 1.0 = ディレイ音のみ)
    pub mix: f32,
    /// エフェクトが有効かどうか
    pub enabled: bool,
    /// サンプルレート
    sample_rate: f32,
    /// 内部リングバッファ (ヒープアロケーションを避けるための固定長配列)
    buffer: [f32; MAX_DELAY_SAMPLES],
    /// バッファの書き込み位置
    write_index: usize,
}

impl Default for DelayEffect {
    fn default() -> Self {
        Self::new(48000.0) // デフォルトサンプルレート
    }
}

impl DelayEffect {
    /// 新しいディレイエフェクトを作成します
    pub fn new(sample_rate: f32) -> Self {
        Self {
            time_ms: 300.0, // デフォルト300ms
            feedback: 0.3,  // デフォルト30%のフィードバック
            mix: 0.5,       // デフォルト50%のミックス
            enabled: true,
            sample_rate,
            buffer: [0.0; MAX_DELAY_SAMPLES],
            write_index: 0,
        }
    }

    /// パラメータを設定します
    pub fn set_params(&mut self, time_ms: f32, feedback: f32, mix: f32) {
        self.time_ms = time_ms.max(0.0);
        self.feedback = feedback.clamp(0.0, 1.0);
        self.mix = mix.clamp(0.0, 1.0);
    }
}

impl AudioEffect for DelayEffect {
    fn process(&mut self, buffer: &mut [f32], channels: u16) {
        if !self.enabled {
            return;
        }

        // 遅延サンプル数を計算 (インターリーブされたチャンネルを考慮)
        let delay_frames = ((self.time_ms / 1000.0) * self.sample_rate) as usize;
        let mut delay_samples = delay_frames * (channels as usize);

        // メモリの制約に従い、underflow panicを防ぐために .min(max_samples) を適用
        let max_samples = self.buffer.len();
        delay_samples = delay_samples.min(max_samples);

        // チャンネル単位のアライメントを維持するため、channelsの倍数に丸める
        let c = channels as usize;
        if c > 0 {
            delay_samples = (delay_samples / c) * c;
        }

        // バッファが空または遅延が0の場合は処理しない
        if max_samples == 0 || delay_samples == 0 {
            return;
        }

        for sample in buffer.iter_mut() {
            // 現在のサンプルを読み込み
            let current_sample = *sample;

            // 読み込みインデックスを計算
            let mut read_index = self.write_index as isize - delay_samples as isize;
            if read_index < 0 {
                read_index += max_samples as isize;
            }
            let delayed_sample = self.buffer[read_index as usize];

            // 新しいサンプルをリングバッファに書き込み
            self.buffer[self.write_index] = current_sample + delayed_sample * self.feedback;

            // 出力サンプルを計算（原音とディレイ音のミックス）
            *sample = current_sample * (1.0 - self.mix) + delayed_sample * self.mix;

            // 書き込みインデックスを更新
            self.write_index = (self.write_index + 1) % max_samples;
        }
    }

    fn name(&self) -> &str {
        "Delay"
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
    fn test_delay_effect_new() {
        let effect = DelayEffect::new(48000.0);
        assert_eq!(effect.time_ms, 300.0);
        assert_eq!(effect.feedback, 0.3);
        assert_eq!(effect.mix, 0.5);
        assert!(effect.is_enabled());
        assert_eq!(effect.name(), "Delay");
    }

    #[test]
    fn test_delay_effect_set_params() {
        let mut effect = DelayEffect::new(48000.0);
        effect.set_params(500.0, 0.8, 0.7);
        assert_eq!(effect.time_ms, 500.0);
        assert_eq!(effect.feedback, 0.8);
        assert_eq!(effect.mix, 0.7);

        // クランプのテスト
        effect.set_params(-100.0, 1.5, -0.5);
        assert_eq!(effect.time_ms, 0.0);
        assert_eq!(effect.feedback, 1.0);
        assert_eq!(effect.mix, 0.0);
    }
}
