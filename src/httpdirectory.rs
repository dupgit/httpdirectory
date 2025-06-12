use crate::error::HttpDirError;
use crate::httpdirectoryentry::{CompareField, HttpDirectoryEntry};
use crate::requests::{Request, join_url};
use crate::scrape::scrape_body;
use crate::stats::Stats;
use log::{debug, error};
use regex::Regex;
use std::fmt;
use std::sync::Arc;

/// Main structure that provides methods to access, parse a directory
/// webpage and fill that structure.
#[derive(Debug, Clone)]
pub struct HttpDirectory {
    entries: Vec<HttpDirectoryEntry>,
    url: Arc<String>,
    request: Arc<Request>,
}

pub enum Sorting {
    Ascending,
    Descending,
}

// @todo: ? implement an iterator ?
impl HttpDirectory {
    /// Crawls the `url` and returns (if no error occurred) the
    /// `HttpDirectory` of that url
    pub async fn new(url: &str) -> Result<Self, HttpDirError> {
        let client = Request::new()?;
        let response = client.get(url).await?;

        let entries = get_entries_from_body(&response.text().await?);
        Ok(HttpDirectory {
            entries,
            url: Arc::new(url.to_string()),
            request: Arc::new(client),
        })
    }

    /// Change directory if possible to dir (from url) and gets the new
    /// `HttpDirectory` listing if any and returns it.
    pub async fn cd(mut self, dir: &str) -> Result<Self, HttpDirError> {
        let url = join_url(&self.url, dir)?;
        debug!("cd is going to {url}");
        let response = self.request.get(&url).await?;
        let entries = get_entries_from_body(&response.text().await?);
        self.entries = entries;
        self.url = Arc::new(url);
        Ok(self)
    }

    /// Sorts the Directory entries by their names
    #[must_use]
    pub fn sort_by_name(mut self, order: Sorting) -> Self {
        self.entries.sort_by(|a, b| a.cmp_by_field(b, &CompareField::Name, &order));
        self
    }

    /// Sorts the Directory entries by their dates
    #[must_use]
    pub fn sort_by_date(mut self, order: Sorting) -> Self {
        self.entries.sort_by(|a, b| a.cmp_by_field(b, &CompareField::Date, &order));
        self
    }

    /// Sorts the Directory entries by their sizes
    #[must_use]
    pub fn sort_by_size(mut self, order: Sorting) -> Self {
        self.entries.sort_by(|a, b| a.cmp_by_field(b, &CompareField::Size, &order));
        self
    }

    /// Returns only elements of the `HttpDirectory` listing that
    /// matches the predicate f. An element of this predicate is
    /// of type `HttpDirectoryEntry`
    #[must_use]
    pub fn filtering<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&HttpDirectoryEntry) -> bool,
    {
        let mut entries = self.entries.clone();
        entries.retain(|elem| f(elem));

        HttpDirectory {
            entries,
            url: self.url.clone(),
            request: self.request.clone(),
        }
    }

    /// Returns only directories of the `HttpDirectory` listing
    #[must_use]
    pub fn dirs(&self) -> Self {
        self.filtering(|e| e.is_directory())
    }

    /// Returns only files of the `HttpDirectory` listing
    #[must_use]
    pub fn files(&self) -> Self {
        self.filtering(|e| e.is_file())
    }

    /// Returns only the parent directory the `HttpDirectory` listing
    #[must_use]
    pub fn parent_directory(&self) -> Self {
        self.filtering(|e| e.is_parent_directory())
    }

    /// Returns the last entry Some(`HttpDirectoryEntry`) of that `HttpDirectory`
    /// if it exists or None
    pub fn last(&self) -> Option<&HttpDirectoryEntry> {
        self.entries.last()
    }

    /// Returns the first entry Some(`HttpDirectoryEntry`) of that `HttpDirectory`
    /// if it exists or None
    pub fn first(&self) -> Option<&HttpDirectoryEntry> {
        self.entries.first()
    }

    /// Returns the `Stats` (ie the number of files (with total
    /// apparent size), directories and parent directories) of
    /// the `HttpDirectory` structure
    pub fn stats(&self) -> Stats {
        let mut stats = Stats::new();
        for e in self.entries() {
            stats.count(e);
        }
        stats
    }

    /// Filters the `HttpDirectory` listing by filtering names of each
    /// entry with the `regex` regular expression.
    pub fn filter_by_name(&self, regex: &str) -> Result<Self, HttpDirError> {
        let re = Regex::new(regex)?;
        Ok(self.filtering(|e| e.is_match_by_name(&re)))
    }

    /// Tells whether the `HttpDirectory` listing is empty or not
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the length of the `HttpDirectory` listing
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns all entries of the `HttpDirectory` listing as a
    /// vector of `HttpDirectoryEntry`
    #[must_use]
    pub fn entries(&self) -> &Vec<HttpDirectoryEntry> {
        &self.entries
    }
}

