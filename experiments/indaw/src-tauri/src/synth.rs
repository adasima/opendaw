use crate::state::{SynthParams, OscillatorType};

pub fn calculate_envelope(t: f64, duration: f64, params: &SynthParams) -> f64 {
    if t < params.attack {
        t / params.attack
    } else if t < params.attack + params.decay {
        1.0 - (1.0 - params.sustain) * ((t - params.attack) / params.decay)
    } else if t < duration {
        params.sustain
    } else {
        let release_t = t - duration;
        if release_t < params.release {
            params.sustain * (1.0 - (release_t / params.release))
        } else {
            0.0
        }
    }
}

pub fn calculate_oscillator(phase: f32, osc_type: OscillatorType) -> f32 {
    match osc_type {
        OscillatorType::Sine => phase.sin(),
        OscillatorType::Square => if phase.sin() > 0.0 { 1.0 } else { -1.0 },
        OscillatorType::Sawtooth => {
            let period_pos = (phase / (2.0 * std::f32::consts::PI)) % 1.0;
            2.0 * (period_pos - 0.5)
        },
        OscillatorType::Triangle => {
            let period_pos = (phase / (2.0 * std::f32::consts::PI)).fract();
            if period_pos < 0.5 {
                4.0 * period_pos - 1.0
            } else {
                3.0 - 4.0 * period_pos
            }
        }
    }
}

pub fn note_to_frequency(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}
