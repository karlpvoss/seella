use crate::data_source::DataSource;
use crate::records::{EventRecord, SessionRecord};
use chrono::{Duration, FixedOffset, TimeZone};
use futures::executor::block_on;
use scylla::FromRow;
use std::{collections::HashMap, net::IpAddr, net::SocketAddr};
use thiserror::Error;
use uuid::Uuid;

/// A source for the data based on an exported CSV.
#[derive(Debug)]
pub struct DbSource {
    addr: SocketAddr,
    session_id: Uuid,
}

impl<'a> DbSource {
    pub fn new(addr: impl Into<SocketAddr>, session_id: Uuid) -> Self {
        Self {
            addr: addr.into(),
            session_id,
        }
    }
}

/// The kinds of errors that can be experienced while parsing the data from the CSV.
#[derive(Debug, Error)]
pub enum DbParsingError {
    #[error("the provided session id {0} could not be found")]
    SessionNotFound(Uuid),
    #[error("there were issues deserializing the session data")]
    SessionDeserializationErrors(Vec<csv::Error>),
    #[error("there were issues deserializing the event data")]
    EventDeserializationErrors(Vec<csv::Error>),
    #[error("there were issues finding the files")]
    IoError(#[from] std::io::Error),
}

impl DataSource for DbSource {
    type Error = DbParsingError;

    fn get_data(&self) -> Result<(SessionRecord, Vec<EventRecord>), Self::Error> {
        let _: Result<_, _> = block_on(async {
            let conn = scylla::SessionBuilder::new()
                .known_node_addr(self.addr)
                .build()
                .await?;

            let (session_id, client, command, coordinator, duration, parameters, request, started_at, request_size, response_size, username) = 
                <(String, IpAddr, String, IpAddr, i32, HashMap<String, String>, String, i64, i32, i32, String)>::from_row(conn.query(
                    "SELECT session_id, client, command, coordinator, duration, parameters, request, started_at, request_size, response_size, username FROM system_traces.sessions WHERE session_id=?", 
                    (self.session_id.to_string(),)).await?.first_row()?)?;
            let session_record = SessionRecord {
                session_id: Uuid::parse_str(&session_id)?,
                client,
                command,
                coordinator,
                duration,
                parameters: todo!(),
                request,
                started_at: todo!(),
                request_size: Some(request_size as u32),
                response_size: Some(response_size as u32),
                username: Some(username),
            };

            let (session_id, client, command, coordinator, duration, parameters, request, started_at, request_size, response_size, username) = 
                <(String, IpAddr, String, IpAddr, i32, HashMap<String, String>, String, i64, i32, i32, String)>::from_row(conn.query(
                    "SELECT session_id, client, command, coordinator, duration, parameters, request, started_at, request_size, response_size, username FROM system_traces.sessions WHERE session_id=?", 
                    (self.session_id.to_string(),)).await?.first_row()?)?;
            


            Ok((session_record))
        });

        Ok(())
    }
}
