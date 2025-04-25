use chrono::NaiveDateTime;

/// Defines an Entry for a file or a directory
#[derive(Debug)]
pub struct Entry {
    name: String,
    date: NaiveDateTime,
    size: usize,
}
