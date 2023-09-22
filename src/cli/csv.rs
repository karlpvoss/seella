use clap::Args;
use std::{ffi::OsString, fmt::Display, ops::Deref, path::PathBuf};

/// Options that are specific to the CSV mode of operation.
#[derive(Debug, Args, Clone, Default)]
pub struct CsvModeOptions {
    /// The session id to be visualised
    pub session_id: String,

    /// Path to the CSV containing the sessions data. Any string that can be coerced into a PathBuf
    #[arg(short, long, default_value_t)]
    pub sessions_path: SessionsPath,

    /// Path to the CSV containing the events data. Any string that can be coerced into a PathBuf
    #[arg(short, long, default_value_t)]
    pub events_path: EventsPath,
}

/// Default path to the [Session][crate::SessionRecord] source.
///
/// Type to provide a correct `Default::default()` PathBuf for clap.
#[derive(Debug, Clone)]
pub struct SessionsPath(pub PathBuf);

impl Default for SessionsPath {
    fn default() -> Self {
        Self(PathBuf::from("sessions.csv"))
    }
}

impl Display for SessionsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<OsString> for SessionsPath {
    fn from(value: OsString) -> Self {
        Self(PathBuf::from(value))
    }
}

impl Deref for SessionsPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Default path to the [Event][crate::EventRecord] source.
///
/// Type to provide a correct `Default::default()` PathBuf for clap.
#[derive(Debug, Clone)]
pub struct EventsPath(pub PathBuf);

impl Default for EventsPath {
    fn default() -> Self {
        Self(PathBuf::from("events.csv"))
    }
}

impl Display for EventsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<OsString> for EventsPath {
    fn from(value: OsString) -> Self {
        Self(PathBuf::from(value))
    }
}

impl Deref for EventsPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
