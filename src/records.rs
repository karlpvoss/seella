use crate::SpanId;
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SessionRecord {
    pub(crate) session_id: Uuid,
    pub(crate) client: IpAddr,
    pub(crate) command: String,
    pub(crate) coordinator: IpAddr,
    pub(crate) duration: i32,
    pub(crate) parameters: String,
    pub(crate) request: String,
    pub(crate) started_at: DateTime<FixedOffset>,

    // The following are not present in Cassandra:
    #[serde(default)]
    pub(crate) request_size: Option<u32>, // Strictly speaking, the int type in Scylla is signed, but it doesn't make sense for a size to be negative.
    #[serde(default)]
    pub(crate) response_size: Option<u32>, // Strictly speaking, the int type in Scylla is signed, but it doesn't make sense for a size to be negative.
    #[serde(default)]
    pub(crate) username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EventRecord {
    pub(crate) session_id: Uuid,
    pub(crate) event_id: Uuid,
    pub(crate) activity: String,
    pub(crate) source: IpAddr,
    pub(crate) source_elapsed: i32,
    pub(crate) thread: String,

    // The following are not present in Cassandra:
    #[serde(default)]
    pub(crate) scylla_parent_id: Option<SpanId>,
    #[serde(default)]
    pub(crate) scylla_span_id: Option<SpanId>,
}
