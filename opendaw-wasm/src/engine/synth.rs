//! シンセサイザーモジュール
//!
//! ソフトウェアインストゥルメント（シンセサイザー）の基盤機能を提供します。
//! 基本的なオシレータ、波形選択、ADSRエンベロープを実装します。

pub use crate::engine::oscillator::*;
pub use crate::engine::voice::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_waveform_default() {
        assert_eq!(Waveform::default(), Waveform::Sine);
    }

    #[test]
    fn test_adsr_envelope() {
        let mut env = AdsrEnvelope::new(100.0);
        env.params = AdsrParams {
            attack: 0.1,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
        };

        assert_eq!(env.state, AdsrState::Idle);
        env.note_on();
        assert_eq!(env.state, AdsrState::Attack);

        // Attack phase: 0.1s * 100Hz = 10 samples
        for _ in 0..11 {
            env.next_value();
        }
        assert_eq!(env.state, AdsrState::Decay);
        assert!((env.value - 1.0).abs() < 1e-4);

        // Note off
        env.note_off();
        assert_eq!(env.state, AdsrState::Release);
    }

    #[test]
    fn test_oscillator_new() {
        let osc = Oscillator::new(44100.0);
        assert_eq!(osc.sample_rate, 44100.0);
        assert_eq!(osc.frequency, 440.0);
        assert_eq!(osc.phase, 0.0);
        assert!(!osc.is_active());
        assert_eq!(osc.waveform, Waveform::Sine);
    }

    #[test]
    fn test_oscillator_waveforms() {
        let mut osc = Oscillator::new(4.0);
        osc.set_frequency(1.0);

        // Attack 0 so env is instantly 1.0
        osc.envelope.params.attack = 0.0;
        osc.set_active(true);
        // consume attack (value jumps to 1.0 and state to Decay)
        osc.next_sample();

        osc.waveform = Waveform::Sine;
        osc.phase = PI / 2.0;
        let val = osc.next_sample();
        assert!(val > 0.0);

        osc.waveform = Waveform::Square;
        osc.phase = PI / 2.0;
        let val = osc.next_sample();
        assert!(val > 0.0);

        osc.waveform = Waveform::Sawtooth;
        osc.phase = PI / 2.0;
        let val = osc.next_sample();
        assert!(val < 0.0); // (pi/2) / pi - 1 = -0.5
    }
}
