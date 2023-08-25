use std::path::PathBuf;

pub fn test_data_sessions() -> PathBuf {
    PathBuf::from(format!(
        "{}/tests/data/sessions.csv",
        env!("CARGO_MANIFEST_DIR")
    ))
}

pub fn test_data_events() -> PathBuf {
    PathBuf::from(format!(
        "{}/tests/data/events.csv",
        env!("CARGO_MANIFEST_DIR")
    ))
}
