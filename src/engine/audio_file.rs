//! オーディオファイル読み込みモジュール
//!
//! `hound` クレートを使用してWAVファイルを読み込み、
//! 内部のオーディオ処理で扱いやすい `f32` のバッファに変換します。

use std::io::{Read, Seek};
use std::path::Path;

/// WAVファイルから読み込んだオーディオデータを保持する構造体
#[derive(Debug, Clone, PartialEq)]
pub struct AudioBuffer {
    /// チャンネルごとのインターリーブされたオーディオサンプル (-1.0 〜 1.0)
    pub samples: Vec<f32>,
    /// サンプルレート (例: 44100)
    pub sample_rate: u32,
    /// チャンネル数 (1: モノラル, 2: ステレオ)
    pub channels: u16,
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioBuffer {
    /// 空のバッファを作成する
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            sample_rate: 44100,
            channels: 2,
        }
    }
}

/// 指定したパスのWAVファイルを読み込み、正規化された`AudioBuffer`を返す
pub fn load_wav<P: AsRef<Path>>(path: P) -> Result<AudioBuffer, String> {
    let reader = hound::WavReader::new(std::fs::File::open(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    process_wav_reader(reader)
}

/// 任意のRead + Seek実装からWAVデータを読み込み、正規化された`AudioBuffer`を返す
pub fn load_wav_from_reader<R: Read + Seek>(reader: R) -> Result<AudioBuffer, String> {
    let wav_reader = hound::WavReader::new(reader).map_err(|e| e.to_string())?;
    process_wav_reader(wav_reader)
}

/// WavReaderからサンプルを読み込み、-1.0 〜 1.0 に正規化してAudioBufferを生成する
fn process_wav_reader<R: Read + Seek>(
    mut reader: hound::WavReader<R>,
) -> Result<AudioBuffer, String> {
    let spec = reader.spec();
    let sample_rate = spec.sample_rate;
    let channels = spec.channels;
    let duration = reader.duration() as usize;
    let total_samples = duration * channels as usize;

    let mut samples = Vec::with_capacity(total_samples);

    match spec.sample_format {
        hound::SampleFormat::Int => {
            let max_val = match spec.bits_per_sample {
                16 => i16::MAX as f32,
                24 => 8_388_607.0, // 2^23 - 1
                32 => i32::MAX as f32,
                8 => 128.0, // 8-bit is unsigned, but hound returns it as i32 shifted. Actually 8-bit WAV is unsigned (0-255).
                _ => {
                    return Err(format!(
                        "Unsupported integer bit depth: {}",
                        spec.bits_per_sample
                    ));
                }
            };

            if spec.bits_per_sample == 8 {
                // 8-bit WAV is unsigned, hound handles it by returning i32.
                // Wait, hound returns i32 for all int formats up to 32 bits.
                for sample in reader.samples::<i32>() {
                    let s = sample.map_err(|e| e.to_string())?;
                    // 8-bit is 0..255, but hound converts it to i32.
                    // Wait, hound's documentation says: for 8-bit, it returns the value directly (0..255).
                    // We need to normalize 0..255 to -1.0..1.0
                    samples.push((s as f32 - 128.0) / 128.0);
                }
            } else {
                for sample in reader.samples::<i32>() {
                    let s = sample.map_err(|e| e.to_string())?;
                    samples.push(s as f32 / max_val);
                }
            }
        }
        hound::SampleFormat::Float => {
            if spec.bits_per_sample != 32 {
                return Err(format!(
                    "Unsupported float bit depth: {}",
                    spec.bits_per_sample
                ));
            }
            for sample in reader.samples::<f32>() {
                let s = sample.map_err(|e| e.to_string())?;
                samples.push(s);
            }
        }
    }

    Ok(AudioBuffer {
        samples,
        sample_rate,
        channels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_audio_buffer_new() {
        let buf = AudioBuffer::new();
        assert!(buf.samples.is_empty());
        assert_eq!(buf.sample_rate, 44100);
        assert_eq!(buf.channels, 2);
    }

    #[test]
    fn test_load_wav_16bit() -> Result<(), Box<dyn std::error::Error>> {
        // 16bitのダミーWAVデータを作成
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut cursor = Cursor::new(Vec::new());
        {
            let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
            writer.write_sample(0i16)?;
            writer.write_sample(std::i16::MAX)?;
            writer.write_sample(std::i16::MIN)?;
            writer.finalize()?;
        }

        cursor.set_position(0);

        // 読み込んで確認
        let buffer = load_wav_from_reader(cursor)?;
        assert_eq!(buffer.channels, 1);
        assert_eq!(buffer.sample_rate, 44100);
        assert_eq!(buffer.samples.len(), 3);

        assert_eq!(buffer.samples[0], 0.0);
        assert_eq!(buffer.samples[1], 1.0);
        assert!((buffer.samples[2] - (-1.0000305)).abs() < 1e-6); // MIN / MAX => -32768 / 32767 = -1.0000305
        Ok(())
    }

    #[test]
    fn test_load_wav_float() -> Result<(), Box<dyn std::error::Error>> {
        // 32bit FloatのダミーWAVデータを作成
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 48000,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut cursor = Cursor::new(Vec::new());
        {
            let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
            writer.write_sample(0.0f32)?;
            writer.write_sample(1.0f32)?;
            writer.write_sample(-0.5f32)?;
            writer.write_sample(0.5f32)?;
            writer.finalize()?;
        }

        cursor.set_position(0);

        // 読み込んで確認
        let buffer = load_wav_from_reader(cursor)?;
        assert_eq!(buffer.channels, 2);
        assert_eq!(buffer.sample_rate, 48000);
        assert_eq!(buffer.samples.len(), 4);

        assert_eq!(buffer.samples[0], 0.0);
        assert_eq!(buffer.samples[1], 1.0);
        assert_eq!(buffer.samples[2], -0.5);
        assert_eq!(buffer.samples[3], 0.5);
        Ok(())
    }
}