impl fmt::Display for HttpDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", &self.url)?;
        for entry in &self.entries {
            writeln!(f, "{entry}")?;
        }
        Ok(())
    }
}

impl Default for HttpDirectory {
    /// Returns an `HttpDirectory` initialized with default
    /// values (empty vector, empty url and no `HttpEngine`)
    fn default() -> Self {
        HttpDirectory {
            entries: vec![],
            url: Arc::new(String::new()),
            request: Arc::new(Request::None),
        }
    }
}

fn get_entries_from_body(body: &str) -> Vec<HttpDirectoryEntry> {
    match scrape_body(body) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Error getting entries: {e}");
            vec![]
        }
    }
}

mod tests {
    use super::{HttpDirectory, HttpDirectoryEntry, Request};
    use crate::{httpdirectory::Sorting, httpdirectoryentry::EntryType, httpdirectoryentry::assert_entry};
    use std::sync::Arc;

    #[test]
    fn test_httpdirectory_default() {
        let httpdir = HttpDirectory::default();
        assert!(httpdir.is_empty());
        assert_eq!(httpdir.url, Arc::new("".to_string()));

        match Arc::into_inner(httpdir.request) {
            Some(Request::Reqwest(request)) => panic!("{request:?} should be None"),
            Some(Request::None) => (),
            None => panic!("Arc should return Some(Request::None) and not None"),
        }
    }

    #[tokio::test]
    async fn test_httpdirectory_no_base_url() {
        let httpdir = HttpDirectory::default();

        match httpdir.cd("/dir").await {
            Ok(_) => panic!("This test should return Err()"),
            Err(e) => assert_eq!(e.to_string(), "Error: relative URL without a base"),
        }
    }

    // This is an helper function used to prepare some `HttpDirectory`
    // structure that will be used in tests
    fn prepare_httpdir() -> HttpDirectory {
        let mut httpdir = HttpDirectory::default();

        httpdir.entries.push(HttpDirectoryEntry::new("dir1", "2025-01-26 12:54", "-", "dir1/"));
        httpdir.entries.push(HttpDirectoryEntry::new("test2", "2025-02-16 13:37", "-", "test2/"));
        httpdir.entries.push(HttpDirectoryEntry::new("debian3", "2025-03-01 07:11", "-", "debian3/"));
        httpdir.entries.push(HttpDirectoryEntry::new("entry4", "2025-01-02 12:32", "-", "entry4/"));
        httpdir.entries.push(HttpDirectoryEntry::new("file1", "1987-10-09 04:37", "123", "file1/"));
        httpdir.entries.push(HttpDirectoryEntry::new("files2", "2023-01-01 00:00", "2345", "files2/"));
        httpdir.entries.push(HttpDirectoryEntry::new("entry3", "2025-07-17 23:59", "67K", "entry3/"));
        httpdir.entries.push(HttpDirectoryEntry::new("debian4", "2024-12-08 08:22", "123M", "debian4/"));
        httpdir.entries.push(HttpDirectoryEntry::new("parent directory", "2025-01-26 12:54", "-", "../"));

        httpdir
    }

