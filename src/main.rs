use clap::Parser;
use seella::{session_from_config, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let s = session_from_config(&cli)?;

    println!("Session ID: {}", &s.id);
    println!("{}", &s.started_at.to_rfc3339());
    println!(
        "{:15} ({}) -> {:15}",
        &s.client, &s.username, &s.coordinator
    );
    println!("Request Size:  {}", &s.request_size);
    println!("Response Size: {}", &s.response_size);
    println!("{} {}", &s.command, &s.request);
    println!("{:?}", &s.parameters);

    let events = s.events();
    let activity_width = events
        .iter()
        .map(|e| e.activity_length())
        .max()
        .or(Some(0))
        .unwrap();
    for e in s.events() {
        println!("{}", e.display(&cli, activity_width));
    }

    Ok(())
}
