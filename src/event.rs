use crate::{cli::DurationFormat, records::EventRecord, Cli, COMPLAIN_ABOUT_TRACE_SIZE};
use chrono::Duration;
use std::{fmt::Display, net::IpAddr};
use uuid::Uuid;

/// All of the information related to an event, as well as all child events.
#[derive(Debug)]
pub struct Event {
    /// The UUID of the Event
    pub id: Uuid,
    /// The UUID of the Session
    pub session_id: Uuid,
    /// What is being done in this Event
    pub activity: String,
    /// The source IP for this Event
    pub source: IpAddr,
    /// Duration of this only this Event, not including child events
    pub duration: Duration,
    /// The name of the thread from which this Event originated
    pub thread: String,

    /// Unique identifier for this Event's span
    /// Not present in Cassandra
    pub span_id: SpanId,
    /// Span ID for this Event's parent. Used to identity the tree structure
    /// Not present in Cassandra
    pub parent_span_id: SpanId,

    child_events: Vec<Event>,
}

impl Event {
    /// The length of the string in [Event::activity].
    ///
    /// Used for nicer formatting.
    pub fn activity_length(&self) -> usize {
        self.activity.len()
    }

    /// Return the total duration of this span and it's children, and the duration of just this span.
    ///
    /// First field is the total, second field is just this span.
    pub fn durations(&self) -> (i64, i64) {
        let self_dur = self
            .duration
            .num_microseconds()
            .expect(COMPLAIN_ABOUT_TRACE_SIZE);

        (self_dur + self.sum_of_child_durations(), self_dur)
    }

    /// Returns the total sum of all children's durations.
    pub fn sum_of_child_durations(&self) -> i64 {
        self.child_events.iter().map(|e| e.durations().0).sum()
    }

    /// Generate a waterfall chart from the trace data.
    ///
    /// The intention of this is to show the proportion of the total trace taken up by this span,
    /// and where it lies relative to the start and end of the span.
    ///
    /// This is in the format of:
    /// ```text
    /// [  <blank spaces>  █████<duration of the span>█████───<duration of child spans>───┤   <blank spaces>   ]
    /// ```
    ///
    /// `offset` is the time in microseconds since the start of the trace to this span.
    /// `session_duration` is the total duration of the session.
    pub fn waterfall(&self, config: &Cli, offset: i64, session_duration: i64) -> String {
        let (total_dur, self_dur) = self.durations();
        let e_start = offset;
        let e_end = offset + self_dur;
        let e_tail = offset + total_dur;

        // Calculate positions as a factor of the waterfall width
        let e_start_pos = (e_start as f64 * config.waterfall_width as f64 / session_duration as f64)
            .floor() as usize;
        let e_end_pos = ((e_end as f64 * config.waterfall_width as f64 / session_duration as f64)
            .floor() as usize)
            .max(e_start_pos + 1);
        let e_tail_pos = ((e_tail as f64 * config.waterfall_width as f64 / session_duration as f64)
            .floor() as usize)
            .max(e_start_pos + 1);

        let block_width = e_end_pos - e_start_pos;
        let tail_width = e_tail_pos - e_end_pos;
        let rem_width = config.waterfall_width - e_start_pos - block_width - tail_width;

        let tail = match tail_width {
            0 => "",
            _ => "┤",
        };

        format!(
            "[{:<e_start_pos$}{:█<block_width$}{tail:─>tail_width$}{:<rem_width$}]",
            "", "", ""
        )
    }

    /// Generates a texttual representation of the event to display alongside the waterfall view.
    ///
    /// This contains, by default, the [span duration][Event::duration], [source node IP][Event::source],
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
    pub fn display(
        &self,
        config: &Cli,
        min_activity_width: usize,
        depth: usize,
        max_depth: usize,
    ) -> String {
        let duration = match config.duration_format {
            DurationFormat::Millis => self.duration.num_milliseconds(),
            DurationFormat::Micros => self
                .duration
                .num_microseconds()
                .expect(COMPLAIN_ABOUT_TRACE_SIZE),
        }
        .to_string();
        let source = self.source.to_string();
        let activity = &self.activity;
        let event_id = self.id.to_string();
        let span_id = self.span_id.to_string();
        let parent_span_id = self.parent_span_id.to_string();

        // Activity tree
        let mut tree_bit = format!("{:│>t_depth$}", "├", t_depth = depth + 1);
        if self.is_parent() {
            tree_bit.push('┬');
        }
        let tree = format!("{tree_bit:─<t_depth$}", t_depth = max_depth + 2);

        event_display_str(
            config,
            min_activity_width,
            &duration,
            &source,
            &tree,
            activity,
            &event_id,
            &span_id,
            &parent_span_id,
            &self.thread,
        )
    }

    pub(crate) fn is_parent(&self) -> bool {
        !self.child_events.is_empty()
    }

    pub(crate) fn count_including_children(&self) -> usize {
        1 + self
            .child_events
            .iter()
            .map(|e| e.count_including_children())
            .sum::<usize>()
    }

    pub(crate) fn recurse_events<'a>(&'a self, vec: &mut Vec<(&'a Event, usize)>, depth: usize) {
        vec.extend(std::iter::once((self, depth)));
        for child in &self.child_events {
            child.recurse_events(vec, depth + 1);
        }
    }

    pub(crate) fn try_add_child(&mut self, child_event: Event) -> Result<(), Event> {
        // Base case, the provided event is a direct child of this event
        if child_event.parent_span_id == self.span_id {
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
            duration: Duration::microseconds(value.source_elapsed.into()),
            thread: value.thread(),
            child_events: Vec::new(),
        }
    }
}

/// Generates the formatted string used by [Event::display].
///
/// Here to allow us to re-use the same formatting options for the headers.
#[allow(clippy::too_many_arguments)]
pub fn event_display_str(
    config: &Cli,
    min_activity_width: usize,
    duration: &str,
    source: &str,
    tree: &str,
    activity: &str,
    event_id: &str,
    span_id: &str,
    parent_span_id: &str,
    thread: &str,
) -> String {
    let d_min = config.min_duration_width;
    let a_min = min_activity_width.min(config.max_activity_width);
    let a_max = config.max_activity_width;

    let mut output = format!("{duration:d_min$} {source:15} {tree} {activity:a_min$.a_max$}");

    if config.show_event_id {
        output.push_str(&format!(" {event_id:37}"));
    }
    if config.show_span_ids {
        output.push_str(&format!(" {span_id:20} {parent_span_id:20}"));
    }
    if config.show_thread {
        output.push_str(&format!(" {thread}"));
    }

    output
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
