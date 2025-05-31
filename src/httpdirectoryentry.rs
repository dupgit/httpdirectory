use crate::entry::Entry;
use crate::httpdirectory::Sorting;
use chrono::NaiveDate;
use log::trace;
use regex::Regex;
use std::cmp::Ordering;
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

pub enum CompareField {
    Name,
    Date,
    Size,
}

impl HttpDirectoryEntry {
    /// Creates a new `HttpDirectoryEntry` entry with name, date, size and link
    /// string slices
    pub(crate) fn new(name: &str, date: &str, size: &str, link: &str) -> Self {
        trace!("name: {name}, date: {date}, size: {size}, link: {link}");
        if name.to_lowercase() == "parent directory"
            || name.to_lowercase() == "parent directory/"
            || link.to_lowercase() == ".."
            || link.to_lowercase() == "../"
        {
            return HttpDirectoryEntry::ParentDirectory(link.to_string());
        }

        let entry = Entry::new(name.trim(), link.trim(), date.trim(), size.trim());

        // `size` may be flipped with `date` so using the one guessed in entry
        // that is likely to be more accurate
        if entry.size().contains('-') || entry.size().contains('—') {
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
    /// `ParentDirectory` is never matched.
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
    #[must_use]
    pub fn filename(&self) -> Option<&str> {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) | HttpDirectoryEntry::Directory(_) => None,
            HttpDirectoryEntry::File(file) => Some(file.name()),
        }
    }

    /// Returns an `Option` with the name of the directory corresponding to the
    /// `HttpDirectoryEntry` if this entry is effectively a directory.
    /// Returns None otherwise
    #[must_use]
    pub fn dirname(&self) -> Option<&str> {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) | HttpDirectoryEntry::File(_) => None,
            HttpDirectoryEntry::Directory(dir) => Some(dir.name()),
        }
    }

    /// Compares entries by the selected field from `CompareField` enum using
    /// a sorting order as of `Sorting` enum
    #[must_use]
    pub fn cmp_by_field(&self, other: &Self, field: &CompareField, order: &Sorting) -> Ordering {
        match (self, other) {
            (
                HttpDirectoryEntry::ParentDirectory(_),
                HttpDirectoryEntry::File(_) | HttpDirectoryEntry::Directory(_),
            ) => Ordering::Less,
            (
                HttpDirectoryEntry::File(_) | HttpDirectoryEntry::Directory(_),
                HttpDirectoryEntry::ParentDirectory(_),
            ) => Ordering::Greater,
            (HttpDirectoryEntry::ParentDirectory(_), HttpDirectoryEntry::ParentDirectory(_)) => Ordering::Equal,
            (
                HttpDirectoryEntry::File(entry) | HttpDirectoryEntry::Directory(entry),
                HttpDirectoryEntry::File(other_entry) | HttpDirectoryEntry::Directory(other_entry),
            ) => match field {
                CompareField::Name => entry.cmp_by_name(other_entry, order),
                CompareField::Date => entry.cmp_by_date(other_entry, order),
                CompareField::Size => entry.cmp_by_size(other_entry, order),
            },
        }
    }
}

impl fmt::Display for HttpDirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => write!(f, "DIR  {:>5}  {:>16}  ..", "-", "")?,
            HttpDirectoryEntry::Directory(entry) => write!(f, "DIR  {entry}")?,
            HttpDirectoryEntry::File(entry) => write!(f, "FILE {entry}")?,
        }
        Ok(())
    }
}

