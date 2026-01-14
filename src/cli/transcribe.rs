use crate::audio::load_audio;
use crate::cli::TranscribeArgs;
use crate::error::Result;
use crate::log_info;
use crate::log_success;
use crate::model::load_model;
use std::fs::File;
use std::io::Write;

pub fn execute(args: &TranscribeArgs) -> Result<()> {
    log_info!("Loading audio from {}...", args.file_name);
    let audio_data = load_audio(&args.file_name)?;

    log_info!("Loading model...");
    let context = load_model(args.model.as_deref(), args.language.as_deref())?;

    log_info!("Running transcription...");
    let mut state = context.create_state()?;
    let mut full_params =
        whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });

    if let Some(lang) = &args.language {
        full_params.set_language(Some(lang));
    } else {
        full_params.set_language(None);
    }

    full_params.set_n_threads(4);
    full_params.set_print_special(false);
    full_params.set_print_progress(false);
    full_params.set_print_realtime(false);
    full_params.set_print_timestamps(false);

    state.full(full_params, &audio_data.samples[..])?;

    let num_segments = state.full_n_segments()?;

    let mut transcription = String::new();

    for i in 0..num_segments {
        let segment_text = state.full_get_segment_text(i)?;
        let trimmed = segment_text.trim();
        if !trimmed.is_empty() && trimmed != "[BLANK_AUDIO]" {
            transcription.push_str(trimmed);
            transcription.push(' ');
        }
    }

    let output = transcription.trim().to_string();

    if let Some(outfile) = &args.outfile {
        let mut file = File::create(outfile)?;
        file.write_all(output.as_bytes())?;
        log_success!("Transcription saved to {}", outfile);
    } else {
        println!("{}", output);
    }

    log_success!("Transcription complete");
    Ok(())
}
