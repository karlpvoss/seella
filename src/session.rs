use crate::{
    event::Event,
    records::{EventRecord, SessionRecord},
    Cli,
};
use chrono::{DateTime, Duration, Utc};
use std::{collections::VecDeque, fmt::Debug, net::IpAddr};
use uuid::Uuid;

/// All of the information related to a single tracing session.
///
/// This is effectively:
///
/// ```text
/// SELECT * FROM system_traces.sessions WHERE session_id=227aff60-4f21-11e6-8835-000000000000
/// JOIN
/// SELECT * FROM system_traces.events WHERE session_id=227aff60-4f21-11e6-8835-000000000000
/// ```
///
/// This gives us all possible tracing information for a single session, where that session may be a single query,
/// or some other command.
///
/// [Events][Event] can be accessed through the [Session::events()] method, and these will be presented depth-first;
/// i.e. we provide the children of the first root trace before moving on to the second root trace.
#[derive(Debug)]
pub struct Session {
    /// The UUID of the Session
    pub id: Uuid,
    /// The IP address of the connecting client
    pub client: IpAddr,
    /// Currently, this can only be "QUERY"
    pub command: String,
    /// The IP address of the coordinating Scylla Node
    pub coordinator: IpAddr,
    /// Total duration of the Session
    pub duration: Duration,
    // TODO FIX DOCS
    /// A scylla map containing string pairs that describe the query
    // Not currently parsing as a HashMap<String, String> due to issues with quoting
    pub parameters: String,
    /// A short string decribing the Session. Is _not_ the CQL query being ran; that is in `parameters`.
    pub request: String,
    /// DateTime of the start of this tracing session
    pub started_at: DateTime<Utc>,

    /// Size of the request
    /// Since Scylla 3.0
    /// Not present in Cassandra
    pub request_size: Option<u32>,
    /// Size of the response
    /// Since Scylla 3.0
    /// Not present in Cassandra
    pub response_size: Option<u32>,
    /// The username associated with the request? Lacking documentation.
    /// Not present in Cassandra
    pub username: Option<String>,

    root_events: Vec<Event>,
}

impl Session {
    pub(crate) fn new(session_record: SessionRecord, event_records: Vec<EventRecord>) -> Self {
        let (mut root_events, mut child_events): (VecDeque<Event>, VecDeque<Event>) = event_records
            .into_iter()
            .map(Event::from)
            .partition(|event| event.parent_span_id.is_root());

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

        Self {
            id: session_record.session_id,
            client: session_record.client,
            command: session_record.command,
            coordinator: session_record.coordinator,
            duration: Duration::microseconds(session_record.duration.into()),
            parameters: session_record.parameters,
            request: session_record.request,
            request_size: session_record.request_size,
            response_size: session_record.response_size,
            started_at: session_record.started_at,
            username: session_record.username,
            root_events: root_events.into(),
        }
    }

    /// Recurses the tree of [events][Event] without needing to allocate or otherwise work too hard.
    pub fn event_count(&self) -> usize {
        self.root_events
            .iter()
            .map(|e| e.count_including_children())
            .sum::<usize>()
    }

    /// Depth-first recursion of all events in the tree.
    pub fn events(&self) -> Vec<(&Event, usize)> {
        let count = self.event_count();
        let mut events = Vec::with_capacity(count);

        for root_event in &self.root_events {
            root_event.recurse_events(&mut events, 0);
        }

        events
    }

    /// Returns the total duration of the trace.
    ///
    /// Given by summing the total durations of all root traces.
    pub fn total_duration(&self) -> i64 {
        self.root_events.iter().map(|e| e.durations().0).sum()
    }

    pub fn display(&self, cli: Cli, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Print out the session info
        writeln!(w, "Session ID: {}", &self.id)?;
        writeln!(w, "{}", &self.started_at.to_rfc3339())?;
        writeln!(
            w,
            "{:15} ({}) -> {:15}",
            &self.client,
            &self.username.clone().unwrap_or_else(|| String::from("N/A")),
            &self.coordinator
        )?;
        writeln!(
            w,
            "Request Size:  {}",
            &self
                .request_size
                .map(|rs| rs.to_string())
                .unwrap_or_else(|| String::from("N/A"))
        )?;
        writeln!(
            w,
            "Response Size: {}",
            &self
                .response_size
                .map(|rs| rs.to_string())
                .unwrap_or_else(|| String::from("N/A"))
        )?;
        writeln!(w, "{}", &self.request)?;
        writeln!(w, "{:?}", &self.parameters)?;

        // Calculations for the waterfall boxes
        let s_end = self.total_duration();
        let mut offset = 0i64;

        let events = self.events();
        let a_max_width = events
            .iter()
            .map(|(e, _)| e.activity_length())
            .max()
            .unwrap_or(0);
        let max_depth = events.iter().map(|(_, depth)| *depth).max().unwrap_or(1);
        let i_max_width = self.event_count().to_string().len();
        let w_width = *cli.waterfall_width + 2;

        // Headers
        writeln!(w)?;
        writeln!(
            w,
            "{:i_max_width$} {:w_width$} {}",
            "",
            "waterfall chart",
            crate::event_display_str(
                &cli,
                a_max_width,
                "dur",
                "node",
                &format!("{:tree_width$}", "", tree_width = max_depth + 2),
                "activity",
                "event id",
                "span id",
                "parent span id",
                "thread name",
            )
        )?;

        // print out the actual chart and details
        for (i, (e, depth)) in events.iter().enumerate() {
            writeln!(
                w,
                "{:i_max_width$} {} {}",
                i + 1,
                e.waterfall(&cli, offset, s_end),
                e.display(&cli, a_max_width, *depth, max_depth)
            )?;

            // Move the offset up for the next event
            offset += e.durations().1;
        }

        // total duration information
        let total_dur = self.total_duration().to_string();
        let total_dur_width = total_dur.len();
        writeln!(
            w,
            "{:i_max_width$} {:w_width$} {}",
            "",
            "",
            crate::event_display_str(
                &cli,
                a_max_width,
                &format!("{:-<total_dur_width$}", ""),
                "",
                "",
                "",
                "",
                "",
                "",
                "",
            )
        )?;
        writeln!(
            w,
            "{:i_max_width$} {:w_width$} {}",
            "",
            "",
            crate::event_display_str(&cli, a_max_width, &total_dur, "", "", "", "", "", "", "",)
        )?;

        Ok(())
    }
}
