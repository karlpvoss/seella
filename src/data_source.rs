use crate::records::{EventRecord, SessionRecord};

/// Generalises the ability to get the [SessionRecord] and [EventRecords][EventRecord] from anywhere.
pub trait DataSource {
    type Error;
    type P;

    fn get_data(&self) -> Result<(SessionRecord<Self::P>, Vec<EventRecord>), Self::Error>;
}
