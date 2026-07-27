use crate::state::{SynthParams, OscillatorType};
use crate::constants;

pub fn calculate_envelope(
    t: f64,
    note_duration: f64,
    synth_params: &SynthParams,
) -> f64 {
    if t < synth_params.attack {
        t / synth_params.attack
    } else if t < synth_params.attack + synth_params.decay {
        1.0 - (1.0 - synth_params.sustain) * ((t - synth_params.attack) / synth_params.decay)
    } else if t < note_duration {
        synth_params.sustain
    } else {
        let release_t = t - note_duration;
        if release_t < synth_params.release {
            synth_params.sustain * (1.0 - (release_t / synth_params.release))
        } else {
            0.0
        }
    }
}

pub fn calculate_oscillator(
    phase: f32,
    oscillator_type: OscillatorType,
) -> f32 {
    match oscillator_type {
        OscillatorType::Sine => phase.sin(),
        OscillatorType::Square => if phase.sin() > 0.0 { 1.0 } else { -1.0 },
        OscillatorType::Sawtooth => {
            let period_pos = (phase / (2.0 * std::f32::consts::PI)) % 1.0;
            2.0 * (period_pos - 0.5)
        },
        OscillatorType::Triangle => {
            let period_pos = (phase / (2.0 * std::f32::consts::PI)).fract(); // 0..1
            if period_pos < 0.5 {
                4.0 * period_pos - 1.0
            } else {
                3.0 - 4.0 * period_pos
            }
        }
    }
}

pub fn calculate_voice_sample(
    current_time: f64,
    start_time: f64,
    note_duration: f64,
    frequency: f32,
    velocity: f32,
    synth_params: &SynthParams,
) -> f32 {
    let t = current_time - start_time;
    let envelope = calculate_envelope(t, note_duration, synth_params);
    let phase = current_time as f32 * frequency * 2.0 * std::f32::consts::PI;
    let osc_val = calculate_oscillator(phase, synth_params.oscillator_type);

    osc_val * constants::SYNTH_OUTPUT_GAIN * envelope as f32 * velocity
}
