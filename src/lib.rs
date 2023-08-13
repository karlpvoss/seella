//! A tool for visualising the traces emitted by ScyllaDB.
//!
//! Use it like so!
//! ```rust
//! # use clap::Parser;
//! # use seella::{session_from_config, Cli};
//! # fn main() -> anyhow::Result<()> {
//! let cli = Cli::parse();
//! let s = session_from_config(&cli)?;
//!
//! println!("Session ID: {}", &s.id());
//! println!("{}", &s.started_at.to_rfc3339());
//!
//! for e in s.events() {
//!     println!("{}", e.display(&cli, 100));
//! }
//!
//! # Ok(())
//! # }
//! ```

mod cli;
mod event;
mod records;
mod session;

use crate::records::{EventRecord, SessionRecord};
use anyhow::{anyhow, bail};
use std::{fs::File, io::BufReader};
use uuid::Uuid;

pub use {
    cli::Cli,
    event::{event_display_str, Event, SpanId},
    session::Session,
};

pub(crate) const COMPLAIN_ABOUT_TRACE_SIZE: &str =
    "what are you doing with 2^63 microseconds in a single trace!";

/// Constructs a Session instance from the files given in the [cli][Cli] config.
///
/// This [Session] instance contains all of the information available from the `session.csv` file, as well as all
/// of the information for the [events][Event] relating to that session from the `events.csv` file.
///
/// ```rust
/// # use clap::Parser;
/// # use seella::{session_from_config, Cli};
/// # fn main() -> anyhow::Result<()> {
/// let cli = Cli::parse();
/// let session = session_from_config(&cli)?;
/// println!("Session ID: {}", &session.id());
/// println!("{}", &session.started_at.to_rfc3339());
///
/// for event in session.events() {
///     println!("{}", event.id());
/// }
/// # }
/// ```
pub fn session_from_config(cli: &Cli) -> anyhow::Result<Session> {
    let session_id = Uuid::try_parse(&cli.session_id)?;

    let mut session_deserialization_errors = Vec::new();
    let session_record = csv::Reader::from_reader(BufReader::new(File::open(&cli.sessions_path)?))
        .deserialize::<SessionRecord>()
        .filter_map(|record_res| {
            record_res
                .map_err(|err| session_deserialization_errors.push(err))
                .ok()
        })
        .find(|record| record.id() == session_id)
        .ok_or(anyhow!(
            "could not find the session with id: {}",
            session_id
        ))?;

    if !session_deserialization_errors.is_empty() {
        for error in session_deserialization_errors {
            eprintln!("{}", error);
        }
        bail!(
            "errors were experienced deserializing sessions from {:?}",
            &cli.sessions_path
        );
    }

    let mut event_deserialization_errors = Vec::new();
    let event_records: Vec<EventRecord> =
        csv::Reader::from_reader(BufReader::new(File::open(&cli.events_path)?))
            .deserialize::<EventRecord>()
            .filter_map(|record_res| {
                record_res
                    .map_err(|err| event_deserialization_errors.push(err))
                    .ok()
            })
            .filter(|record| record.session_id() == session_id)
            .collect();

    if !event_deserialization_errors.is_empty() {
        for error in event_deserialization_errors {
            eprintln!("{}", error);
        }

        bail!(
            "errors were experienced deserializing events from {:?}",
            &cli.events_path
        );
    }

    Ok(Session::new(session_record, event_records))
}
