use clap::Parser;
use seella::{session_from_config, Cli};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let s = session_from_config(&cli)?;

    s.display(cli);

    Ok(())
}
