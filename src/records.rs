use crate::SpanId;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{fmt::Debug, net::IpAddr};
use uuid::Uuid;

/// The basic structure and data of a Session, before it is made into the head of a tree.
#[derive(Debug, Deserialize)]
pub struct SessionRecord<P> {
    pub session_id: Uuid,
    pub client: IpAddr,
    pub command: String,
    pub coordinator: IpAddr,
    pub duration: i32,
    pub parameters: P,
    pub request: String,
    pub started_at: DateTime<Utc>,

    // The following are not present in Cassandra:
    #[serde(default)]
    pub request_size: Option<u32>, // Strictly speaking, the int type in Scylla is signed, but it doesn't make sense for a size to be negative.
    #[serde(default)]
    pub response_size: Option<u32>, // Strictly speaking, the int type in Scylla is signed, but it doesn't make sense for a size to be negative.
    #[serde(default)]
    pub username: Option<String>,
}

/// The basic structure and data of a Event, before it is made into the leaves of a tree.
#[derive(Debug, Deserialize)]
pub struct EventRecord {
    pub session_id: Uuid,
    pub event_id: Uuid,
    pub activity: String,
    pub source: IpAddr,
    pub source_elapsed: i32,
    pub thread: String,

    // The following are not present in Cassandra:
    #[serde(default)]
    pub scylla_parent_id: Option<SpanId>,
    #[serde(default)]
    pub scylla_span_id: Option<SpanId>,
}
