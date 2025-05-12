use crate::entry::Entry;
use log::trace;
use std::fmt;

/// HttpDirectoryEntry represents either the ParentDirectory,
/// a Directory or a File that have a name, link, date and size
/// Entry.
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
    /// Creates a new HttpDirectoryEntry entry with name, date, size and link
    /// string slices
    pub(crate) fn new(name: &str, date: &str, size: &str, link: &str) -> Self {
        trace!("name: {name}, date: {date}, size: {size}, link: {link}");
        if name.to_lowercase() == "parent directory" {
            return HttpDirectoryEntry::ParentDirectory(link.to_string());
        }

        let entry = Entry::new(name.trim(), link.trim(), date.trim(), size.trim());

        if size.contains(" - ") {
            return HttpDirectoryEntry::Directory(entry);
        } else {
            return HttpDirectoryEntry::File(entry);
        }
    }

    /// Tells whether this HttpDirectoryEntry represents
    /// a file or not
    pub fn is_file(&self) -> bool {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => false,
            HttpDirectoryEntry::Directory(_) => false,
            HttpDirectoryEntry::File(_) => true,
        }
    }

    /// Tells whether this HttpDirectoryEntry represents
    /// a directory or not
    pub fn is_directory(&self) -> bool {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => false,
            HttpDirectoryEntry::Directory(_) => true,
            HttpDirectoryEntry::File(_) => false,
        }
    }

    /// Tells whether this HttpDirectoryEntry represents
    /// a parent directory or not
    pub fn is_parent_directory(&self) -> bool {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => true,
            HttpDirectoryEntry::Directory(_) => false,
            HttpDirectoryEntry::File(_) => false,
        }
    }
}

impl fmt::Display for HttpDirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => write!(f, "DIR  {:>5}  {:>16}  {}", "-", "", "..")?,
            HttpDirectoryEntry::Directory(entry) => write!(f, "DIR  {entry}")?,
            HttpDirectoryEntry::File(entry) => write!(f, "FILE {entry}")?,
        };

        Ok(())
    }
}
