#![allow(deprecated)]
use assert_cmd::Command;
use assert_fs::prelude::*;
use hound::{WavSpec, WavWriter};
use predicates::prelude::*;
use tempfile::NamedTempFile;

fn create_test_wav() -> NamedTempFile {
    let temp_file = NamedTempFile::with_suffix(".wav").unwrap();
    let spec = WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::new(temp_file.reopen().unwrap(), spec).unwrap();

    for i in 0..48000 {
        let sample = ((i as f32 / 48000.0).sin() * 32767.0) as i16;
        writer.write_sample::<i16>(sample).unwrap();
    }

    writer.finalize().unwrap();
    temp_file
}

fn create_test_file_unsupported(extension: &str) -> NamedTempFile {
    let temp_file = NamedTempFile::with_suffix(extension).unwrap();
    std::fs::write(temp_file.path(), b"not an audio file").unwrap();
    temp_file
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("whisper-rs-cli"))
        .stdout(predicate::str::contains("transcribe"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("whisper-rs-cli"));
}

#[test]
fn test_cli_missing_file() {
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe");
    cmd.assert().failure();
}

#[test]
fn test_silent_mode() {
    let test_wav = create_test_wav();
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let output_path = temp_dir.child("output.txt");

    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg(test_wav.path())
        .arg("--outfile")
        .arg(output_path.path());
    cmd.assert().success().stdout(predicate::str::is_empty());
}

#[test]
fn test_model_option() {
    let test_wav = create_test_wav();
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg(test_wav.path())
        .arg("--model")
        .arg("tiny");
    cmd.assert().success();
}

#[test]
fn test_language_option() {
    let test_wav = create_test_wav();
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg(test_wav.path())
        .arg("--language")
        .arg("en");
    cmd.assert().success();
}

#[test]
fn test_outfile_option() {
    let test_wav = create_test_wav();
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let output_path = temp_dir.child("output.txt");

    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg(test_wav.path())
        .arg("--outfile")
        .arg(output_path.path());

    cmd.assert().success();
}

#[test]
fn test_multiple_options() {
    let test_wav = create_test_wav();
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg(test_wav.path())
        .arg("--model")
        .arg("base")
        .arg("--language")
        .arg("en");
    cmd.assert().success();
}

#[test]
fn test_unsupported_format_txt() {
    let test_file = create_test_file_unsupported(".txt");
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe").arg(test_file.path());
    cmd.assert().failure();
}

#[test]
fn test_unsupported_format_pdf() {
    let test_file = create_test_file_unsupported(".pdf");
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe").arg(test_file.path());
    cmd.assert().failure();
}

#[test]
fn test_file_not_found() {
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg("/nonexistent/file.wav");
    cmd.assert().failure();
}

#[test]
fn test_invalid_model_option() {
    let test_wav = create_test_wav();
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe").arg(test_wav.path()).arg("--model");
    cmd.assert().failure();
}

#[test]
fn test_invalid_language_option() {
    let test_wav = create_test_wav();
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe").arg(test_wav.path()).arg("--language");
    cmd.assert().failure();
}

#[test]
fn test_wav_file_extension() {
    let test_wav = create_test_wav();
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe").arg(test_wav.path());
    cmd.assert().success();
}

#[test]
fn test_empty_file() {
    let temp_file = NamedTempFile::with_suffix(".wav").unwrap();
    std::fs::write(temp_file.path(), b"").unwrap();

    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe").arg(temp_file.path());
    cmd.assert().failure();
}

#[test]
fn test_silent_mode_no_stderr_logs() {
    let test_wav = create_test_wav();

    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg(test_wav.path());

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("whisper_").not());
}

#[test]
fn test_debug_mode_has_stderr_logs() {
    let test_wav = create_test_wav();

    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe")
        .arg(test_wav.path())
        .arg("--debug");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("whisper_"));
}

#[test]
fn test_cli_command_structure() {
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.arg("transcribe");
    cmd.assert().failure();
}

#[test]
fn test_no_command() {
    let mut cmd = Command::cargo_bin("whisper-rs-cli").unwrap();
    cmd.assert().failure();
}
