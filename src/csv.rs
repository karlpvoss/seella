use crate::data_source::{DataSource, DataSourceResult};
use crate::records::{EventRecord, SessionRecord};
use std::{fs::File, io::BufReader, path::PathBuf};
use thiserror::Error;
use uuid::Uuid;

/// A source for the data based on an exported CSV.
#[derive(Debug)]
pub struct CsvSource<'a> {
    sessions: &'a PathBuf,
    events: &'a PathBuf,
    session_id: Uuid,
}

impl<'a> CsvSource<'a> {
    pub fn new(sessions: &'a PathBuf, events: &'a PathBuf, session_id: Uuid) -> Self {
        Self {
            sessions,
            events,
            session_id,
        }
    }
}

/// The kinds of errors that can be experienced while parsing the data from the CSV.
#[derive(Debug, Error)]
pub enum CsvParsingError {
    #[error("the provided session id {0} could not be found")]
    SessionNotFound(Uuid),
    #[error("there were issues deserializing the session data")]
    SessionDeserializationErrors(Vec<csv::Error>),
    #[error("there were issues deserializing the event data")]
    EventDeserializationErrors(Vec<csv::Error>),
    #[error("there were issues finding the files")]
    IoError(#[from] std::io::Error),
}

impl<'a> DataSource for CsvSource<'a> {
    type Error = CsvParsingError;

    fn get_data(&self) -> DataSourceResult<Self::Error> {
        let mut session_deserialization_errors = Vec::new();
        let session_record = csv::Reader::from_reader(BufReader::new(File::open(self.sessions)?))
            .deserialize::<SessionRecord>()
            .filter_map(|record_res| {
                record_res
                    .map_err(|err| session_deserialization_errors.push(err))
                    .ok()
            })
            .find(|record| record.session_id == self.session_id)
            .ok_or(CsvParsingError::SessionNotFound(self.session_id))?;

        if !session_deserialization_errors.is_empty() {
            return Err(CsvParsingError::SessionDeserializationErrors(
                session_deserialization_errors,
            ));
        }

        let mut event_deserialization_errors = Vec::new();
        let event_records: Vec<EventRecord> =
            csv::Reader::from_reader(BufReader::new(File::open(self.events)?))
                .deserialize::<EventRecord>()
                .filter_map(|record_res| {
                    record_res
                        .map_err(|err| event_deserialization_errors.push(err))
                        .ok()
                })
                .filter(|record| record.session_id == self.session_id)
                .collect();

        if !event_deserialization_errors.is_empty() {
            return Err(CsvParsingError::EventDeserializationErrors(
                event_deserialization_errors,
            ));
        }

        Ok((session_record, event_records))
    }
}
