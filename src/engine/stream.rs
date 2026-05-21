use crate::engine::audio_file::AudioBuffer;
use crate::engine::channel::{AudioChannels, AudioToUiMsg, UiToAudioMsg};
use cpal::traits::DeviceTrait;
use cpal::{Device, SampleFormat, Stream, StreamConfig};
use ringbuf::traits::{Consumer, Producer};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// オーディオストリームの構築エラー
#[derive(Debug)]
pub enum StreamBuildError {
    /// デフォルトデバイスが見つからない場合
    NoDevice,
    /// サポートされていないサンプルフォーマットの場合
    UnsupportedFormat(SampleFormat),
    /// cpalのストリーム構築エラー
    CpalError(cpal::BuildStreamError),
}

/// I16用のミキシングバッファサイズ
const MIX_BUFFER_SIZE: usize = 65536;

/// 再生コンテキスト（ロックフリーにオーディオスレッドへ渡す状態）
pub struct PlaybackContext {
    /// オーディオデータ
    pub buffer: AudioBuffer,
    /// 現在の再生位置（サンプル単位）
    pub position: Arc<AtomicUsize>,
    /// 再生中かどうか
    pub playing: Arc<AtomicBool>,
    /// UI通信チャンネル
    pub channels: Option<AudioChannels>,
}

/// f32バッファに無音（0.0）を書き込む
pub fn write_silence_f32(buffer: &mut [f32], _channels: u16) {
    for sample in buffer.iter_mut() {
        *sample = 0.0;
    }
}

/// i16バッファに無音（0）を書き込む
pub fn write_silence_i16(buffer: &mut [i16], _channels: u16) {
    for sample in buffer.iter_mut() {
        *sample = 0;
    }
}

