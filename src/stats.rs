use crate::entry::Entry;
use std::fmt;

/// Gives statistics about an `HttpDirectoryEntry`
#[derive(Default, Debug)]
pub struct Stats {
    /// number of parent directory (there should only be one)
    pub parent_dir: u8,

    /// number of directories
    pub dirs: u32,

    /// number of files
    pub files: u32,

    /// sum of the size of each file in the directory
    pub total_size: u64,

    /// number of files and directories with guessed date
    pub with_date: u32,

    /// number of files and directories without guessed date
    pub without_date: u32,
}

impl Stats {
    pub(crate) fn new() -> Self {
        Stats::default()
    }

    pub(crate) fn add_parent_directory(&mut self) -> &Self {
        self.parent_dir += 1;
        self.without_date += 1;
        self
    }

    pub(crate) fn add_directory(&mut self, dir: &Entry) -> &Self {
        self.dirs += 1;
        match dir.date() {
            Some(_) => self.with_date += 1,
            None => self.without_date += 1,
        }
        self
    }

    pub(crate) fn add_file(&mut self, file: &Entry) -> &Self {
        self.files += 1;
        self.total_size += file.size() as u64;
        match file.date() {
            Some(_) => self.with_date += 1,
            None => self.without_date += 1,
        }
        self
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Parent directory: {}", self.parent_dir)?;
        writeln!(f, "Directories: {}", self.dirs)?;
        writeln!(f, "Files: {}", self.files)?;
        writeln!(f, "Total apparent file sizes: {}", self.total_size)?;
        writeln!(f, "Entries with dates: {}", self.with_date)?;
        writeln!(f, "Entries without any date: {}", self.without_date)?;
        Ok(())
    }
}

// This module is tested in httpdirectory module.
