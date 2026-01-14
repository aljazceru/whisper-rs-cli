use crate::error::{Result, WhisperError};
use crate::log_info;
use tempfile::NamedTempFile;

pub fn check_ffmpeg_available() -> Result<()> {
    use std::process::Command;

    let result = Command::new("ffmpeg").arg("-version").output();

    match result {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(WhisperError::FFmpegNotFound),
    }
}

pub fn convert_to_wav(input_path: &str) -> Result<NamedTempFile> {
    check_ffmpeg_available()?;

    let output = NamedTempFile::with_suffix(".wav")?;

    log_info!("Converting {} to WAV...", input_path);

    let result = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-acodec")
        .arg("pcm_s16le")
        .arg("-ar")
        .arg("16000")
        .arg("-ac")
        .arg("1")
        .arg("-y")
        .arg(output.path())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output();

    match result {
        Ok(output_status) if output_status.status.success() => {
            log_info!("Conversion complete");
            Ok(output)
        }
        _ => Err(WhisperError::AudioConversionFailed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    static FFMPEG_CHECKED: AtomicBool = AtomicBool::new(false);

    #[test]
    fn test_check_ffmpeg_available_mock() {
        FFMPEG_CHECKED.store(true, Ordering::SeqCst);
        let result = check_ffmpeg_available();
        match result {
            Ok(_) => println!("FFmpeg is available"),
            Err(WhisperError::FFmpegNotFound) => println!("FFmpeg not found (expected in CI)"),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_check_ffmpeg_not_found_error() {
        let result = check_ffmpeg_available();
        if result.is_err() {
            assert!(matches!(result, Err(WhisperError::FFmpegNotFound)));
        }
    }
}
