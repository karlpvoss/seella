use crate::{cli::DurationFormat, records::EventRecord, Cli};
use chrono::Duration;
use std::{fmt::Display, net::IpAddr};
use uuid::Uuid;

/// All of the information related to an event, as well as all child events.
#[derive(Debug)]
pub struct Event {
    pub id: Uuid,
    pub session_id: Uuid,
    pub span_id: SpanId,
    pub parent_span_id: SpanId,
    pub activity: String,
    pub source: IpAddr,
    pub source_elapsed: Duration,
    pub thread: String,
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

    /// The length of the string in [Event::activity].
    ///
    /// Used for nicer formatting.
    pub fn activity_length(&self) -> usize {
        self.activity.len()
    }

    pub(crate) fn try_add_child(&mut self, child_event: Event) -> Result<(), Event> {
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

    /// Generates a texttual representation of the event to display alongside the waterfall view.
    ///
    /// This contains, by default, the [span duration][Event::source_elapsed], [source node IP][Event::source],
    /// and the [activity][Event::activity]:
    ///
    /// ```text
    ///     0 10.17.145.76    Querying is done
    /// ```
    ///
    /// This can be extended with the [config][Cli] to include the [event id][Event::id],
    /// the [local][Event::span_id] and [parent][Event::parent_span_id] span IDs, and the [thread name][Event::thread]:
    ///
    /// ```text
    ///     0 10.17.145.76    Querying is done     3d07a953-313e-11ee-95bc-69d50677a8c4 75964065742287       191362128677         shard 2
    /// ```
    ///
    pub fn display(&self, config: &Cli, min_activity_width: usize) -> String {
        let duration = match config.duration_format {
            DurationFormat::Millis => self.source_elapsed.num_milliseconds(),
            DurationFormat::Micros => self
                .source_elapsed
                .num_microseconds()
                .or(Some(i64::MAX))
                .unwrap(),
        };
        let d_min = config.min_duration_width;
        let source = self.source;
        let activity = &self.activity;
        let a_min = min_activity_width.min(config.max_activity_width);
        let a_max = config.max_activity_width;
        let id = if config.show_event_id {
            format!(" {}", self.id.to_string())
        } else {
            String::new()
        };
        let span_ids = if config.show_span_ids {
            format!(
                " {span_id:20} {parent_span_id:20}",
                span_id = self.span_id.to_string(),
                parent_span_id = self.parent_span_id.to_string()
            )
        } else {
            String::new()
        };
        let thread = if config.show_thread {
            format!(" {}", &self.thread)
        } else {
            String::new()
        };

        format!("{duration:d_min$} {source:15} {activity:a_min$.a_max$}{id}{span_ids}{thread}")
    }

    pub(crate) fn count_including_children(&self) -> usize {
        1 + self
            .child_events
            .iter()
            .map(|e| e.count_including_children())
            .sum::<usize>()
    }

    pub(crate) fn recurse_events<'a>(&'a self, vec: &mut Vec<&'a Event>) {
        vec.extend(std::iter::once(self));
        for child in &self.child_events {
            child.recurse_events(vec);
        }
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

/// Wrapper type for the `i64` used by Scylla for span IDs.
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