    #[test]
    fn test_httpdirectory_first_and_last() {
        let httpdir = prepare_httpdir();

        if let Some(entry) = httpdir.first() {
            assert_entry(entry, &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        } else {
            panic!("This test should return an entry");
        }

        if let Some(entry) = httpdir.last() {
            assert_entry(entry, &EntryType::ParentDirectory, "../", 0, "2025-01-26 12:54");
        } else {
            panic!("This test should return an entry");
        }

        let httpdir = httpdir.files();
        if let Some(entry) = httpdir.first() {
            assert_entry(entry, &EntryType::File, "file1", 123, "1987-10-09 04:37");
        } else {
            panic!("This test should return an entry");
        }

        if let Some(entry) = httpdir.last() {
            assert_entry(entry, &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
        } else {
            panic!("This test should return an entry");
        }
    }

    #[test]
    fn test_httpdirectory_dirs() {
        let httpdir = prepare_httpdir().dirs();
        assert!(!httpdir.is_empty());
        assert_eq!(httpdir.len(), 4);

        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        assert_entry(&entries[1], &EntryType::Directory, "test2", 0, "2025-02-16 13:37");
        assert_entry(&entries[2], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[3], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");

        let output = format!("{httpdir}");
        assert_eq!(
            output,
            r##"
DIR         -  2025-01-26 12:54  dir1
DIR         -  2025-02-16 13:37  test2
DIR         -  2025-03-01 07:11  debian3
DIR         -  2025-01-02 12:32  entry4
"##
        );
    }

    #[test]
    fn test_httpdirectory_files() {
        let httpdir = prepare_httpdir().files();
        assert_eq!(httpdir.len(), 4);

        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::File, "file1", 123, "1987-10-09 04:37");
        assert_entry(&entries[1], &EntryType::File, "files2", 2_345, "2023-01-01 00:00");
        assert_entry(&entries[2], &EntryType::File, "entry3", 68_608, "2025-07-17 23:59");
        assert_entry(&entries[3], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
    }

    #[test]
    fn test_httpdirectory_filter_by_name_simple_regex() {
        // unwrap here is ok since we know this should not return anything else
        // than Ok(httpdir) if it does it should panic as the test failed.
        let httpdir = prepare_httpdir().filter_by_name("debian").unwrap();
        assert_eq!(httpdir.len(), 2);

        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[1], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
    }

    #[test]
    fn test_httpdirectory_filter_by_name_bad_regex() {
        match prepare_httpdir().filter_by_name("deb-[n+-*") {
            Ok(_) => panic!("This call must return an Err(), not Ok()"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Error: regex parse error:\n    deb-[n+-*\n          ^^^\nerror: invalid character class range, the start must be <= the end"
            ),
        }
    }

    #[test]
    fn test_httpdirectory_filter_by_name_less_simple_regex() {
        // unwrap here is ok since we know this should not return anything else
        // than Ok(httpdir) if it does it should panic as the test failed.
        let httpdir = prepare_httpdir().filter_by_name(r#"debian\d|entry|file\d"#).unwrap();
        assert_eq!(httpdir.len(), 5);

        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[1], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");
        assert_entry(&entries[2], &EntryType::File, "file1", 123, "1987-10-09 04:37");
        assert_entry(&entries[3], &EntryType::File, "entry3", 68_608, "2025-07-17 23:59");
        assert_entry(&entries[4], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
    }

    #[test]
    fn test_httpdirectory_parent_directory() {
        let httpdir = prepare_httpdir().parent_directory();
        let entries = httpdir.entries();

        assert_eq!(httpdir.len(), 1);

        assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    }

    #[test]
    fn test_httpdirectory_nothing() {
        let httpdir = prepare_httpdir().dirs().files();

        assert_eq!(httpdir.len(), 0);
    }

    #[test]
    fn test_httpdirectory_sort_by_name() {
        let httpdir = prepare_httpdir().sort_by_name(Sorting::Ascending);

        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
        assert_entry(&entries[1], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[2], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
        assert_entry(&entries[3], &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        assert_entry(&entries[4], &EntryType::File, "entry3", 6_8608, "2025-07-17 23:59");
        assert_entry(&entries[5], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");
        assert_entry(&entries[6], &EntryType::File, "file1", 123, "1987-10-09 04:37");
        assert_entry(&entries[7], &EntryType::File, "files2", 2_345, "2023-01-01 00:00");
        assert_entry(&entries[8], &EntryType::Directory, "test2", 0, "2025-02-16 13:37");

        let output = format!("{httpdir}");
        assert_eq!(
            output,
            r##"
DIR         -                    ..
DIR         -  2025-03-01 07:11  debian3
FILE     123M  2024-12-08 08:22  debian4
DIR         -  2025-01-26 12:54  dir1
FILE      67K  2025-07-17 23:59  entry3
DIR         -  2025-01-02 12:32  entry4
FILE      123  1987-10-09 04:37  file1
FILE     2345  2023-01-01 00:00  files2
DIR         -  2025-02-16 13:37  test2
"##
        );
        let httpdir = httpdir.sort_by_name(Sorting::Descending);
        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
        assert_entry(&entries[1], &EntryType::Directory, "test2", 0, "2025-02-16 13:37");
        assert_entry(&entries[2], &EntryType::File, "files2", 2_345, "2023-01-01 00:00");
        assert_entry(&entries[3], &EntryType::File, "file1", 123, "1987-10-09 04:37");
        assert_entry(&entries[4], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");
        assert_entry(&entries[5], &EntryType::File, "entry3", 68_608, "2025-07-17 23:59");
        assert_entry(&entries[6], &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        assert_entry(&entries[7], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
        assert_entry(&entries[8], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");

        let output = format!("{httpdir}");
        assert_eq!(
            output,
            r##"
DIR         -                    ..
DIR         -  2025-02-16 13:37  test2
FILE     2345  2023-01-01 00:00  files2
FILE      123  1987-10-09 04:37  file1
DIR         -  2025-01-02 12:32  entry4
FILE      67K  2025-07-17 23:59  entry3
DIR         -  2025-01-26 12:54  dir1
FILE     123M  2024-12-08 08:22  debian4
DIR         -  2025-03-01 07:11  debian3
"##
        );
    }

    #[test]
    fn test_httpdirectory_sort_by_date() {
        let httpdir = prepare_httpdir().sort_by_date(Sorting::Ascending);

        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
        assert_entry(&entries[1], &EntryType::File, "file1", 123, "1987-10-09 04:37");
        assert_entry(&entries[2], &EntryType::File, "files2", 2_345, "2023-01-01 00:00");
        assert_entry(&entries[3], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
        assert_entry(&entries[4], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");
        assert_entry(&entries[5], &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        assert_entry(&entries[6], &EntryType::Directory, "test2", 0, "2025-02-16 13:37");
        assert_entry(&entries[7], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[8], &EntryType::File, "entry3", 68_608, "2025-07-17 23:59");

        let output = format!("{httpdir}");
        assert_eq!(
            output,
            r##"
DIR         -                    ..
FILE      123  1987-10-09 04:37  file1
FILE     2345  2023-01-01 00:00  files2
FILE     123M  2024-12-08 08:22  debian4
DIR         -  2025-01-02 12:32  entry4
DIR         -  2025-01-26 12:54  dir1
DIR         -  2025-02-16 13:37  test2
DIR         -  2025-03-01 07:11  debian3
FILE      67K  2025-07-17 23:59  entry3
"##
        );
        let httpdir = httpdir.sort_by_date(Sorting::Descending);
        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
        assert_entry(&entries[1], &EntryType::File, "entry3", 68_608, "2025-07-17 23:59");
        assert_entry(&entries[2], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[3], &EntryType::Directory, "test2", 0, "2025-02-16 13:37");
        assert_entry(&entries[4], &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        assert_entry(&entries[5], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");
        assert_entry(&entries[6], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
        assert_entry(&entries[7], &EntryType::File, "files2", 2_345, "2023-01-01 00:00");
        assert_entry(&entries[8], &EntryType::File, "file1", 123, "1987-10-09 04:37");

        let output = format!("{httpdir}");
        assert_eq!(
            output,
            r##"
DIR         -                    ..
FILE      67K  2025-07-17 23:59  entry3
DIR         -  2025-03-01 07:11  debian3
DIR         -  2025-02-16 13:37  test2
DIR         -  2025-01-26 12:54  dir1
DIR         -  2025-01-02 12:32  entry4
FILE     123M  2024-12-08 08:22  debian4
FILE     2345  2023-01-01 00:00  files2
FILE      123  1987-10-09 04:37  file1
"##
        );
    }

    #[test]
    fn test_httpdirectory_sort_by_size() {
        let httpdir = prepare_httpdir().sort_by_size(Sorting::Ascending);

        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
        assert_entry(&entries[1], &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        assert_entry(&entries[2], &EntryType::Directory, "test2", 0, "2025-02-16 13:37");
        assert_entry(&entries[3], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[4], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");
        assert_entry(&entries[5], &EntryType::File, "file1", 123, "1987-10-09 04:37");
        assert_entry(&entries[6], &EntryType::File, "files2", 2_345, "2023-01-01 00:00");
        assert_entry(&entries[7], &EntryType::File, "entry3", 68_608, "2025-07-17 23:59");
        assert_entry(&entries[8], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");

        let output = format!("{httpdir}");
        assert_eq!(
            output,
            r##"
DIR         -                    ..
DIR         -  2025-01-26 12:54  dir1
DIR         -  2025-02-16 13:37  test2
DIR         -  2025-03-01 07:11  debian3
DIR         -  2025-01-02 12:32  entry4
FILE      123  1987-10-09 04:37  file1
FILE     2345  2023-01-01 00:00  files2
FILE      67K  2025-07-17 23:59  entry3
FILE     123M  2024-12-08 08:22  debian4
"##
        );
        let httpdir = httpdir.sort_by_size(Sorting::Descending);
        let entries = httpdir.entries();
        assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
        assert_entry(&entries[1], &EntryType::File, "debian4", 128_974_848, "2024-12-08 08:22");
        assert_entry(&entries[2], &EntryType::File, "entry3", 68_608, "2025-07-17 23:59");
        assert_entry(&entries[3], &EntryType::File, "files2", 2_345, "2023-01-01 00:00");
        assert_entry(&entries[4], &EntryType::File, "file1", 123, "1987-10-09 04:37");
        assert_entry(&entries[5], &EntryType::Directory, "dir1", 0, "2025-01-26 12:54");
        assert_entry(&entries[6], &EntryType::Directory, "test2", 0, "2025-02-16 13:37");
        assert_entry(&entries[7], &EntryType::Directory, "debian3", 0, "2025-03-01 07:11");
        assert_entry(&entries[8], &EntryType::Directory, "entry4", 0, "2025-01-02 12:32");

        let output = format!("{httpdir}");
        assert_eq!(
            output,
            r##"
DIR         -                    ..
FILE     123M  2024-12-08 08:22  debian4
FILE      67K  2025-07-17 23:59  entry3
FILE     2345  2023-01-01 00:00  files2
FILE      123  1987-10-09 04:37  file1
DIR         -  2025-01-26 12:54  dir1
DIR         -  2025-02-16 13:37  test2
DIR         -  2025-03-01 07:11  debian3
DIR         -  2025-01-02 12:32  entry4
"##
        );
    }

    #[test]
    fn test_httpdirectory_stats() {
        let httpdir = prepare_httpdir();
        let stats = httpdir.stats();

        assert_eq!(stats.parent_dir, 1);
        assert_eq!(stats.dirs, 4);
        assert_eq!(stats.files, 4);
        assert_eq!(stats.total_size, 129_045_924);
    }
}
