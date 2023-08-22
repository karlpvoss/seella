//! A tool for visualising the traces emitted by ScyllaDB.

mod cli;
mod csv;
mod data_source;
mod event;
mod records;
mod session;

use uuid::Uuid;

pub use {
    crate::csv::{CsvParsingError, CsvSource},
    cli::Cli,
    data_source::DataSource,
    event::{event_display_str, Event, SpanId},
    session::Session,
};

pub(crate) const COMPLAIN_ABOUT_TRACE_SIZE: &str =
    "what are you doing with 2^63 microseconds in a single trace!";

/// Constructs a Session instance from the files given in the [cli][Cli] config.
///
/// This [Session] instance contains all of the information available from the `session.csv` file, as well as all
/// of the information for the [events][Event] relating to that session from the `events.csv` file.
pub fn session_from_config(cli: &Cli) -> Result<Session, Box<dyn std::error::Error>> {
    let session_id = Uuid::try_parse(&cli.session_id)?;
    let (session_record, event_records) =
        CsvSource::new(&cli.sessions_path, &cli.events_path, session_id).get_data()?;

    Ok(Session::new(session_record, event_records))
}
