use cpal::traits::DeviceTrait;
use cpal::{Device, Stream, StreamConfig, SampleFormat};

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

/// オーディオストリームを構築するスケルトン関数
/// リアルタイム制約に従い、アロケーション・ロック・I/Oを回避してダミーの無音出力を実装します。
pub fn build_output_stream(device: &Device, config: &StreamConfig, sample_format: SampleFormat) -> Result<Stream, StreamBuildError> {
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
                    write_silence_f32(data, channels);
                },
                err_fn,
                None, // タイムアウトなし
            ).map_err(StreamBuildError::CpalError)?
        },
        SampleFormat::I16 => {
            device.build_output_stream(
                config,
                move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    write_silence_i16(data, channels);
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
}
