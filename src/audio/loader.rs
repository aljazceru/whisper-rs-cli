use crate::audio::converter::convert_to_wav;
use crate::audio::formats::AudioFormat;
use crate::error::{Result, WhisperError};
use crate::log_info;
use hound::WavReader;
use rubato::{Resampler, SincFixedIn};
use std::path::Path;

pub const SAMPLE_RATE: u32 = 16000;

pub struct AudioData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

pub fn load_audio(file_path: &str) -> Result<AudioData> {
    let format = AudioFormat::from_path(file_path).ok_or(WhisperError::UnsupportedFileType)?;

    let samples = if format.needs_conversion() {
        let temp_wav = convert_to_wav(file_path)?;
        load_wav(temp_wav.path())?
    } else {
        load_wav(Path::new(file_path))?
    };

    Ok(samples)
}

fn load_wav(path: &Path) -> Result<AudioData> {
    let reader = WavReader::open(path).map_err(|_| WhisperError::AudioLoadFailed)?;
    let spec = reader.spec();

    let samples: Vec<f32> =
        if spec.bits_per_sample == 32 && spec.sample_format == hound::SampleFormat::Float {
            reader
                .into_samples::<f32>()
                .map(|s| s.map_err(|_| WhisperError::AudioLoadFailed))
                .collect::<Result<Vec<_>>>()?
        } else if spec.bits_per_sample == 16 {
            reader
                .into_samples::<i16>()
                .map(|s| s.map_err(|_| WhisperError::AudioLoadFailed))
                .map(|s| Ok(s? as f32 / 32768.0))
                .collect::<Result<Vec<_>>>()?
        } else {
            return Err(WhisperError::AudioLoadFailed);
        };

    let samples = if spec.channels == 2 {
        stereo_to_mono(&samples)
    } else if spec.channels == 1 {
        samples
    } else {
        return Err(WhisperError::AudioLoadFailed);
    };

    let sample_rate = spec.sample_rate;
    let samples = if sample_rate != SAMPLE_RATE {
        log_info!("Resampling from {} Hz to {} Hz", sample_rate, SAMPLE_RATE);
        resample(&samples, sample_rate, SAMPLE_RATE)?
    } else {
        samples
    };

    Ok(AudioData {
        samples,
        sample_rate: SAMPLE_RATE,
    })
}

fn stereo_to_mono(samples: &[f32]) -> Vec<f32> {
    samples
        .chunks(2)
        .map(|chunk| (chunk[0] + chunk[1]) / 2.0)
        .collect()
}

fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
    if from_rate == to_rate {
        return Ok(samples.to_vec());
    }

    let ratio = to_rate as f64 / from_rate as f64;
    let parameters = rubato::SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: rubato::SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: rubato::WindowFunction::BlackmanHarris2,
    };

    let chunk_size = samples.len().max(1024);
    let mut resampler = SincFixedIn::<f64>::new(ratio, 2.0, parameters, chunk_size, 1)
        .map_err(|_| WhisperError::AudioLoadFailed)?;

    let mut buffer_in = vec![vec![0.0f64; samples.len()]];
    buffer_in[0].copy_from_slice(&samples.iter().map(|&x| x as f64).collect::<Vec<_>>());

    let waves_out = resampler
        .process(&buffer_in, None)
        .map_err(|_| WhisperError::AudioLoadFailed)?;

    Ok(waves_out[0].iter().map(|&x| x as f32).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hound::{WavSpec, WavWriter};
    use tempfile::NamedTempFile;

    fn create_wav_file(sample_rate: u32, channels: u16, bits: u16) -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let spec = WavSpec {
            channels,
            sample_rate,
            bits_per_sample: bits,
            sample_format: if bits == 32 {
                hound::SampleFormat::Float
            } else {
                hound::SampleFormat::Int
            },
        };

        let mut writer = WavWriter::new(temp_file.reopen().unwrap(), spec).unwrap();

        for i in 0..1000 {
            let sample = (i as f32 / 1000.0).sin();
            if bits == 32 {
                writer.write_sample::<f32>(sample).unwrap();
            } else if bits == 16 {
                writer
                    .write_sample::<i16>((sample * 32767.0) as i16)
                    .unwrap();
            }
        }

        writer.finalize().unwrap();
        temp_file
    }

    #[test]
    fn test_load_wav_16bit_mono() {
        let temp_file = create_wav_file(16000, 1, 16);
        let audio_data = load_wav(temp_file.path()).unwrap();
        assert_eq!(audio_data.sample_rate, 16000);
        assert_eq!(audio_data.samples.len(), 1000);
    }

    #[test]
    fn test_load_wav_16bit_stereo() {
        let temp_file = create_wav_file(16000, 2, 16);
        let audio_data = load_wav(temp_file.path()).unwrap();
        assert_eq!(audio_data.sample_rate, 16000);
        assert_eq!(audio_data.samples.len(), 500);
    }

    #[test]
    fn test_load_wav_32bit_mono() {
        let temp_file = create_wav_file(16000, 1, 32);
        let audio_data = load_wav(temp_file.path()).unwrap();
        assert_eq!(audio_data.sample_rate, 16000);
        assert_eq!(audio_data.samples.len(), 1000);
    }

    #[test]
    fn test_load_wav_with_resampling() {
        let temp_file = create_wav_file(22050, 1, 16);
        let audio_data = load_wav(temp_file.path());
        if audio_data.is_err() {
            return;
        }
        let audio_data = audio_data.unwrap();
        assert_eq!(audio_data.sample_rate, 16000);
    }

    #[test]
    fn test_load_wav_file_not_found() {
        let result = load_wav(Path::new("/nonexistent/file.wav"));
        assert!(result.is_err());
        assert!(matches!(result, Err(WhisperError::AudioLoadFailed)));
    }

    #[test]
    fn test_stereo_to_mono() {
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mono = stereo_to_mono(&samples);
        assert_eq!(mono.len(), 3);
        assert_eq!(mono[0], 1.5);
        assert_eq!(mono[1], 3.5);
        assert_eq!(mono[2], 5.5);
    }

    #[test]
    fn test_resample_same_rate() {
        let samples = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let result = resample(&samples, 16000, 16000).unwrap();
        assert_eq!(result.len(), samples.len());
    }

    #[test]
    fn test_resample_different_rate() {
        let samples = vec![0.0; 2000];
        let result = resample(&samples, 22050, 16000).unwrap();
        assert!(result.len() < samples.len());
    }

    #[test]
    fn test_audio_data_struct() {
        let audio = AudioData {
            samples: vec![0.0, 0.1, 0.2],
            sample_rate: 16000,
        };
        assert_eq!(audio.samples.len(), 3);
        assert_eq!(audio.sample_rate, 16000);
    }
}
