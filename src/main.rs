use clap::Parser;
use whisper_rs_cli::cli::Commands;
use whisper_rs_cli::error::Result;
use whisper_rs_cli::init_whisper_logging;
use whisper_rs_cli::set_silent;

fn main() -> Result<()> {
    init_whisper_logging();

    let cli = whisper_rs_cli::cli::Cli::parse();

    set_silent(!cli.debug);

    match cli.command {
        Commands::Transcribe(args) => whisper_rs_cli::cli::transcribe::execute(&args),
    }
}
