use crate::{entry::Entry, httpdirectoryentry::HttpDirectoryEntry};
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

    pub(crate) fn count(&mut self, entry: &HttpDirectoryEntry) -> &Self {
        match entry {
            HttpDirectoryEntry::ParentDirectory(_) => self.add_parent_directory(),
            HttpDirectoryEntry::Directory(dir) => self.add_directory(dir),
            HttpDirectoryEntry::File(file) => self.add_file(file),
        }
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

#[cfg(test)]
mod test {
    use {super::Stats, crate::httpdirectoryentry::HttpDirectoryEntry};

    #[test]
    fn test_stats_new() {
        let stats = Stats::new();

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 0);
        assert_eq!(stats.files, 0);
        assert_eq!(stats.total_size, 0);
        assert_eq!(stats.with_date, 0);
        assert_eq!(stats.total_size, 0);
    }

    #[test]
    fn test_stats_count() {
        let mut stats = Stats::new();
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-31 16:58", "-", "name/");
        stats.count(&httpdirectoryentry);

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 0);
        assert_eq!(stats.total_size, 0);
        assert_eq!(stats.with_date, 1);
        assert_eq!(stats.without_date, 0);

        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-31 16:58", "3.1K", "name/");
        stats.count(&httpdirectoryentry);

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 1);
        assert_eq!(stats.total_size, 3174);
        assert_eq!(stats.with_date, 2);
        assert_eq!(stats.without_date, 0);

        let httpdirectoryentry = HttpDirectoryEntry::new("Parent Directory", "2025-05-31 16:58", "-", "../");
        stats.count(&httpdirectoryentry);

        assert_eq!(stats.parent_dir, 1);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 1);
        assert_eq!(stats.total_size, 3174);
        assert_eq!(stats.with_date, 2);
        assert_eq!(stats.without_date, 1);
    }

    #[test]
    fn test_stats_output() {
        let mut stats = Stats::new();
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-31 16:58", "3.1K", "name/");
        stats.count(&httpdirectoryentry);
        let output = r##"Parent directory: 0
Directories: 0
Files: 1
Total apparent file sizes: 3174
Entries with dates: 1
Entries without any date: 0
"##;

        assert_eq!(stats.to_string(), output);
    }

    #[test]
    fn test_stats_count_no_date() {
        let mut stats = Stats::new();
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "", "-", "name/");
        stats.count(&httpdirectoryentry);

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 0);
        assert_eq!(stats.total_size, 0);
        assert_eq!(stats.with_date, 0);
        assert_eq!(stats.without_date, 1);

        let httpdirectoryentry = HttpDirectoryEntry::new("name", "", "3.1K", "name/");
        stats.count(&httpdirectoryentry);

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 1);
        assert_eq!(stats.total_size, 3174);
        assert_eq!(stats.with_date, 0);
        assert_eq!(stats.without_date, 2);
    }
}
