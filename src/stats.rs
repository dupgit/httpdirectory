use crate::httpdirectoryentry::HttpDirectoryEntry;
use std::fmt;

#[derive(Default, Debug)]
pub struct Stats {
    pub parent_dir: u8,
    pub dirs: u32,
    pub files: u32,
    pub total_size: u64,
    pub with_date: u32,
    pub without_date: u32,
}

impl Stats {
    pub fn new() -> Self {
        Stats::default()
    }

    pub fn count(&mut self, entry: &HttpDirectoryEntry) -> &Self {
        match entry {
            HttpDirectoryEntry::ParentDirectory(_) => {
                self.parent_dir += 1;
                self.without_date += 1;
            }
            HttpDirectoryEntry::Directory(dir) => {
                self.dirs += 1;
                match dir.date() {
                    Some(_) => self.with_date += 1,
                    None => self.without_date += 1,
                }
            }
            HttpDirectoryEntry::File(file) => {
                self.files += 1;
                self.total_size += file.apparent_size() as u64;
                match file.date() {
                    Some(_) => self.with_date += 1,
                    None => self.without_date += 1,
                }
            }
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

mod test {
    use crate::httpdirectoryentry::HttpDirectoryEntry;

    use super::Stats;

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
}