/// オーディオストリームを構築する関数
/// コンテキストが存在し再生中の場合はAudioBufferの内容を再生し、それ以外は無音を出力します。
pub fn build_output_stream(
    device: &Device,
    config: &StreamConfig,
    sample_format: SampleFormat,
    mut context: Option<PlaybackContext>,
) -> Result<Stream, StreamBuildError> {
    let err_fn = |_err| {
        // リアルタイムスレッドでのロギング(アロケーションやI/O)は避ける
    };

    let channels = config.channels;

    let stream = match sample_format {
        SampleFormat::F32 => {
            let mut active_notes = [0.0; crate::engine::channel::MAX_ACTIVE_NOTES];
            let mut active_note_count = 0;
            let mut track_oscillator = crate::engine::synth::Oscillator::new(config.sample_rate as f32);
            device
                .build_output_stream(
                    config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        if let Some(ctx) = context.as_mut() {
                            let channels_opt = ctx.channels.as_mut();
                            if let Some(channels) = channels_opt {
                                while let Some(msg) = channels.0.try_pop() {
                                    match msg {
                                        UiToAudioMsg::SetPlaying(playing) => {
                                            ctx.playing.store(playing, Ordering::Relaxed)
                                        }
                                        UiToAudioMsg::ActiveNotes(_id, notes, count) => {
                                            active_notes = notes;
                                            active_note_count = count;
                                        }
                                        UiToAudioMsg::UpdateSynthParams(_id, waveform, params) => {
                                            track_oscillator.waveform = waveform;
                                            track_oscillator.envelope.params = params;
                                        }
                                    }
                                }
                            }
                        }

                        let mut handled = false;
                        if let Some(ctx) = context
                            .as_ref()
                            .filter(|c| c.playing.load(Ordering::Relaxed))
                        {
                            let mut pos = ctx.position.load(Ordering::Relaxed);

                            let remaining_samples = ctx.buffer.samples.len().saturating_sub(pos);
                            let samples_to_read = remaining_samples.min(data.len());

                            #[allow(unused_mut)]
                            let mut track_data = crate::engine::mixer::TrackMixData {
                                samples: &ctx.buffer.samples[pos..pos + samples_to_read],
                                channels: ctx.buffer.channels,
                                volume: 1.0,
                                pan: 0.0,
                                is_muted: false,
                                is_solo: false,
                                active_notes,
                                active_note_count,
                                effects: &mut [],
                                oscillator: Some(&mut track_oscillator),
                            };

                            crate::engine::mixer::mix_tracks(data, channels, &mut [track_data]);
                            pos += samples_to_read;
                            ctx.position.store(pos, Ordering::Relaxed);
                            handled = true;
                        }
                        if !handled {
                            write_silence_f32(data, channels);
                        }

                        if let Some(ctx) = context.as_mut() {
                            let channels_opt = ctx.channels.as_mut();
                            if let Some(channels) = channels_opt {
                                let _ = channels.1.try_push(AudioToUiMsg::PlaybackPosition(
                                    ctx.position.load(Ordering::Relaxed) as f32,
                                ));
                            }
                        }
                    },
                    err_fn,
                    None, // タイムアウトなし
                )
                .map_err(StreamBuildError::CpalError)?
        }
        SampleFormat::I16 => {
            let mut active_notes = [0.0; crate::engine::channel::MAX_ACTIVE_NOTES];
            let mut active_note_count = 0;
            let mut track_oscillator = crate::engine::synth::Oscillator::new(config.sample_rate as f32);
            let mut mix_buf = vec![0.0; MIX_BUFFER_SIZE]; // 事前確保しておくミキシング用バッファ
            device
                .build_output_stream(
                    config,
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        if let Some(ctx) = context.as_mut() {
                            let channels_opt = ctx.channels.as_mut();
                            if let Some(channels) = channels_opt {
                                while let Some(msg) = channels.0.try_pop() {
                                    match msg {
                                        UiToAudioMsg::SetPlaying(playing) => {
                                            ctx.playing.store(playing, Ordering::Relaxed)
                                        }
                                        UiToAudioMsg::ActiveNotes(_id, notes, count) => {
                                            active_notes = notes;
                                            active_note_count = count;
                                        }
                                        UiToAudioMsg::UpdateSynthParams(_id, waveform, params) => {
                                            track_oscillator.waveform = waveform;
                                            track_oscillator.envelope.params = params;
                                        }
                                    }
                                }
                            }
                        }

                        let mut handled = false;
                        if let Some(ctx) = context
                            .as_ref()
                            .filter(|c| c.playing.load(Ordering::Relaxed))
                        {
                            let mut current_pos = ctx.position.load(Ordering::Relaxed);
                            let mut remaining_samples =
                                ctx.buffer.samples.len().saturating_sub(current_pos);

                            for chunk in data.chunks_mut(mix_buf.len()) {
                                let chunk_len = chunk.len();
                                let samples_to_read = remaining_samples.min(chunk_len);
                                let mix_slice = &mut mix_buf[..chunk_len];

                                #[allow(unused_mut)]
                                let mut track_data = crate::engine::mixer::TrackMixData {
                                    samples: &ctx.buffer.samples
                                        [current_pos..current_pos + samples_to_read],
                                    channels: ctx.buffer.channels,
                                    volume: 1.0,
                                    pan: 0.0,
                                    is_muted: false,
                                    is_solo: false,
                                    active_notes,
                                    active_note_count,
                                    effects: &mut [],
                                    oscillator: Some(&mut track_oscillator),
                                };

                                crate::engine::mixer::mix_tracks(
                                    mix_slice,
                                    channels,
                                    &mut [track_data],
                                );

                                for (i, &f_sample) in mix_slice.iter().enumerate() {
                                    chunk[i] = (f_sample * i16::MAX as f32) as i16;
                                }

                                current_pos += samples_to_read;
                                remaining_samples =
                                    remaining_samples.saturating_sub(samples_to_read);
                            }

                            ctx.position.store(current_pos, Ordering::Relaxed);
                            handled = true;
                        }
                        if !handled {
                            write_silence_i16(data, channels);
                        }

                        if let Some(ctx) = context.as_mut() {
                            let channels_opt = ctx.channels.as_mut();
                            if let Some(channels) = channels_opt {
                                let _ = channels.1.try_push(AudioToUiMsg::PlaybackPosition(
                                    ctx.position.load(Ordering::Relaxed) as f32,
                                ));
                            }
                        }
                    },
                    err_fn,
                    None, // タイムアウトなし
                )
                .map_err(StreamBuildError::CpalError)?
        }
        _ => return Err(StreamBuildError::UnsupportedFormat(sample_format)),
    };

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_silence_f32() {
        let mut buffer = vec![1.0, -1.0, 0.5, -0.5];
        write_silence_f32(&mut buffer, 2);
        for &sample in &buffer {
            assert_eq!(sample, 0.0);
        }
    }

    #[test]
    fn test_write_silence_i16() {
        let mut buffer = vec![32767, -32768, 16384, -16384];
        write_silence_i16(&mut buffer, 2);
        for &sample in &buffer {
            assert_eq!(sample, 0);
        }
    }

    // Since build_output_stream requires a real audio device and might fail on CI,
    // we don't call it directly. Instead we test the playback logic which is simple enough,
    // but we can at least test the context setup here.
    #[test]
    fn test_playback_context_creation() {
        let buffer = AudioBuffer {
            samples: vec![0.1, 0.2, 0.3, 0.4],
            sample_rate: 44100,
            channels: 2,
        };

        let context = PlaybackContext {
            buffer,
            position: Arc::new(AtomicUsize::new(0)),
            playing: Arc::new(AtomicBool::new(false)),
            channels: None,
        };

        assert_eq!(context.buffer.samples.len(), 4);
        assert_eq!(context.position.load(Ordering::SeqCst), 0);
        assert!(!context.playing.load(Ordering::SeqCst));

        context.playing.store(true, Ordering::SeqCst);
        context.position.store(2, Ordering::SeqCst);

        assert!(context.playing.load(Ordering::SeqCst));
        assert_eq!(context.position.load(Ordering::SeqCst), 2);
    }
}
