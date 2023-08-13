use clap::Parser;
use seella::{event_display_str, session_from_config, Cli};

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
    let s_end = s.total_duration();
    let mut offset = 0i64;

    let events = s.events();
    let a_max_width = events
        .iter()
        .map(|(e, _)| e.activity_length())
        .max()
        .unwrap_or(0);
    let max_depth = events.iter().map(|(_, depth)| *depth).max().unwrap_or(1);
    let i_max_width = s.event_count().to_string().len();

    // Headers
    println!(
        "{:i_max_width$} {:102} {}",
        "",
        "waterfall chart",
        event_display_str(
            &cli,
            a_max_width,
            "dur",
            "node",
            &format!("{:tree_width$}", "", tree_width = max_depth + 2),
            "activity",
            "event id",
            "span id",
            "parent span id",
            "thread name",
        )
    );

    for (i, (e, depth)) in events.iter().enumerate() {
        println!(
            "{:i_max_width$} {} {}",
            i + 1,
            e.waterfall(offset, s_end),
            e.display(&cli, a_max_width, *depth, max_depth)
        );

        // Move the offset up for the next event
        offset += e.durations().1;
    }

    Ok(())
}
