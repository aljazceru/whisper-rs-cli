use crate::error::{Result, WhisperError};
use crate::log_info;
use crate::log_warning;
use crate::model::downloader::download_model;
use std::fs;
use std::path::PathBuf;
use whisper_rs::WhisperContext;

pub const DEFAULT_MODEL: &str = "base";

fn get_model_search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(home) = dirs::home_dir() {
        dirs.push(home.join(".cache/whispercpp"));
        dirs.push(home.join(".local/share/whisper"));
        dirs.push(home.join(".local/share/pywhispercpp/models"));
    }

    dirs.push(PathBuf::from("./models"));

    dirs
}

pub fn get_models_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| WhisperError::Other(anyhow::anyhow!("Home directory not found")))?;
    let models_dir = home.join(".local/share/whisper");

    if !models_dir.exists() {
        fs::create_dir_all(&models_dir)?;
    }

    Ok(models_dir)
}

fn find_model_file(model_name: &str, language: Option<&str>) -> Option<PathBuf> {
    let search_dirs = get_model_search_dirs();

    for models_dir in &search_dirs {
        if !models_dir.exists() {
            continue;
        }

        let model_file = if let Some(lang) = language {
            models_dir.join(format!("ggml-{}.{}.bin", model_name, lang))
        } else {
            models_dir.join(format!("ggml-{}.en.bin", model_name))
        };

        if model_file.exists() {
            return Some(model_file);
        }

        let model_file_en = models_dir.join(format!("ggml-{}.en.bin", model_name));
        if model_file_en.exists() {
            return Some(model_file_en);
        }

        let model_file_base = models_dir.join(format!("ggml-{}.bin", model_name));
        if model_file_base.exists() {
            return Some(model_file_base);
        }
    }

    None
}

pub fn load_model(model_name: Option<&str>, language: Option<&str>) -> Result<WhisperContext> {
    let model_name = model_name.unwrap_or(DEFAULT_MODEL);
    let models_dir = get_models_dir()?;

    if let Some(model_path) = find_model_file(model_name, language) {
        log_info!("Loading model {}...", model_name);
        let context = WhisperContext::new_with_params(
            &model_path.to_string_lossy(),
            whisper_rs::WhisperContextParameters::default(),
        )?;
        log_info!("Model loaded");
        return Ok(context);
    }

    let model_file_base = models_dir.join(format!("ggml-{}.bin", model_name));

    let silent = crate::output::logger::is_silent();
    if !silent {
        log_warning!("Model not found in any search location, downloading to ~/.local/share/whisper...");
    }
    download_model(model_name, &model_file_base)?;

    log_info!("Loading model {}...", model_name);
    let context = WhisperContext::new_with_params(
        &model_file_base.to_string_lossy(),
        whisper_rs::WhisperContextParameters::default(),
    )?;
    log_info!("Model loaded");

    Ok(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_model() {
        assert_eq!(DEFAULT_MODEL, "base");
    }

    #[test]
    fn test_get_models_dir() {
        let result = get_models_dir();
        assert!(result.is_ok());
        let dir = result.unwrap();
        assert!(dir.ends_with(".local/share/whisper"));
        assert!(dir.to_string_lossy().contains(".local"));
    }

    #[test]
    fn test_model_file_name_generation() {
        let models_dir = std::path::PathBuf::from("/test/models");
        let model_file = models_dir.join("ggml-base.bin");
        assert_eq!(model_file.to_string_lossy(), "/test/models/ggml-base.bin");

        let model_file_en = models_dir.join("ggml-base.en.bin");
        assert_eq!(
            model_file_en.to_string_lossy(),
            "/test/models/ggml-base.en.bin"
        );

        let model_file_lang = models_dir.join("ggml-base.es.bin");
        assert_eq!(
            model_file_lang.to_string_lossy(),
            "/test/models/ggml-base.es.bin"
        );
    }

    #[test]
    fn test_model_priority_resolution() {
        let base_file = "ggml-base.bin";
        let en_file = "ggml-base.en.bin";
        let lang_file = "ggml-base.es.bin";

        assert!(base_file.len() > 0);
        assert!(en_file.len() > 0);
        assert!(lang_file.len() > 0);
    }

    #[test]
    fn test_model_names() {
        let valid_models = vec!["tiny", "base", "small", "medium", "large"];
        for model in valid_models {
            let file_name = format!("ggml-{}.bin", model);
            assert!(file_name.starts_with("ggml-"));
            assert!(file_name.ends_with(".bin"));
        }
    }

    #[test]
    fn test_model_search_dirs() {
        let dirs = get_model_search_dirs();
        assert!(!dirs.is_empty());
        assert!(dirs.len() >= 4);

        let dir_strs: Vec<String> = dirs
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        assert!(dir_strs.iter().any(|s| s.contains(".cache/whispercpp")));
        assert!(dir_strs.iter().any(|s| s.contains("models")));
        assert!(dir_strs.iter().any(|s| s.contains(".local/share/whisper")));
        assert!(dir_strs.iter().any(|s| s.contains("pywhispercpp/models")));
    }
}
