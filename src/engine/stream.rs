use cpal::traits::DeviceTrait;
use cpal::{Device, Stream, StreamConfig, SampleFormat};
use crate::engine::audio_file::AudioBuffer;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

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

/// 再生コンテキスト（ロックフリーにオーディオスレッドへ渡す状態）
pub struct PlaybackContext {
    /// オーディオデータ
    pub buffer: AudioBuffer,
    /// 現在の再生位置（サンプル単位）
    pub position: Arc<AtomicUsize>,
    /// 再生中かどうか
    pub playing: Arc<AtomicBool>,
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
    context: Option<PlaybackContext>,
) -> Result<Stream, StreamBuildError> {
    let err_fn = |err| {
        // ロギング(アロケーション等)は避けるか、最小限のコンソール出力に留める
        eprintln!("an error occurred on stream: {}", err);
    };

    let channels = config.channels;

    let stream = match sample_format {
        SampleFormat::F32 => {
            device.build_output_stream(
                config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut handled = false;
                    if let Some(ctx) = context.as_ref().filter(|c| c.playing.load(Ordering::Relaxed)) {
                        let mut pos = ctx.position.load(Ordering::Relaxed);
                        for sample in data.iter_mut() {
                            if pos < ctx.buffer.samples.len() {
                                *sample = ctx.buffer.samples[pos];
                                pos += 1;
                            } else {
                                *sample = 0.0;
                            }
                        }
                        ctx.position.store(pos, Ordering::Relaxed);
                        handled = true;
                    }
                    if !handled {
                        write_silence_f32(data, channels);
                    }
                },
                err_fn,
                None, // タイムアウトなし
            ).map_err(StreamBuildError::CpalError)?
        },
        SampleFormat::I16 => {
            device.build_output_stream(
                config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    let mut handled = false;
                    if let Some(ctx) = context.as_ref().filter(|c| c.playing.load(Ordering::Relaxed)) {
                        let mut pos = ctx.position.load(Ordering::Relaxed);
                        for sample in data.iter_mut() {
                            if pos < ctx.buffer.samples.len() {
                                let s = ctx.buffer.samples[pos];
                                *sample = (s * i16::MAX as f32) as i16;
                                pos += 1;
                            } else {
                                *sample = 0;
                            }
                        }
                        ctx.position.store(pos, Ordering::Relaxed);
                        handled = true;
                    }
                    if !handled {
                        write_silence_i16(data, channels);
                    }
                },
                err_fn,
                None, // タイムアウトなし
            ).map_err(StreamBuildError::CpalError)?
        },
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
