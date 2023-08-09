use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Configuration for the clap parser driving the binary.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The session id to be visualised
    pub session_id: String,

    /// Path to the CSV containing the sessions data. Any string that can be coerced into a PathBuf
    #[arg(short, long, default_value = "sessions.csv")]
    pub sessions_path: PathBuf,

    /// Path to the CSV containing the events data. Any string that can be coerced into a PathBuf
    #[arg(short, long, default_value = "events.csv")]
    pub events_path: PathBuf,

    /// The width of the waterfall chart
    #[arg(short, long, default_value_t = 100)]
    pub waterfall_width: usize,

    /// Whether to generate span durations in milliseconds or microseconds
    #[arg(value_enum, short, long, default_value_t = DurationFormat::Millis)]
    pub duration_format: DurationFormat,

    /// Minimum print width for the duration field, remaining will be filled with spaces
    #[arg(long, default_value_t = 5)]
    pub min_duration_width: usize,

    /// Maximum print width for the activity field, remaining will be truncated
    #[arg(long, default_value_t = 300)]
    pub max_activity_width: usize,

    /// Whether to show the event uuid
    #[arg(long)]
    pub show_event_id: bool,

    /// Whether to show the span ids
    #[arg(long)]
    pub show_span_ids: bool,

    /// Whether to show the thread name
    #[arg(long)]
    pub show_thread: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum DurationFormat {
    Millis,
    Micros,
}
