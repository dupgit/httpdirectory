/// HttpDirectoryEntry represents either the ParentDirectory,
/// a Directory or a File that have a name, date and size
/// Entry.
use crate::entry::Entry;

#[derive(Debug)]
pub enum HttpDirectoryEntry {
    ParentDirectory,
    Directory(Entry),
    File(Entry),
}
