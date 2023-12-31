//! A tool for visualising the traces emitted by ScyllaDB.

mod cli;
mod csv;
mod data_source;
mod db;
mod event;
mod records;
mod session;

use std::{net::SocketAddr, path::PathBuf};
use uuid::Uuid;

pub use {crate::csv::*, cli::*, db::*, event::*, records::*, session::*};

/// It's possible for a [chrono::Duration] to have more than 2^63 microseconds and overflow, we ignore that possibility.
pub const COMPLAIN_ABOUT_TRACE_SIZE: &str =
    "what are you doing with 2^63 microseconds in a single trace!";

/// Constructs a [Session] instance from the files given in the [CsvModeOptions] config.
///
/// This [Session] instance contains all of the information available from the `session.csv` file, as well as all
/// of the information for the [events][Event] relating to that session from the `events.csv` file.
pub fn session_from_csv(
    sessions_path: &PathBuf,
    events_path: &PathBuf,
    session_id: &str,
) -> Result<Session, Box<dyn std::error::Error>> {
    let session_id = Uuid::try_parse(session_id)?;
    let (session_record, event_records) =
        CsvSource::new(sessions_path, events_path, session_id).get_data()?;

    Ok(Session::new(session_record, event_records))
}

/// Constructs a [Session] instance from a live database, given the [DbModeOptions] config.
///
/// This [Session] instance contains all of the information available from the `system_traces.sessions` table, as well
/// as all of the information for the [events][Event] relating to that session from the `system_traces.events` table.
pub async fn session_from_db(
    addr: SocketAddr,
    session_id: &str,
) -> Result<Session, Box<dyn std::error::Error>> {
    let session_id = Uuid::try_parse(session_id)?;
    let (session_record, event_records) = DbSource::new(addr, session_id).get_data().await?;
    Ok(Session::new(session_record, event_records))
}
