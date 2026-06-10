use std::sync::OnceLock;

static PITCH_TO_FREQ: OnceLock<[f32; 128]> = OnceLock::new();

#[inline(always)]
pub fn pitch_to_freq(pitch: u8) -> f32 {
    let table = PITCH_TO_FREQ.get_or_init(|| {
        let mut table = [0.0; 128];
        for (i, freq) in table.iter_mut().enumerate() {
            *freq = 440.0 * 2.0_f32.powf((i as f32 - 69.0) / 12.0);
        }
        table
    });

    if pitch < 128 {
        table[pitch as usize]
    } else {
        // Fallback for unexpected values
        440.0 * 2.0_f32.powf((pitch as f32 - 69.0) / 12.0)
    }
}
