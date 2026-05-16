/// Audio Engine Constants
/// Centralized configuration for audio processing

/// Meter update interval in milliseconds (~30fps)
pub const METER_UPDATE_INTERVAL_MS: u64 = 32;

/// Playhead update interval in milliseconds (~60fps)
pub const PLAYHEAD_UPDATE_INTERVAL_MS: u64 = 16;

/// MIDI reference frequency (A4 = 440Hz)
pub const MIDI_A4_FREQUENCY: f32 = 440.0;

/// MIDI note number for A4
pub const MIDI_A4_NOTE: u8 = 69;

/// Maximum MIDI velocity value
pub const MIDI_VELOCITY_MAX: f32 = 127.0;

/// Default synth output gain (amplitude multiplier)
pub const SYNTH_OUTPUT_GAIN: f32 = 0.1;

/// Initial voice vector capacity per track
pub const VOICE_INITIAL_CAPACITY: usize = 32;
