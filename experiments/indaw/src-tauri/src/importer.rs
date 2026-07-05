use std::path::Path;
use std::fs::File;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::audio::SampleBuffer;
use anyhow::{Context, Result};

pub struct LoadedAudio {
    pub samples: Vec<f32>,
    pub channels: u16,
    pub sample_rate: u32,
    pub duration_seconds: f64,
}

pub fn import_audio_file<P: AsRef<Path>>(path: P) -> Result<LoadedAudio> {
    // Open the file
    let src = File::open(&path).context("failed to open audio file")?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create the probe to detect format automatically
    let mut hint = symphonia::core::probe::Hint::new();
    if let Some(ext) = path.as_ref().extension() {
        if let Some(ext_str) = ext.to_str() {
            hint.with_extension(ext_str);
        }
    }

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .context("unsupported format")?;

    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .context("no supported audio tracks")?;

    let dec_opts: DecoderOptions = Default::default();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .context("unsupported codec")?;

    let track_id = track.id;
    let mut samples: Vec<f32> = Vec::new();
    let mut sample_rate = 0;
    let mut channels = 0;

    // Decode loop
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::IoError(_)) => break, // EOF
            Err(symphonia::core::errors::Error::ResetRequired) => {
                // The track list has been changed. Re-instantiate the decoder.
                // For MVP we might error out, but usually fine to just ignore or reset.
                return Err(anyhow::anyhow!("Stream reset required (unsupported for simple import)"));
            }
            Err(err) => return Err(err.into()),
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                if sample_rate == 0 {
                    let spec = decoded.spec();
                    sample_rate = spec.rate;
                    channels = spec.channels.count() as u16;
                }
                
                // Copy samples to f32 buffer
                // SampleBuffer helps convert diverse formats to f32
                let mut buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
                buf.copy_interleaved_ref(decoded);
                
                samples.extend_from_slice(buf.samples());
            }
            Err(symphonia::core::errors::Error::IoError(_)) => break,
            Err(symphonia::core::errors::Error::DecodeError(_)) => (), // ignore decode errors
            Err(err) => return Err(err.into()),
        }
    }

    // Duration calculation
    let duration = if channels > 0 && sample_rate > 0 {
        samples.len() as f64 / channels as f64 / sample_rate as f64
    } else { 0.0 };
    
    // --- RESAMPLING (Rubato) ---
    // Target sample rate: 44100 (Standard)
    let target_rate = 44100;
    
    if sample_rate != target_rate && sample_rate > 0 {
        use rubato::{Resampler, SincFixedIn, InterpolationType, WindowFunction, InterpolationParameters};
        
        // De-interleave for rubato
        let mut planar_input: Vec<Vec<f32>> = vec![Vec::with_capacity(samples.len() / channels as usize); channels as usize];
        for (i, sample) in samples.iter().enumerate() {
            planar_input[i % channels as usize].push(*sample);
        }
        
        // Create Resampler
        let params = InterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: InterpolationType::Linear, // Use Linear for speed or Cubic? Rubato Sinc is high quality.
            oversampling_factor: 128,
            window: WindowFunction::BlackmanHarris2,
        };
        
        // SincFixedIn is good for whole-file resampling
        let mut resampler = SincFixedIn::<f32>::new(
            target_rate as f64 / sample_rate as f64,
            2.0, // Max ratio
            params,
            planar_input[0].len(), // Input chunk size (whole file)
            channels as usize
        ).context("failed to create resampler")?;
        
        let waves_in = planar_input;
        let waves_out = resampler.process(&waves_in, None).context("resampling failed")?;
        
        // Interleave back
        let new_len = waves_out[0].len() * channels as usize;
        let mut new_samples = Vec::with_capacity(new_len);
        for i in 0..waves_out[0].len() {
             for ch in 0..channels as usize {
                 if i < waves_out[ch].len() {
                     new_samples.push(waves_out[ch][i]);
                 }
             }
        }
        
        return Ok(LoadedAudio {
            samples: new_samples,
            channels,
            sample_rate: target_rate,
            duration_seconds: duration, // Duration matches (approx time same)
        });
    }

    Ok(LoadedAudio {
        samples,
        channels,
        sample_rate,
        duration_seconds: duration,
    })
}
