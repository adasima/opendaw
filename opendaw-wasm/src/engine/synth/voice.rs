//! ボイス管理モジュール
//!
//! エンベロープや発音状態を管理します。

use crate::state::track_plugin::AdsrParams;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
