use crate::records::{EventRecord, SessionRecord};

/// Generalises the ability to get the [SessionRecord] and [EventRecords][EventRecord] from anywhere.
pub trait DataSource {
    type Error;

    fn get_data(&self) -> Result<(SessionRecord, Vec<EventRecord>), Self::Error>;
}
