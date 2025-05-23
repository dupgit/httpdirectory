use crate::entry::Entry;
use chrono::NaiveDate;
use log::trace;
use regex::Regex;
use std::fmt;

/// `HttpDirectoryEntry` represents either the `ParentDirectory`,
/// a `Directory` or a `File` that have a name, link, date and size
/// `Entry`.
#[derive(Debug)]
pub enum HttpDirectoryEntry {
    /// Parent directory with its link (as a `String`) to the effective parent directory
    ParentDirectory(String),

    /// Directory with its `Entry` that collects its data (name, link, date and size)
    Directory(Entry),

    /// File with its `Entry` that collects its data (name, link, date and size)
    File(Entry),
}

impl HttpDirectoryEntry {
    /// Creates a new `HttpDirectoryEntry` entry with name, date, size and link
    /// string slices
    pub(crate) fn new(name: &str, date: &str, size: &str, link: &str) -> Self {
        trace!("name: {name}, date: {date}, size: {size}, link: {link}");
        if name.to_lowercase() == "parent directory" || name.to_lowercase() == "parent directory/" {
            return HttpDirectoryEntry::ParentDirectory(link.to_string());
        }

        let entry = Entry::new(name.trim(), link.trim(), date.trim(), size.trim());

        // `size` may be flipped with `date` so using the one guessed in entry
        // that is likely to be more accurate
        if entry.size().contains("-") {
            HttpDirectoryEntry::Directory(entry)
        } else {
            HttpDirectoryEntry::File(entry)
        }
    }

    /// Tells whether this `HttpDirectoryEntry` represents
    /// a file or not
    #[must_use]
    pub fn is_file(&self) -> bool {
        match self {
            HttpDirectoryEntry::File(_) => true,
            HttpDirectoryEntry::ParentDirectory(_) | HttpDirectoryEntry::Directory(_) => false,
        }
    }

    /// Tells whether this `HttpDirectoryEntry` represents
    /// a directory or not
    #[must_use]
    pub fn is_directory(&self) -> bool {
        match self {
            HttpDirectoryEntry::Directory(_) => true,
            HttpDirectoryEntry::File(_) | HttpDirectoryEntry::ParentDirectory(_) => false,
        }
    }

    /// Tells whether this `HttpDirectoryEntry` represents
    /// a parent directory or not
    #[must_use]
    pub fn is_parent_directory(&self) -> bool {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => true,
            HttpDirectoryEntry::File(_) | HttpDirectoryEntry::Directory(_) => false,
        }
    }

    /// returns true if the regular expression matches
    /// the name of the entry (only for files and directory)
    /// ParentDirectory is never matched.
    #[must_use]
    pub fn is_match_by_name(&self, re: &Regex) -> bool {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => false,
            HttpDirectoryEntry::Directory(dir) => re.is_match(dir.name()),
            HttpDirectoryEntry::File(file) => re.is_match(file.name()),
        }
    }

    /// Returns an `Option` with the name of the file corresponding to the
    /// `HttpDirectoryEntry` if this entry is effectively a file
    /// Returns None otherwise.
    pub fn filename(&self) -> Option<&str> {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) | HttpDirectoryEntry::Directory(_) => None,
            HttpDirectoryEntry::File(file) => Some(file.name()),
        }
    }

    /// Returns an `Option` with the name of the directory corresponding to the
    /// `HttpDirectoryEntry` if this entry is effectively a directory.
    /// Returns None otherwise
    pub fn dirname(&self) -> Option<&str> {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) | HttpDirectoryEntry::File(_) => None,
            HttpDirectoryEntry::Directory(dir) => Some(dir.name()),
        }
    }
}

impl fmt::Display for HttpDirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => write!(f, "DIR  {:>5}  {:>16}  ..", "-", "")?,
            HttpDirectoryEntry::Directory(entry) => write!(f, "DIR  {entry}")?,
            HttpDirectoryEntry::File(entry) => write!(f, "FILE {entry}")?,
        };

        Ok(())
    }
}

/// Helper function to assert a directory entry is what is expected
/// This function is used for testing the library and not intended
/// for any other usage. Compares the size with the apparent size.
pub fn assert_entry(
    dir_entry: &HttpDirectoryEntry,
    parent: bool,
    dir: bool,
    file: bool,
    name: &str,
    size: usize,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minutes: u32,
) {
    // Use cargo t -- --show-output to show outputs while testing
    println!("{dir_entry:?}, {parent}, {dir}, {file}, {name}, {size}, {year}, {month}, {day}, {hour}, {minutes}");
    match dir_entry {
        HttpDirectoryEntry::Directory(entry) => {
            assert!(dir);
            assert_eq!(entry.apparent_size(), size);
            assert_eq!(entry.name(), name);
            assert_eq!(
                entry.date(),
                Some(NaiveDate::from_ymd_opt(year, month, day).unwrap().and_hms_opt(hour, minutes, 0).unwrap())
            );
        }
        HttpDirectoryEntry::File(entry) => {
            assert!(file);
            assert_eq!(entry.apparent_size(), size);
            assert_eq!(entry.name(), name);
            assert_eq!(
                entry.date(),
                Some(NaiveDate::from_ymd_opt(year, month, day).unwrap().and_hms_opt(hour, minutes, 0).unwrap())
            );
        }
        HttpDirectoryEntry::ParentDirectory(link) => {
            assert!(parent);
            assert_eq!(link, name);
        }
    }
}

mod tests {
    use crate::httpdirectory;

    use super::HttpDirectoryEntry;

    #[test]
    fn test_file_httpdirectoryentry_output() {
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-20 20:19", "5.0K", "link/");

        let output = format!("{httpdirectoryentry}");
        assert_eq!(output, "FILE  5.0K  2025-05-20 20:19  name");
        assert_eq!(httpdirectoryentry.filename(), Some("name"));
        assert_eq!(httpdirectoryentry.dirname(), None);
    }

    #[test]
    fn test_dir_httpdirectoryentry_output() {
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-20 20:19", "-", "link/");

        let output = format!("{httpdirectoryentry}");
        assert_eq!(output, "DIR      -  2025-05-20 20:19  name");
        assert_eq!(httpdirectoryentry.dirname(), Some("name"));
        assert_eq!(httpdirectoryentry.filename(), None);
    }

    #[test]
    fn test_parentdir_httpdirectoryentry_output() {
        let httpdirectoryentry = HttpDirectoryEntry::new("Parent directory", "", "-", "../");

        let output = format!("{httpdirectoryentry}");
        assert_eq!(output, "DIR      -                    ..");
    }
}
