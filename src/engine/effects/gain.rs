//! ゲインエフェクト
//!
//! オーディオ信号の音量を変更するシンプルなエフェクトです。

use crate::engine::effects::AudioEffect;

/// ゲインエフェクトの実装
pub struct GainEffect {
    /// ゲイン係数 (1.0 = 変化なし, 0.0 = 無音, >1.0 = ブースト)
    pub gain: f32,
    /// エフェクトが有効かどうか
    pub enabled: bool,
}

impl Default for GainEffect {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl GainEffect {
    /// 新しいゲインエフェクトを作成します
    pub fn new(gain: f32) -> Self {
        Self {
            gain,
            enabled: true,
        }
    }

    /// ゲインを設定します
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain.max(0.0);
    }
}

impl AudioEffect for GainEffect {
    fn process(&mut self, buffer: &mut [f32], _channels: u16) {
        if !self.enabled || (self.gain - 1.0).abs() < f32::EPSILON {
            return;
        }

        for sample in buffer.iter_mut() {
            *sample *= self.gain;
        }
    }

    fn name(&self) -> &str {
        "Gain"
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
    fn test_gain_effect_new() {
        let effect = GainEffect::new(0.5);
        assert_eq!(effect.gain, 0.5);
        assert!(effect.is_enabled());
        assert_eq!(effect.name(), "Gain");
    }

    #[test]
    fn test_gain_effect_set_gain() {
        let mut effect = GainEffect::new(1.0);
        effect.set_gain(2.0);
        assert_eq!(effect.gain, 2.0);

        effect.set_gain(-1.0);
        assert_eq!(effect.gain, 0.0); // 0.0にクランプされる
    }

    #[test]
    fn test_gain_effect_process() {
        let mut effect = GainEffect::new(0.5);
        let mut buffer = vec![1.0, -1.0, 0.5, -0.5];

        effect.process(&mut buffer, 2);

        assert_eq!(buffer[0], 0.5);
        assert_eq!(buffer[1], -0.5);
        assert_eq!(buffer[2], 0.25);
        assert_eq!(buffer[3], -0.25);
    }

    #[test]
    fn test_gain_effect_bypass_when_disabled() {
        let mut effect = GainEffect::new(0.5);
        effect.set_enabled(false);
        let mut buffer = vec![1.0, -1.0, 0.5, -0.5];

        effect.process(&mut buffer, 2);

        // 無効な場合はバッファが変更されない
        assert_eq!(buffer[0], 1.0);
        assert_eq!(buffer[1], -1.0);
        assert_eq!(buffer[2], 0.5);
        assert_eq!(buffer[3], -0.5);
    }
}
