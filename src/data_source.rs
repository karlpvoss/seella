use crate::records::{EventRecord, SessionRecord};

pub type DataSourceResult<T, E> = Result<(SessionRecord<T>, Vec<EventRecord>), E>;

/// Generalises the ability to get the [SessionRecord] and [EventRecords][EventRecord] from anywhere.
pub trait DataSource {
    type Error;
    type P;

    #[allow(clippy::type_complexity)]
    fn get_data(&self) -> Result<(SessionRecord<Self::P>, Vec<EventRecord>), Self::Error>;
}
