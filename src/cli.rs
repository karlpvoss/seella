use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{
    ffi::OsString, fmt::Display, num::ParseIntError, ops::Deref, path::PathBuf, str::FromStr,
};

//   ___ _    ___
//  / __| |  |_ _|
// | (__| |__ | |
//  \___|____|___|

/// Configuration for the cli.
#[derive(Debug, Parser, Default)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The mode of operation to use, and data source to use
    #[command(subcommand)]
    pub mode: OperationMode,

    /// The width of the waterfall chart
    #[arg(short, long, default_value_t)]
    pub waterfall_width: WaterfallWidth,

    /// Whether to generate span durations in milliseconds or microseconds
    #[arg(value_enum, short, long, default_value_t)]
    pub duration_format: DurationFormat,

    /// Minimum print width for the duration field, remaining will be filled with spaces
    #[arg(long, default_value_t)]
    pub min_duration_width: MinDurationWidth,

    /// Maximum print width for the activity field, remaining will be truncated
    #[arg(long, default_value_t)]
    pub max_activity_width: MaxActivityWidth,

    /// Whether to show the event uuid
    #[arg(long)]
    pub show_event_id: bool,

    /// Whether to show the span ids
    #[arg(long)]
    pub show_span_ids: bool,

    /// Whether to show the thread name
    #[arg(long)]
    pub show_thread: bool,
}

//   ___  ___ ___ ___    _ _____ ___ ___  _  _   __  __  ___  ___  ___
//  / _ \| _ \ __| _ \  /_\_   _|_ _/ _ \| \| | |  \/  |/ _ \|   \| __|
// | (_) |  _/ _||   / / _ \| |  | | (_) | .` | | |\/| | (_) | |) | _|
//  \___/|_| |___|_|_\/_/ \_\_| |___\___/|_|\_| |_|  |_|\___/|___/|___|

/// Mode of operation; whether to use a CSV or a DB.
#[derive(Debug, Subcommand, Clone)]
pub enum OperationMode {
    /// Use a pair of CSVs as a data source
    Csv(CsvModeOptions),
    /// Use a live database (Scylla or Cassandra) as a data source.
    Db,
}

impl Default for OperationMode {
    fn default() -> Self {
        OperationMode::Csv(CsvModeOptions::default())
    }
}

//   ___ _____   __  __  __  ___  ___  ___
//  / __/ __\ \ / / |  \/  |/ _ \|   \| __|
// | (__\__ \\ V /  | |\/| | (_) | |) | _|
//  \___|___/ \_/   |_|  |_|\___/|___/|___|

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

//  ___ ___ ___ ___ ___ ___  _  _ ___   ___  _ _____ _  _
// / __| __/ __/ __|_ _/ _ \| \| / __| | _ \/_\_   _| || |
// \__ \ _|\__ \__ \| | (_) | .` \__ \ |  _/ _ \| | | __ |
// |___/___|___/___/___\___/|_|\_|___/ |_|/_/ \_\_| |_||_|

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

//  _____   _____ _  _ _____ ___   ___  _ _____ _  _
// | __\ \ / / __| \| |_   _/ __| | _ \/_\_   _| || |
// | _| \ V /| _|| .` | | | \__ \ |  _/ _ \| | | __ |
// |___| \_/ |___|_|\_| |_| |___/ |_|/_/ \_\_| |_||_|

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

// __      ___ _____ ___ ___ ___ _   _    _     __      _____ ___ _____ _  _
// \ \    / /_\_   _| __| _ \ __/_\ | |  | |    \ \    / /_ _|   \_   _| || |
//  \ \/\/ / _ \| | | _||   / _/ _ \| |__| |__   \ \/\/ / | || |) || | | __ |
//   \_/\_/_/ \_\_| |___|_|_\_/_/ \_\____|____|   \_/\_/ |___|___/ |_| |_||_|

/// Default width for the waterfall chart.
///
/// Type to provide a correct `Default::default()` usize for clap.
#[derive(Debug, Clone)]
pub struct WaterfallWidth(pub usize);

impl Default for WaterfallWidth {
    fn default() -> Self {
        Self(100)
    }
}

impl Display for WaterfallWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for WaterfallWidth {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(usize::from_str(s)?))
    }
}

impl Deref for WaterfallWidth {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//  ___  _   _ ___    _ _____ ___ ___  _  _   ___ ___  ___ __  __   _ _____
// |   \| | | | _ \  /_\_   _|_ _/ _ \| \| | | __/ _ \| _ \  \/  | /_\_   _|
// | |) | |_| |   / / _ \| |  | | (_) | .` | | _| (_) |   / |\/| |/ _ \| |
// |___/ \___/|_|_\/_/ \_\_| |___\___/|_|\_| |_| \___/|_|_\_|  |_/_/ \_\_|

/// Which unit of measurement to use for the display of durations of spans.
#[derive(Debug, Default, Clone, ValueEnum)]
pub enum DurationFormat {
    Millis,
    #[default]
    Micros,
}

//  __  __ ___ _  _   ___  _   _ ___    _ _____ ___ ___  _  _  __      _____ ___ _____ _  _
// |  \/  |_ _| \| | |   \| | | | _ \  /_\_   _|_ _/ _ \| \| | \ \    / /_ _|   \_   _| || |
// | |\/| || || .` | | |) | |_| |   / / _ \| |  | | (_) | .` |  \ \/\/ / | || |) || | | __ |
// |_|  |_|___|_|\_| |___/ \___/|_|_\/_/ \_\_| |___\___/|_|\_|   \_/\_/ |___|___/ |_| |_||_|

/// Default minimum width for the `dur` column.
///
/// Type to provide a correct `Default::default()` usize for clap.
#[derive(Debug, Clone)]
pub struct MinDurationWidth(pub usize);

impl Default for MinDurationWidth {
    fn default() -> Self {
        Self(6)
    }
}

impl Display for MinDurationWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MinDurationWidth {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(usize::from_str(s)?))
    }
}

impl Deref for MinDurationWidth {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//  __  __   _   __  __    _   ___ _____ _____   _____ _______   __ __      _____ ___ _____ _  _
// |  \/  | /_\  \ \/ /   /_\ / __|_   _|_ _\ \ / /_ _|_   _\ \ / / \ \    / /_ _|   \_   _| || |
// | |\/| |/ _ \  >  <   / _ \ (__  | |  | | \ V / | |  | |  \ V /   \ \/\/ / | || |) || | | __ |
// |_|  |_/_/ \_\/_/\_\ /_/ \_\___| |_| |___| \_/ |___| |_|   |_|     \_/\_/ |___|___/ |_| |_||_|

/// Default maximum width for the `activity` column.
///
/// Type to provide a correct `Default::default()` usize for clap.
#[derive(Debug, Clone)]
pub struct MaxActivityWidth(pub usize);

impl Default for MaxActivityWidth {
    fn default() -> Self {
        Self(300)
    }
}

impl Display for MaxActivityWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MaxActivityWidth {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(usize::from_str(s)?))
    }
}

impl Deref for MaxActivityWidth {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