/// # Panics
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
    use super::HttpDirectoryEntry;
    use crate::{
        httpdirectory::{self, Sorting},
        httpdirectoryentry::CompareField,
    };
    use std::cmp::Ordering;

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

    #[test]
    fn test_httpdirectoryentry_parent_directory_cmp_by_name() {
        let parent1 = HttpDirectoryEntry::new("Parent directory", "", "-", "../");
        let parent2 = HttpDirectoryEntry::new("Parent directory", "", "-", "../");

        assert_eq!(parent1.cmp_by_field(&parent2, &CompareField::Name, &Sorting::Ascending), Ordering::Equal);
        assert_eq!(parent1.cmp_by_field(&parent2, &CompareField::Name, &Sorting::Descending), Ordering::Equal);
    }

    #[test]
    fn test_httpdirectoryentry_file_parent_directory_cmp_by_name() {
        let parent1 = HttpDirectoryEntry::new("Parent directory", "", "-", "../");
        let file2 = HttpDirectoryEntry::new("filename", "2025-05-20 20:19", "5.0K", "filelink/");

        assert_eq!(parent1.cmp_by_field(&file2, &CompareField::Name, &Sorting::Ascending), Ordering::Less);
        assert_eq!(file2.cmp_by_field(&parent1, &CompareField::Name, &Sorting::Ascending), Ordering::Greater);

        // Ordering with a parent directory should not change: the parent directory is always at top
        assert_eq!(parent1.cmp_by_field(&file2, &CompareField::Name, &Sorting::Descending), Ordering::Less);
        assert_eq!(file2.cmp_by_field(&parent1, &CompareField::Name, &Sorting::Descending), Ordering::Greater);
    }

    #[test]
    fn test_httpdirectoryentry_dir_parent_directory_cmp_by_name() {
        let parent1 = HttpDirectoryEntry::new("Parent directory", "", "-", "../");
        let dir2 = HttpDirectoryEntry::new("dirname", "2025-05-20 20:19", "-", "dirlink/");

        assert_eq!(parent1.cmp_by_field(&dir2, &CompareField::Name, &Sorting::Ascending), Ordering::Less);
        assert_eq!(dir2.cmp_by_field(&parent1, &CompareField::Name, &Sorting::Ascending), Ordering::Greater);

        // Ordering with a parent directory should not change: the parent directory is always at top
        assert_eq!(parent1.cmp_by_field(&dir2, &CompareField::Name, &Sorting::Descending), Ordering::Less);
        assert_eq!(dir2.cmp_by_field(&parent1, &CompareField::Name, &Sorting::Descending), Ordering::Greater);
    }

    #[test]
    fn test_httpdirectoryentry_file_cmp_by_date() {
        let file1 = HttpDirectoryEntry::new("name", "2025-04-20 18:55", "5.0K", "link/");
        let file2 = HttpDirectoryEntry::new("other name", "2025-05-20 20:19", "12G", "other_name/");

        assert_eq!(file1.cmp_by_field(&file2, &CompareField::Date, &Sorting::Ascending), Ordering::Less);
        assert_eq!(file2.cmp_by_field(&file1, &CompareField::Date, &Sorting::Ascending), Ordering::Greater);

        // Here comparing two files the ordering must change
        assert_eq!(file1.cmp_by_field(&file2, &CompareField::Date, &Sorting::Descending), Ordering::Greater);
        assert_eq!(file2.cmp_by_field(&file1, &CompareField::Date, &Sorting::Descending), Ordering::Less);
    }

    #[test]
    fn test_httpdirectoryentry_file_cmp_by_size() {
        let file1 = HttpDirectoryEntry::new("name", "2025-04-20 18:55", "5.0K", "link/");
        let file2 = HttpDirectoryEntry::new("other name", "2025-05-20 20:19", "12G", "other_name/");

        assert_eq!(file1.cmp_by_field(&file2, &CompareField::Size, &Sorting::Ascending), Ordering::Less);
        assert_eq!(file2.cmp_by_field(&file1, &CompareField::Size, &Sorting::Ascending), Ordering::Greater);

        // Here comparing two files the ordering must change
        assert_eq!(file1.cmp_by_field(&file2, &CompareField::Size, &Sorting::Descending), Ordering::Greater);
        assert_eq!(file2.cmp_by_field(&file1, &CompareField::Size, &Sorting::Descending), Ordering::Less);
    }
}
