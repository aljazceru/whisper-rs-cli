use clap::Parser;
use whisper_rs_cli::cli::Commands;
use whisper_rs_cli::error::Result;
use whisper_rs_cli::output::set_silent;

fn main() -> Result<()> {
    let cli = whisper_rs_cli::cli::Cli::parse();

    set_silent(cli.silent);

    match cli.command {
        Commands::Transcribe(args) => whisper_rs_cli::cli::transcribe::execute(&args),
    }
}
