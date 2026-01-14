use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhisperError {
    #[error("Unsupported file type")]
    UnsupportedFileType,
    #[error("FFmpeg not found")]
    FFmpegNotFound,
    #[error("Audio conversion failed")]
    AudioConversionFailed,
    #[error("Failed to load audio")]
    AudioLoadFailed,
    #[error("Model not found")]
    ModelNotFound,
    #[error("Model download failed")]
    ModelDownloadFailed,
    #[error("Failed to load model")]
    ModelLoadFailed,
    #[error("Transcription failed")]
    TranscriptionFailed,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Whisper error: {0}")]
    WhisperError(#[from] whisper_rs::WhisperError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, WhisperError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            WhisperError::UnsupportedFileType.to_string(),
            "Unsupported file type"
        );
        assert_eq!(WhisperError::FFmpegNotFound.to_string(), "FFmpeg not found");
        assert_eq!(
            WhisperError::AudioConversionFailed.to_string(),
            "Audio conversion failed"
        );
        assert_eq!(
            WhisperError::AudioLoadFailed.to_string(),
            "Failed to load audio"
        );
        assert_eq!(WhisperError::ModelNotFound.to_string(), "Model not found");
        assert_eq!(
            WhisperError::ModelDownloadFailed.to_string(),
            "Model download failed"
        );
        assert_eq!(
            WhisperError::ModelLoadFailed.to_string(),
            "Failed to load model"
        );
        assert_eq!(
            WhisperError::TranscriptionFailed.to_string(),
            "Transcription failed"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let whisper_err: WhisperError = io_err.into();
        assert!(matches!(whisper_err, WhisperError::IoError(_)));
        assert!(whisper_err.to_string().contains("IO error"));
    }

    #[test]
    fn test_anyhow_error_conversion() {
        let anyhow_err = anyhow::anyhow!("test error");
        let whisper_err: WhisperError = anyhow_err.into();
        assert!(matches!(whisper_err, WhisperError::Other(_)));
    }

    #[test]
    fn test_result_type() {
        let ok_result: Result<()> = Ok(());
        assert!(ok_result.is_ok());

        let err_result: Result<()> = Err(WhisperError::UnsupportedFileType);
        assert!(err_result.is_err());
    }
}
