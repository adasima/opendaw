//! オーディバスモジュール
//!
//! 各トラックのオシレーター、エフェクト、およびそれらに関連する状態を管理する構造体です。

use crate::engine::channel::{EffectParams, UiToAudioMsg};
use crate::engine::effects::{AudioEffect, delay::DelayEffect, filter::{BiquadFilter, FilterType}, gain::GainEffect};
use crate::engine::synth::Oscillator;
use crate::engine::channel::MAX_ACTIVE_NOTES;

/// 1つのトラックのオーディオエフェクトやシンセサイザーの状態を保持するバス
pub struct AudioBus {
    pub oscillator: Oscillator,
    pub delay: DelayEffect,
    pub gain: GainEffect,
    pub filter: BiquadFilter,
    pub active_notes: [f32; MAX_ACTIVE_NOTES],
    pub active_note_count: usize,
}

impl AudioBus {
    /// 新しいAudioBusを作成します。
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(sample_rate),
            delay: DelayEffect::new(sample_rate),
            gain: GainEffect::new(1.0),
            filter: BiquadFilter::new(FilterType::LowPass, 1000.0, sample_rate),
            active_notes: [0.0; MAX_ACTIVE_NOTES],
            active_note_count: 0,
        }
    }

    /// UIからのメッセージを受け取り、状態を更新します。
    pub fn process_ui_message(&mut self, msg: &UiToAudioMsg) {
        match msg {
            UiToAudioMsg::ActiveNotes(_id, notes, count) => {
                self.active_notes = *notes;
                self.active_note_count = *count;
            }
            UiToAudioMsg::UpdateSynthParams(_id, waveform, params) => {
                self.oscillator.waveform = waveform.clone();
                self.oscillator.envelope.params = params.clone();
            }
            UiToAudioMsg::UpdateEffectParams(_track_id, _effect_id, params) => {
                match params {
                    EffectParams::Delay { time_ms, feedback, mix } => {
                        self.delay.set_params(*time_ms, *feedback, *mix);
                    }
                    EffectParams::Gain(gain) => {
                        self.gain.set_gain(*gain);
                    }
                    EffectParams::Filter { cutoff_freq, filter_type } => {
                        self.filter.set_cutoff(*cutoff_freq);
                        self.filter.set_type(filter_type.clone());
                    }
                }
            }
            UiToAudioMsg::SetEffectEnabled(_track_id, effect_id, enabled) => {
                // 簡易的にeffect_idで区別 (0: Gain, 1: Filter, 2: Delay)
                match effect_id {
                    0 => self.gain.set_enabled(*enabled),
                    1 => self.filter.set_enabled(*enabled),
                    _ => self.delay.set_enabled(*enabled),
                }
            }
            _ => {}
        }
    }
}
