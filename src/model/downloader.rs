use crate::error::{Result, WhisperError};
use crate::log_info;
use crate::output::logger::is_silent;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

const MODEL_BASE_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";

pub fn download_model(model_name: &str, output_path: &Path) -> Result<()> {
    let url = format!("{}/ggml-{}.bin", MODEL_BASE_URL, model_name);

    log_info!("Downloading model from {}...", url);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(600))
        .build()?;

    let response = client.get(&url).send()?;
    let total_size = response.content_length().unwrap_or(0);

    let mut file = File::create(output_path)?;

    if !is_silent() && total_size > 0 {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        let mut downloaded = 0u64;
        let mut source = response;
        let mut buffer = [0u8; 8192];

        loop {
            let bytes_read = source
                .read(&mut buffer)
                .map_err(|_| WhisperError::ModelDownloadFailed)?;
            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read])?;
            downloaded += bytes_read as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");
    } else {
        let mut source = response;
        std::io::copy(&mut source, &mut file)?;
        log_info!("Download complete");
    }

    Ok(())
}

pub fn generate_url(model_name: &str) -> String {
    format!("{}/ggml-{}.bin", MODEL_BASE_URL, model_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_url() {
        let url = generate_url("base");
        assert_eq!(
            url,
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin"
        );

        let url = generate_url("tiny");
        assert_eq!(
            url,
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin"
        );

        let url = generate_url("small");
        assert_eq!(
            url,
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin"
        );
    }

    #[test]
    fn test_generate_url_various_models() {
        let models = vec![
            "tiny", "base", "small", "medium", "large", "large-v1", "large-v2", "large-v3",
        ];
        for model in models {
            let url = generate_url(model);
            assert!(url.contains(model));
            assert!(url.starts_with("https://huggingface.co"));
        }
    }

    #[test]
    fn test_model_base_url() {
        assert_eq!(
            MODEL_BASE_URL,
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main"
        );
    }
}
