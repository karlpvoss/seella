use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use std::{collections::HashMap, net::IpAddr};
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
    pub(crate) request_size: i32,
    pub(crate) response_size: i32,
    pub(crate) started_at: DateTime<FixedOffset>,
    pub(crate) username: String,
}

impl SessionRecord {
    pub fn id(&self) -> Uuid {
        self.session_id
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn parameters(&self) -> Result<HashMap<String, String>, serde_json::Error> {
        // Since this is coming out of a double-quoted CSV, all double quotes within the string should already be escaped.
        // Therefore, it should be safe to replace all single quotes with double quotes and call it JSON.
        serde_json::from_str(&self.parameters.clone().replace("'", "\""))
    }

    pub fn request(&self) -> String {
        self.request.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct EventRecord {
    pub(crate) session_id: Uuid,
    pub(crate) event_id: Uuid,
    pub(crate) activity: String,
    pub(crate) scylla_parent_id: i64,
    pub(crate) scylla_span_id: i64,
    pub(crate) source: IpAddr,
    pub(crate) source_elapsed: i32,
    pub(crate) thread: String,
}

impl EventRecord {
    pub fn id(&self) -> Uuid {
        self.event_id
    }

    pub fn session_id(&self) -> Uuid {
        self.session_id
    }

    pub fn activity(&self) -> String {
        self.activity.clone()
    }

    pub fn thread(&self) -> String {
        self.thread.clone()
    }
}
