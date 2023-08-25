use std::path::PathBuf;

pub fn test_data(filename: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}/tests/data/{}",
        env!("CARGO_MANIFEST_DIR"),
        filename
    ))
}
