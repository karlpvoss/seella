use crate::{data_source::{DataSource, DataSourceResult}, records::{EventRecord, SessionRecord}, SpanId};
use chrono::{TimeZone, Utc, LocalResult};
use futures::executor::block_on;
use scylla::FromRow;
use std::{collections::HashMap, net::IpAddr, net::SocketAddr, str::FromStr};
use thiserror::Error;
use uuid::Uuid;

pub type DbSessionRecord = (String, IpAddr, String, IpAddr, i32, HashMap<String, String>, String, i64, i32, i32, String);
pub type DbEventRecord = (String, String, String, IpAddr, i32, String, i64, i64);

/// A source for the data based on an exported CSV.
#[derive(Debug)]
pub struct DbSource<'a> {
    addr: SocketAddr,
    session_id: &'a str,
}

impl<'a> DbSource<'a> {
    pub fn new(addr: impl Into<SocketAddr>, session_id: &'a str) -> Self {
        Self {
            addr: addr.into(),
            session_id,
        }
    }
}

/// The kinds of errors that can be experienced while parsing the data from the CSV.
#[derive(Debug, Error)]
pub enum DbParsingError {
    #[error("the provided uuid could not be parsed: {0}")]
    UuidParse(#[from] uuid::Error),
    #[error("there was an issue creating your db session: {0}")]
    NewSessionError(#[from] scylla::transport::errors::NewSessionError),
    #[error("the request resulted in a error: {0}")]
    ScyllaQueryError(#[from] scylla::transport::errors::QueryError),
    #[error("we didn't get any results back from the db: {0}")]
    FisrtRowError(#[from] scylla::transport::query_result::FirstRowError),
    #[error("we didn't get any results back from the db: {0}")]
    RowsExpectedError(#[from] scylla::transport::query_result::RowsExpectedError),
    #[error("there was an issue parsing the data from the returned row: {0}")]
    FromRowError(#[from] scylla::cql_to_rust::FromRowError),
}

impl<'a> DataSource for DbSource<'a> {
    type Error = DbParsingError;
    type P = HashMap<String, String>;
    
    #[allow(clippy::type_complexity)] 
    fn get_data(&self) -> DataSourceResult<Self::P, Self::Error> {
        block_on(async {
            let conn = scylla::SessionBuilder::new()
                .known_node_addr(self.addr)
                .build()
                .await?;

            let (session_id, client, command, coordinator, duration, parameters, request, started_at, request_size, response_size, username): DbSessionRecord = 
                <_>::from_row(conn.query(
                    "SELECT session_id, client, command, coordinator, duration, parameters, request, started_at, request_size, response_size, username FROM system_traces.sessions WHERE session_id=?", 
                    (self.session_id.to_string(),)).await?.first_row()?)?;
            
            let started_at = match Utc.timestamp_millis_opt(started_at) {
                LocalResult::Single(datetime) => datetime,
                _ => todo!(),
            };

            let session_record = SessionRecord {
                session_id: Uuid::parse_str(&session_id)?,
                client,
                command,
                coordinator,
                duration,
                parameters,
                request,
                started_at,
                request_size: Some(request_size as u32),
                response_size: Some(response_size as u32),
                username: Some(username),
            };

            let rows = conn.query(
                "SELECT session_id, event_id, activity, source, source_elapsed, thread, scylla_parent_id, scylla_span_id FROM system_traces.sessions WHERE session_id=?", 
                (self.session_id.to_string(),)).await?.rows()?;

            let mut event_records = vec![];
            for row in rows {
                let (session_id, event_id, activity, source, source_elapsed, thread, scylla_parent_id, scylla_span_id): DbEventRecord = 
                <_>::from_row(row)?;
                event_records.push(EventRecord {
                    session_id: Uuid::from_str(&session_id)?,
                    event_id: Uuid::from_str(&event_id)?,
                    activity,
                    source,
                    source_elapsed,
                    thread,
                    scylla_parent_id: Some(SpanId::new(scylla_parent_id)),
                    scylla_span_id: Some(SpanId::new(scylla_span_id))
                })
            }

            Ok((session_record, event_records))
    })
}   
}
