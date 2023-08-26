use clap::Parser;
use seella::{session_from_csv, Cli, OperationMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let s = match cli.mode {
        OperationMode::Csv(ref options) => session_from_csv(
            &options.sessions_path,
            &options.events_path,
            &options.session_id,
        )?,
        OperationMode::Db => unimplemented!(),
    };

    s.display(cli, &mut std::io::stdout())?;

    Ok(())
}
