pub mod transcribe;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser, Debug)]
#[command(name = "whisper-rs-cli")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, action)]
    pub silent: bool,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Transcribe(TranscribeArgs),
}

#[derive(Parser, Debug)]
pub struct TranscribeArgs {
    #[arg(value_name = "FILE")]
    pub file_name: String,

    #[arg(short, long, value_name = "MODEL")]
    pub model: Option<String>,

    #[arg(short, long, value_name = "LANG")]
    pub language: Option<String>,

    #[arg(short, long, value_name = "OUTFILE")]
    pub outfile: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_transcribe() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "transcribe", "test.wav"]);
        assert!(args.is_ok());
        let cli = args.unwrap();
        assert!(matches!(cli.command, Commands::Transcribe(_)));
    }

    #[test]
    fn test_cli_parse_with_file() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "transcribe", "audio.mp3"]);
        assert!(args.is_ok());
        let cli = args.unwrap();
        let Commands::Transcribe(transcribe_args) = cli.command else {
            panic!("Expected Transcribe command");
        };
        assert_eq!(transcribe_args.file_name, "audio.mp3");
    }

    #[test]
    fn test_cli_parse_with_model() {
        let args = Cli::try_parse_from([
            "whisper-rs-cli",
            "transcribe",
            "test.wav",
            "--model",
            "tiny",
        ]);
        assert!(args.is_ok());
        let cli = args.unwrap();
        let Commands::Transcribe(transcribe_args) = cli.command else {
            panic!("Expected Transcribe command");
        };
        assert_eq!(transcribe_args.model, Some("tiny".to_string()));
    }

    #[test]
    fn test_cli_parse_with_language() {
        let args = Cli::try_parse_from([
            "whisper-rs-cli",
            "transcribe",
            "test.wav",
            "--language",
            "es",
        ]);
        assert!(args.is_ok());
        let cli = args.unwrap();
        let Commands::Transcribe(transcribe_args) = cli.command else {
            panic!("Expected Transcribe command");
        };
        assert_eq!(transcribe_args.language, Some("es".to_string()));
    }

    #[test]
    fn test_cli_parse_with_outfile() {
        let args = Cli::try_parse_from([
            "whisper-rs-cli",
            "transcribe",
            "test.wav",
            "--outfile",
            "output.txt",
        ]);
        assert!(args.is_ok());
        let cli = args.unwrap();
        let Commands::Transcribe(transcribe_args) = cli.command else {
            panic!("Expected Transcribe command");
        };
        assert_eq!(transcribe_args.outfile, Some("output.txt".to_string()));
    }

    #[test]
    fn test_cli_parse_silent_flag() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "transcribe", "test.wav", "--silent"]);
        assert!(args.is_ok());
        let cli = args.unwrap();
        assert!(cli.silent);
    }

    #[test]
    fn test_cli_parse_multiple_options() {
        let args = Cli::try_parse_from([
            "whisper-rs-cli",
            "transcribe",
            "test.wav",
            "--model",
            "base",
            "--language",
            "en",
            "--outfile",
            "out.txt",
            "--silent",
        ]);
        assert!(args.is_ok());
        let cli = args.unwrap();
        assert!(cli.silent);
        let Commands::Transcribe(transcribe_args) = cli.command else {
            panic!("Expected Transcribe command");
        };
        assert_eq!(transcribe_args.file_name, "test.wav");
        assert_eq!(transcribe_args.model, Some("base".to_string()));
        assert_eq!(transcribe_args.language, Some("en".to_string()));
        assert_eq!(transcribe_args.outfile, Some("out.txt".to_string()));
    }

    #[test]
    fn test_cli_parse_verbose_flag() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "transcribe", "test.wav", "-v"]);
        assert!(args.is_ok());
    }

    #[test]
    fn test_cli_parse_help() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "--help"]);
        assert!(args.is_err());
    }

    #[test]
    fn test_cli_parse_version() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "--version"]);
        assert!(args.is_err());
    }

    #[test]
    fn test_cli_parse_missing_file() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "transcribe"]);
        assert!(args.is_err());
    }

    #[test]
    fn test_cli_parse_invalid_model() {
        let args = Cli::try_parse_from(["whisper-rs-cli", "transcribe", "test.wav", "--model"]);
        assert!(args.is_err());
    }
}
