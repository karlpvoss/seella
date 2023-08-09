use chrono::{DateTime, Duration, FixedOffset};
use serde::Deserialize;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    net::IpAddr,
};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SpanId(i64);

impl SpanId {
    pub fn new(id: i64) -> Self {
        SpanId(id)
    }

    pub fn is_root(&self) -> bool {
        self.0 == 0
    }
}

impl From<i64> for SpanId {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl Display for SpanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct SessionRecord {
    session_id: Uuid,
    client: IpAddr,
    command: String,
    coordinator: IpAddr,
    duration: i32,
    parameters: String,
    request: String,
    request_size: i32,
    response_size: i32,
    started_at: DateTime<FixedOffset>,
    username: String,
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Session {
    id: Uuid,
    client: IpAddr,
    command: String,
    coordinator: IpAddr,
    duration: Duration,
    parameters: HashMap<String, String>,
    request: String,
    request_size: i32,
    response_size: i32,
    started_at: DateTime<FixedOffset>,
    username: String,
    root_events: Vec<Event>,
}

impl Session {
    pub fn new(
        session_record: SessionRecord,
        event_records: Vec<EventRecord>,
    ) -> Result<Self, serde_json::Error> {
        let (mut root_events, mut child_events): (VecDeque<Event>, VecDeque<Event>) = event_records
            .into_iter()
            .map(|record| Event::from(record))
            .partition(|event| event.parent_span_id().is_root());

        'child_events: while let Some(child_event) = child_events.pop_front() {
            let mut opt = Some(child_event);
            '_root_search: for root_event in &mut root_events {
                // Safe to unwrap here since we'll always add it back in the case that it's not a child of a particular event.
                match root_event.try_add_child(opt.take().unwrap()) {
                    // In the case that this has been handled, we want to move to the next,
                    // skipping adding it back onto the queue after the for loop below.
                    Ok(_) => continue 'child_events,
                    Err(child_event) => opt = Some(child_event),
                }
            }

            // child event was not the child of any current root event or their children. Add it back to the queue.
            child_events.push_back(opt.take().unwrap());
        }

        Ok(Self {
            id: session_record.session_id,
            client: session_record.client,
            command: session_record.command(),
            coordinator: session_record.coordinator,
            duration: Duration::microseconds(session_record.duration.into()),
            parameters: session_record.parameters()?,
            request: session_record.request(),
            request_size: session_record.request_size,
            response_size: session_record.response_size,
            started_at: session_record.started_at,
            username: session_record.username(),
            root_events: root_events.into(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct EventRecord {
    session_id: Uuid,
    event_id: Uuid,
    activity: String,
    scylla_parent_id: i64,
    scylla_span_id: i64,
    source: IpAddr,
    source_elapsed: i32,
    thread: String,
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct Event {
    id: Uuid,
    session_id: Uuid,
    span_id: SpanId,
    parent_span_id: SpanId,
    activity: String,
    source: IpAddr,
    source_elapsed: Duration,
    thread: String,
    child_events: Vec<Event>,
}

impl Event {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn session_id(&self) -> Uuid {
        self.session_id
    }

    pub fn span_id(&self) -> SpanId {
        self.span_id
    }

    pub fn parent_span_id(&self) -> SpanId {
        self.parent_span_id
    }

    fn try_add_child(&mut self, child_event: Event) -> Result<(), Event> {
        // Base case, the provided event is a direct child of this event
        if child_event.parent_span_id() == self.span_id() {
            self.child_events.push(child_event);
            return Ok(());
        }

        // Recursion case, need to see if it is a child of any children
        let mut opt = Some(child_event);
        for event in &mut self.child_events {
            // Safe to unwrap here because we will always put it back if the below returns Err
            let child_event = opt.take().unwrap();
            match event.try_add_child(child_event) {
                Ok(_) => return Ok(()),
                Err(child_event) => {
                    opt = Some(child_event);
                    continue;
                }
            }
        }

        // Error case, the event was not a direct child not a child of any children.
        // Safe to unwrap here because we always will have returned the object, since it never fit in any of the above events.
        Err(opt.take().unwrap())
    }
}

impl From<EventRecord> for Event {
    fn from(value: EventRecord) -> Self {
        Self {
            id: value.event_id,
            session_id: value.session_id,
            span_id: value.scylla_span_id.into(),
            parent_span_id: value.scylla_parent_id.into(),
            activity: value.activity(),
            source: value.source,
            source_elapsed: Duration::microseconds(value.source_elapsed.into()),
            thread: value.thread(),
            child_events: Vec::new(),
        }
    }
}
