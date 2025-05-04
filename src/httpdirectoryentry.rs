use crate::entry::Entry;
use log::trace;
use std::fmt;

/// HttpDirectoryEntry represents either the ParentDirectory,
/// a Directory or a File that have a name, date and size
/// Entry.
#[derive(Debug)]
pub enum HttpDirectoryEntry {
    ParentDirectory(String),
    Directory(Entry),
    File(Entry),
}

impl HttpDirectoryEntry {
    pub fn new(name: &str, date: &str, size: &str, link: &str) -> Self {
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
}

impl fmt::Display for HttpDirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpDirectoryEntry::ParentDirectory(_) => write!(f, "DIR  0  ..")?,
            HttpDirectoryEntry::Directory(entry) => write!(f, "DIR  {entry}")?,
            HttpDirectoryEntry::File(entry) => write!(f, "FILE {entry}")?,
        };

        Ok(())
    }
}
