use clap::Parser;
use seella::{session_from_config, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let s = session_from_config(&cli)?;

    // Print out the session info
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

    // Calculations for the waterfall boxes
    let s_start = 0i64;
    let s_end = s.total_duration();
    let mut offset = 0i64;

    let events = s.events();
    let a_max_width = events
        .iter()
        .map(|e| e.activity_length())
        .max()
        .or(Some(0))
        .unwrap();
    let i_max_width = s.event_count().to_string().len();

    for (i, e) in events.iter().enumerate() {
        let waterfall = e.waterfall(offset, s_start, s_end);

        println!(
            "{:i_max_width$} {} {}",
            i + 1,
            waterfall,
            e.display(&cli, a_max_width)
        );

        // Move the offset up for the next root event
        offset += e.durations().1;
    }

    Ok(())
}
