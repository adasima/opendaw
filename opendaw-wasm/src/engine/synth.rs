//! シンセサイザーモジュール
//!
//! ソフトウェアインストゥルメント（シンセサイザー）の基盤機能を提供します。
//! 基本的なオシレータ、波形選択、ADSRエンベロープを実装します。
//!
//! リアルタイムオーディオコールバックから呼ばれるため、
//! メモリアロケーションやブロッキング処理を含まないように設計されています。

use std::f32::consts::PI;

/// オシレーターの波形
#[derive(Clone, Debug, PartialEq)]
#[derive(Default)]
pub enum Waveform {
    /// サイン波
    #[default]
    Sine,
    /// 矩形波
    Square,
    /// ノコギリ波
    Sawtooth,
}


/// ADSRエンベロープのパラメータ
#[derive(Clone, Debug, PartialEq)]
pub struct AdsrParams {
    /// アタックタイム (秒)
    pub attack: f32,
    /// ディケイタイム (秒)
    pub decay: f32,
    /// サステインレベル (0.0 〜 1.0)
    pub sustain: f32,
    /// リリースタイム (秒)
    pub release: f32,
}

impl Default for AdsrParams {
    fn default() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
        }
    }
}

/// ADSRエンベロープの現在の状態
#[derive(Clone, Debug, PartialEq)]
pub enum AdsrState {
    /// 停止中
    Idle,
    /// アタックフェーズ
    Attack,
    /// ディケイフェーズ
    Decay,
    /// サステインフェーズ
    Sustain,
    /// リリースフェーズ
    Release,
}

/// ADSRエンベロープ処理
#[derive(Clone, Debug)]
pub struct AdsrEnvelope {
    /// パラメータ
    pub params: AdsrParams,
    /// 現在の状態
    pub state: AdsrState,
    /// 現在のエンベロープ値
    pub value: f32,
    /// サンプリングレート
    sample_rate: f32,
}

impl AdsrEnvelope {
    /// 新しいエンベロープを作成します
    pub fn new(sample_rate: f32) -> Self {
        Self {
            params: AdsrParams::default(),
            state: AdsrState::Idle,
            value: 0.0,
            sample_rate,
        }
    }

    /// サンプリングレートを設定します
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    /// ノートオン（発音開始）をトリガーします
    pub fn note_on(&mut self) {
        self.state = AdsrState::Attack;
    }

    /// ノートオフ（発音終了）をトリガーします
    pub fn note_off(&mut self) {
        if self.state != AdsrState::Idle {
            self.state = AdsrState::Release;
        }
    }

    /// 発音が完全に終了しているかを取得します
    pub fn is_idle(&self) -> bool {
        self.state == AdsrState::Idle
    }

    /// 次のエンベロープ値を1つ生成します
    pub fn next_value(&mut self) -> f32 {
        match self.state {
            AdsrState::Idle => {
                self.value = 0.0;
            }
            AdsrState::Attack => {
                let rate = 1.0 / (self.params.attack * self.sample_rate + 1e-5);
                self.value += rate;
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.state = AdsrState::Decay;
                }
            }
            AdsrState::Decay => {
                let rate = (1.0 - self.params.sustain) / (self.params.decay * self.sample_rate + 1e-5);
                self.value -= rate;
                if self.value <= self.params.sustain {
                    self.value = self.params.sustain;
                    self.state = AdsrState::Sustain;
                }
            }
            AdsrState::Sustain => {
                self.value = self.params.sustain;
            }
            AdsrState::Release => {
                let rate = self.value / (self.params.release * self.sample_rate + 1e-5);
                self.value -= rate;
                if self.value <= 0.0 {
                    self.value = 0.0;
                    self.state = AdsrState::Idle;
                }
            }
        }
        self.value
    }
}

/// 基本的なオシレータ
#[derive(Clone, Debug)]
pub struct Oscillator {
    /// サンプリングレート
    sample_rate: f32,
    /// 周波数 (Hz)
    frequency: f32,
    /// 現在の位相 (0.0 〜 2.0 * PI)
    phase: f32,
    /// 発音状態 (エンベロープがIdleでない場合発音中)
    is_active: bool,
    /// 波形
    pub waveform: Waveform,
    /// ADSRエンベロープ
    pub envelope: AdsrEnvelope,
}

impl Oscillator {
    /// 新しいオシレータを作成します
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            frequency: 440.0,
            phase: 0.0,
            is_active: false,
            waveform: Waveform::default(),
            envelope: AdsrEnvelope::new(sample_rate),
        }
    }

    /// サンプリングレートを設定します
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.envelope.set_sample_rate(sample_rate);
    }

    /// 周波数を設定します
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    /// 発音状態を設定します
    pub fn set_active(&mut self, active: bool) {
        if active && !self.is_active {
            // 新規ノートオン
            self.phase = 0.0;
            self.envelope.note_on();
        } else if !active && self.is_active {
            // ノートオフ
            self.envelope.note_off();
        }
        self.is_active = active;
    }

    /// 発音中かどうかを取得します
    pub fn is_active(&self) -> bool {
        !self.envelope.is_idle()
    }

    /// 次のサンプルを1つ生成し、位相を進めます。
    /// エンベロープが適用された値が返ります。
    pub fn next_sample(&mut self) -> f32 {
        if self.envelope.is_idle() {
            return 0.0;
        }

        let env_val = self.envelope.next_value();

        let osc_val = match self.waveform {
            Waveform::Sine => self.phase.sin(),
            Waveform::Square => if self.phase < PI { 1.0 } else { -1.0 },
            Waveform::Sawtooth => (self.phase / PI) - 1.0,
        };

        // 位相を進める
        let phase_increment = 2.0 * PI * self.frequency / self.sample_rate;
        self.phase += phase_increment;
        if self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        osc_val * env_val
    }

    /// 指定されたバッファに対して、生成したサンプルを加算します。
    pub fn process_add(&mut self, buffer: &mut [f32], channels: u16) {
        if self.envelope.is_idle() || channels == 0 {
            return;
        }

        let frames = buffer.len() / (channels as usize);
        for i in 0..frames {
            let sample = self.next_sample();

            // 各チャンネルに加算
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
