use crate::{
    records::{EventRecord, SessionRecord},
    SpanId,
};
use chrono::{LocalResult, TimeZone, Utc};
use scylla::{query::Query, statement::Consistency, FromRow};
use std::{collections::HashMap, net::IpAddr, net::SocketAddr};
use thiserror::Error;
use uuid::Uuid;

pub type DbSessionRecord = (
    Uuid,
    IpAddr,
    String,
    IpAddr,
    i32,
    HashMap<String, String>,
    String,
    i64,
    i32,
    i32,
    String,
);
pub type DbEventRecord = (Uuid, Uuid, String, IpAddr, i32, String, i64, i64);

/// A source for the data based on an exported CSV.
#[derive(Debug)]
pub struct DbSource {
    addr: SocketAddr,
    session_id: Uuid,
}

impl DbSource {
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
    #[error("there was an issue creating your db session: {0}")]
    NewSession(#[from] scylla::transport::errors::NewSessionError),

    #[error("the request resulted in a error: {0}")]
    ScyllaQuery(#[from] scylla::transport::errors::QueryError),

    #[error("we didn't get any results back from the db: {0}")]
    FirstRow(#[from] scylla::transport::query_result::FirstRowError),

    #[error("we didn't get any results back from the db: {0}")]
    RowsExpected(#[from] scylla::transport::query_result::RowsExpectedError),

    #[error("there was an issue parsing the data from the returned row: {0}")]
    FromRow(#[from] scylla::cql_to_rust::FromRowError),
}

impl DbSource {
    pub async fn get_data(&self) -> Result<(SessionRecord, Vec<EventRecord>), DbParsingError> {
        let conn = scylla::SessionBuilder::new()
            .known_node_addr(self.addr)
            .build()
            .await?;

        let mut session_query = Query::from("SELECT session_id, client, command, coordinator, duration, parameters, request, started_at, request_size, response_size, username FROM system_traces.sessions WHERE session_id=?");
        session_query.set_consistency(Consistency::One);
        let (
            session_id,
            client,
            command,
            coordinator,
            duration,
            parameters,
            request,
            started_at,
            request_size,
            response_size,
            username,
        ): DbSessionRecord = <_>::from_row(
            conn.query(session_query, (self.session_id,))
                .await?
                .first_row()?,
        )?;

        let started_at = match Utc.timestamp_millis_opt(started_at) {
            LocalResult::Single(datetime) => datetime,
            _ => todo!(),
        };

        let session_record = SessionRecord {
            session_id,
            client,
            command,
            coordinator,
            duration,
            parameters: format!("{:?}", parameters),
            request,
            started_at,
            request_size: Some(request_size as u32),
            response_size: Some(response_size as u32),
            username: Some(username),
        };

        let mut event_query = Query::from(
            "SELECT session_id, event_id, activity, source, source_elapsed, thread, scylla_parent_id, scylla_span_id FROM system_traces.events WHERE session_id=?");
        event_query.set_consistency(Consistency::One);
        let rows = conn.query(event_query, (self.session_id,)).await?.rows()?;

        let mut event_records = vec![];
        for row in rows {
            let (
                session_id,
                event_id,
                activity,
                source,
                source_elapsed,
                thread,
                scylla_parent_id,
                scylla_span_id,
            ): DbEventRecord = <_>::from_row(row)?;
            event_records.push(EventRecord {
                session_id,
                event_id,
                activity,
                source,
                source_elapsed,
                thread,
                scylla_parent_id: Some(SpanId::new(scylla_parent_id)),
                scylla_span_id: Some(SpanId::new(scylla_span_id)),
            })
        }

        Ok((session_record, event_records))
    }
}
