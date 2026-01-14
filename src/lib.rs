pub mod audio;
pub mod cli;
pub mod error;
pub mod model;
pub mod output;

pub use error::{Result, WhisperError};
pub use output::logger::{init_whisper_logging, set_silent};
