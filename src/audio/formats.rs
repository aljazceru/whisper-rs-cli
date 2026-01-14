#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Webm,
    Mp3,
    M4a,
    Mp4,
    Ogg,
    Flac,
    Aac,
}

impl AudioFormat {
    pub fn from_path(path: &str) -> Option<Self> {
        let extension = path.rsplit('.').next()?.to_lowercase();
        match extension.as_str() {
            "wav" => Some(AudioFormat::Wav),
            "wave" => Some(AudioFormat::Wav),
            "webm" => Some(AudioFormat::Webm),
            "mp3" => Some(AudioFormat::Mp3),
            "m4a" => Some(AudioFormat::M4a),
            "mp4" => Some(AudioFormat::Mp4),
            "ogg" => Some(AudioFormat::Ogg),
            "flac" => Some(AudioFormat::Flac),
            "aac" => Some(AudioFormat::Aac),
            _ => None,
        }
    }

    pub fn needs_conversion(&self) -> bool {
        !matches!(self, AudioFormat::Wav)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection_wav() {
        assert_eq!(AudioFormat::from_path("test.wav"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_path("test.WAV"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_path("test.Wave"), Some(AudioFormat::Wav));
        assert_eq!(AudioFormat::from_path("test.WAVE"), Some(AudioFormat::Wav));
    }

    #[test]
    fn test_format_detection_webm() {
        assert_eq!(AudioFormat::from_path("test.webm"), Some(AudioFormat::Webm));
        assert_eq!(AudioFormat::from_path("test.WEBM"), Some(AudioFormat::Webm));
    }

    #[test]
    fn test_format_detection_mp3() {
        assert_eq!(AudioFormat::from_path("test.mp3"), Some(AudioFormat::Mp3));
        assert_eq!(AudioFormat::from_path("test.MP3"), Some(AudioFormat::Mp3));
    }

    #[test]
    fn test_format_detection_m4a() {
        assert_eq!(AudioFormat::from_path("test.m4a"), Some(AudioFormat::M4a));
        assert_eq!(AudioFormat::from_path("test.M4A"), Some(AudioFormat::M4a));
    }

    #[test]
    fn test_format_detection_mp4() {
        assert_eq!(AudioFormat::from_path("test.mp4"), Some(AudioFormat::Mp4));
        assert_eq!(AudioFormat::from_path("test.MP4"), Some(AudioFormat::Mp4));
    }

    #[test]
    fn test_format_detection_ogg() {
        assert_eq!(AudioFormat::from_path("test.ogg"), Some(AudioFormat::Ogg));
        assert_eq!(AudioFormat::from_path("test.OGG"), Some(AudioFormat::Ogg));
    }

    #[test]
    fn test_format_detection_flac() {
        assert_eq!(AudioFormat::from_path("test.flac"), Some(AudioFormat::Flac));
        assert_eq!(AudioFormat::from_path("test.FLAC"), Some(AudioFormat::Flac));
    }

    #[test]
    fn test_format_detection_aac() {
        assert_eq!(AudioFormat::from_path("test.aac"), Some(AudioFormat::Aac));
        assert_eq!(AudioFormat::from_path("test.AAC"), Some(AudioFormat::Aac));
    }

    #[test]
    fn test_format_detection_unsupported() {
        assert_eq!(AudioFormat::from_path("test.txt"), None);
        assert_eq!(AudioFormat::from_path("test.pdf"), None);
        assert_eq!(AudioFormat::from_path("test"), None);
    }

    #[test]
    fn test_needs_conversion() {
        assert!(!AudioFormat::Wav.needs_conversion());
        assert!(AudioFormat::Webm.needs_conversion());
        assert!(AudioFormat::Mp3.needs_conversion());
        assert!(AudioFormat::M4a.needs_conversion());
        assert!(AudioFormat::Mp4.needs_conversion());
        assert!(AudioFormat::Ogg.needs_conversion());
        assert!(AudioFormat::Flac.needs_conversion());
        assert!(AudioFormat::Aac.needs_conversion());
    }
}
