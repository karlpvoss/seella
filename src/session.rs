use crate::{
    event::Event,
    records::{EventRecord, SessionRecord},
};
use chrono::{DateTime, Duration, FixedOffset};
use std::{
    collections::{HashMap, VecDeque},
    net::IpAddr,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub id: Uuid,
    pub client: IpAddr,
    pub command: String,
    pub coordinator: IpAddr,
    pub duration: Duration,
    pub parameters: HashMap<String, String>,
    pub request: String,
    pub request_size: i32,
    pub response_size: i32,
    pub started_at: DateTime<FixedOffset>,
    pub username: String,
    root_events: Vec<Event>,
}

impl Session {
    pub(crate) fn new(
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn event_count(&self) -> usize {
        self.root_events
            .iter()
            .map(|e| e.count_including_children())
            .sum::<usize>()
    }

    pub fn events(&self) -> Vec<&Event> {
        let count = self.event_count();
        let mut events = Vec::with_capacity(count);

        for root_event in &self.root_events {
            root_event.recurse_events(&mut events);
        }

        events
    }
}
