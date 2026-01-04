use crate::error::HttpDirError;
use crate::httpdirectoryentry::{CompareField, HttpDirectoryEntry};
use crate::requests::{Request, join_url};
use crate::scrape::scrape_body;
use crate::stats::Stats;
use log::{debug, error, trace};
use regex::Regex;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Main structure that provides methods to access, parse a directory
/// webpage and fill that structure.
#[derive(Debug, Clone)]
pub struct HttpDirectory {
    entries: Vec<HttpDirectoryEntry>,
    url: Arc<String>,
    request: Arc<Request>,
    timings: Arc<Timings>,
}

#[derive(Debug, Default, Clone)]
struct Timings {
    /// Total time passed processing the http request
    http_request: Duration,
    get_entries: Duration,
}

impl Timings {
    fn new(http_request: Duration, get_entries: Duration) -> Self {
        Timings {
            http_request,
            get_entries,
        }
    }
}

// @todo: ? implement an iterator ?
impl HttpDirectory {
    /// Crawls the `url` and returns (if no error occurred) the
    /// `HttpDirectory` of that url
    ///
    /// # Errors
    ///
    /// Returns an error if a request client could not be made
    /// or that the request to the url did not return correctly
    /// with a 200 HTTP status code
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn new(url: &str) -> Result<Self, HttpDirError> {
        let now = Instant::now();
        let client = Request::new()?;
        let response = client.get(url).await?;
        let http_request = now.elapsed();
        trace!("Response to get '{url}': {response:?}");

        let now = Instant::now();
        let entries = get_entries_from_body(&response.text().await?);
        let get_entries = now.elapsed();
        let timings = Timings::new(http_request, get_entries);

        Ok(HttpDirectory {
            entries,
            url: Arc::new(url.to_string()),
            request: Arc::new(client),
            timings: Arc::new(timings),
        })
    }

    /// Change directory if possible to dir (from url) and gets the new
    /// `HttpDirectory` listing if any and returns it.
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - an error occurred while trying to retrieve data from the new
    ///   directory
    /// - the web server did not respond with 200 HTTP status code
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn cd(mut self, dir: &str) -> Result<Self, HttpDirError> {
        let url = join_url(&self.url, dir)?;
        debug!("cd is going to {url}");
        let now = Instant::now();
        let response = self.request.get(&url).await?;
        let http_request = now.elapsed();

        let now = Instant::now();
        let entries = get_entries_from_body(&response.text().await?);
        let get_entries = now.elapsed();

        let timings = Timings::new(http_request, get_entries);
        self.entries = entries;
        self.timings = Arc::new(timings);
        self.url = Arc::new(url);
        Ok(self)
    }

    /// Sorts the Directory entries by their names in ascending order when
    /// `ascending` is `true`, in descending order otherwise
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sort_by_name(mut self, ascending: bool) -> Self {
        self.entries.sort_by(|a, b| a.cmp_by_field(b, &CompareField::Name, ascending));
        self
    }

    /// Sorts the Directory entries by their dates in ascending order when
    /// `ascending` is `true`, in descending order otherwise
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sort_by_date(mut self, ascending: bool) -> Self {
        self.entries.sort_by(|a, b| a.cmp_by_field(b, &CompareField::Date, ascending));
        self
    }

    /// Sorts the Directory entries by their sizes in ascending order when
    /// `ascending` is `true`, in descending order otherwise
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn sort_by_size(mut self, ascending: bool) -> Self {
        self.entries.sort_by(|a, b| a.cmp_by_field(b, &CompareField::Size, ascending));
        self
    }

    /// Returns only elements of the `HttpDirectory` listing that
    /// matches the predicate f. An element of this predicate is
    /// of type `HttpDirectoryEntry`
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    #[must_use]
    pub fn filtering<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&HttpDirectoryEntry) -> bool,
    {
        let entries = self.entries.iter().filter(|entry| f(entry)).cloned().collect();

        HttpDirectory {
            entries,
            url: Arc::clone(&self.url),
            request: Arc::clone(&self.request),
            timings: Arc::clone(&self.timings),
        }
    }

    /// Returns only directories of the `HttpDirectory` listing
    #[must_use]
    pub fn dirs(&self) -> Self {
        self.filtering(HttpDirectoryEntry::is_directory)
    }

    /// Returns only files of the `HttpDirectory` listing
    #[must_use]
    pub fn files(&self) -> Self {
        self.filtering(HttpDirectoryEntry::is_file)
    }

    /// Returns only the parent directory the `HttpDirectory` listing
    #[must_use]
    pub fn parent_directory(&self) -> Self {
        self.filtering(HttpDirectoryEntry::is_parent_directory)
    }

    /// Returns the last entry Some(`HttpDirectoryEntry`) of that `HttpDirectory`
    /// if it exists or None
    #[must_use]
    pub fn last(&self) -> Option<&HttpDirectoryEntry> {
        self.entries.last()
    }

    /// Returns the first entry Some(`HttpDirectoryEntry`) of that `HttpDirectory`
    /// if it exists or None
    #[must_use]
    pub fn first(&self) -> Option<&HttpDirectoryEntry> {
        self.entries.first()
    }

    /// Returns the `Stats` (ie the number of files (with total
    /// apparent size), directories and parent directories) of
    /// the `HttpDirectory` structure
    #[must_use]
    pub fn stats(&self) -> Stats {
        let mut stats = Stats::new();
        for entry in self.entries() {
            match entry {
                HttpDirectoryEntry::ParentDirectory(_) => stats.add_parent_directory(),
                HttpDirectoryEntry::Directory(dir) => stats.add_directory(dir.date()),
                HttpDirectoryEntry::File(file) => stats.add_file(file.date(), file.size()),
            };
        }
        stats
    }

    /// Filters the `HttpDirectory` listing by filtering names of each
    /// entry with the `regex` regular expression.
    ///
    /// # Errors
    ///
    /// Will return an error if the regular expression can not be
    /// compiled (invalid pattern, or size limit exceeded). For more
    /// information see  [`Regex`][regex::Regex::new()]
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

    /// Returns the String that represents the url of
    /// that `HttpDirectory`
    #[must_use]
    pub fn get_url(&self) -> Arc<String> {
        self.url.clone()
    }

    /// Returns the process time that the HTTP request in itself
    /// took.
    #[must_use]
    pub fn http_request_time(&self) -> Duration {
        self.timings.http_request
    }

    /// Returns the process time that the analyze of
    /// the HTML retrieved content took
    #[must_use]
    pub fn get_entries_time(&self) -> Duration {
        self.timings.get_entries
    }

    /// Returns the process time that the HTTP request in itself
    /// took.
    #[must_use]
    pub fn total_time(&self) -> Duration {
        self.timings.get_entries + self.timings.http_request
    }
}

impl fmt::Display for HttpDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Processed '{}' in {:.2?} ({:.2?} + {:.2?})",
            &self.url,
            self.total_time(),
            self.http_request_time(),
            self.get_entries_time()
        )?;
        for entry in &self.entries {
            writeln!(f, "{entry}")?;
        }
        Ok(())
    }
}

impl Default for HttpDirectory {
    /// Returns an `HttpDirectory` initialized with default
    /// values (empty vector, empty url and defaults Request
    /// and Timings)
    fn default() -> Self {
        HttpDirectory {
            entries: vec![],
            url: Arc::new(String::new()),
            request: Arc::new(Request::default()),
            timings: Arc::new(Timings::default()),
        }
    }
}

fn entries_from_body(body: &str) -> Vec<HttpDirectoryEntry> {
    match scrape_body(body) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Error getting entries: {e}");
            vec![]
        }
    }
}

/// feature gated to be used only in tests - this method
/// should not be public
#[cfg(any(test, feature = "test-helpers"))]
pub fn get_entries_from_body(body: &str) -> Vec<HttpDirectoryEntry> {
    entries_from_body(body)
}

/// feature gated for production use
#[cfg(not(any(test, feature = "test-helpers")))]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub(crate) fn get_entries_from_body(body: &str) -> Vec<HttpDirectoryEntry> {
    entries_from_body(body)
}

#[cfg(test)]
mod tests {
    use {
        super::{HttpDirectory, HttpDirectoryEntry},
        crate::{
            httpdirectoryentry::{EntryType, assert_entry},
            stats::Stats,
        },
        unwrap_unreachable::UnwrapUnreachable,
    };

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
            r##"Processed '' in 0.00ns (0.00ns + 0.00ns)
DIR          -  2025-01-26 12:54  dir1
DIR          -  2025-02-16 13:37  test2
DIR          -  2025-03-01 07:11  debian3
DIR          -  2025-01-02 12:32  entry4
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
        // unreachable here is ok since we know this should not return anything else
        // than Ok(httpdir) if it does it should panic as the test failed.
        let httpdir = prepare_httpdir().filter_by_name("debian").unreachable();
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
        // unreachable here is ok since we know this should not return anything else
        // than Ok(httpdir) if it does it should panic as the test failed.
        let httpdir = prepare_httpdir().filter_by_name(r#"debian\d|entry|file\d"#).unreachable();
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
        let httpdir = prepare_httpdir().sort_by_name(true);

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
            r##"Processed '' in 0.00ns (0.00ns + 0.00ns)
DIR          -                    ..
DIR          -  2025-03-01 07:11  debian3
FILE      123M  2024-12-08 08:22  debian4
DIR          -  2025-01-26 12:54  dir1
FILE       67K  2025-07-17 23:59  entry3
DIR          -  2025-01-02 12:32  entry4
FILE       123  1987-10-09 04:37  file1
FILE      2345  2023-01-01 00:00  files2
DIR          -  2025-02-16 13:37  test2
"##
        );
        let httpdir = httpdir.sort_by_name(false);
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
            r##"Processed '' in 0.00ns (0.00ns + 0.00ns)
DIR          -                    ..
DIR          -  2025-02-16 13:37  test2
FILE      2345  2023-01-01 00:00  files2
FILE       123  1987-10-09 04:37  file1
DIR          -  2025-01-02 12:32  entry4
FILE       67K  2025-07-17 23:59  entry3
DIR          -  2025-01-26 12:54  dir1
FILE      123M  2024-12-08 08:22  debian4
DIR          -  2025-03-01 07:11  debian3
"##
        );
    }

    #[test]
    fn test_httpdirectory_sort_by_date() {
        let httpdir = prepare_httpdir().sort_by_date(true);

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
            r##"Processed '' in 0.00ns (0.00ns + 0.00ns)
DIR          -                    ..
FILE       123  1987-10-09 04:37  file1
FILE      2345  2023-01-01 00:00  files2
FILE      123M  2024-12-08 08:22  debian4
DIR          -  2025-01-02 12:32  entry4
DIR          -  2025-01-26 12:54  dir1
DIR          -  2025-02-16 13:37  test2
DIR          -  2025-03-01 07:11  debian3
FILE       67K  2025-07-17 23:59  entry3
"##
        );
        let httpdir = httpdir.sort_by_date(false);
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
            r##"Processed '' in 0.00ns (0.00ns + 0.00ns)
DIR          -                    ..
FILE       67K  2025-07-17 23:59  entry3
DIR          -  2025-03-01 07:11  debian3
DIR          -  2025-02-16 13:37  test2
DIR          -  2025-01-26 12:54  dir1
DIR          -  2025-01-02 12:32  entry4
FILE      123M  2024-12-08 08:22  debian4
FILE      2345  2023-01-01 00:00  files2
FILE       123  1987-10-09 04:37  file1
"##
        );
    }

    #[test]
    fn test_httpdirectory_sort_by_size() {
        let httpdir = prepare_httpdir().sort_by_size(true);

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
            r##"Processed '' in 0.00ns (0.00ns + 0.00ns)
DIR          -                    ..
DIR          -  2025-01-26 12:54  dir1
DIR          -  2025-02-16 13:37  test2
DIR          -  2025-03-01 07:11  debian3
DIR          -  2025-01-02 12:32  entry4
FILE       123  1987-10-09 04:37  file1
FILE      2345  2023-01-01 00:00  files2
FILE       67K  2025-07-17 23:59  entry3
FILE      123M  2024-12-08 08:22  debian4
"##
        );
        let httpdir = httpdir.sort_by_size(false);
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
            r##"Processed '' in 0.00ns (0.00ns + 0.00ns)
DIR          -                    ..
FILE      123M  2024-12-08 08:22  debian4
FILE       67K  2025-07-17 23:59  entry3
FILE      2345  2023-01-01 00:00  files2
FILE       123  1987-10-09 04:37  file1
DIR          -  2025-01-26 12:54  dir1
DIR          -  2025-02-16 13:37  test2
DIR          -  2025-03-01 07:11  debian3
DIR          -  2025-01-02 12:32  entry4
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

    // Testing Stats

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
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-31 16:58", "-", "name/");
        let mut httpdirectory = HttpDirectory::default();
        httpdirectory.entries = vec![httpdirectoryentry];
        let stats = httpdirectory.stats();

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 0);
        assert_eq!(stats.total_size, 0);
        assert_eq!(stats.with_date, 1);
        assert_eq!(stats.without_date, 0);

        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-31 16:58", "3.1K", "name/");
        httpdirectory.entries.push(httpdirectoryentry);
        let stats = httpdirectory.stats();

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 1);
        assert_eq!(stats.total_size, 3174);
        assert_eq!(stats.with_date, 2);
        assert_eq!(stats.without_date, 0);

        let httpdirectoryentry = HttpDirectoryEntry::new("Parent Directory", "2025-05-31 16:58", "-", "../");
        httpdirectory.entries.push(httpdirectoryentry);
        let stats = httpdirectory.stats();

        assert_eq!(stats.parent_dir, 1);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 1);
        assert_eq!(stats.total_size, 3174);
        assert_eq!(stats.with_date, 2);
        assert_eq!(stats.without_date, 1);
    }

    #[test]
    fn test_stats_output() {
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "2025-05-31 16:58", "3.1K", "name/");
        let mut httpdirectory = HttpDirectory::default();
        httpdirectory.entries = vec![httpdirectoryentry];
        let stats = httpdirectory.stats();
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
        let httpdirectoryentry = HttpDirectoryEntry::new("name", "", "-", "name/");
        let mut httpdirectory = HttpDirectory::default();
        httpdirectory.entries = vec![httpdirectoryentry];
        let mut stats = httpdirectory.stats();

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 0);
        assert_eq!(stats.total_size, 0);
        assert_eq!(stats.with_date, 0);
        assert_eq!(stats.without_date, 1);

        let httpdirectoryentry = HttpDirectoryEntry::new("name", "", "3.1K", "name/");
        httpdirectory.entries.push(httpdirectoryentry);
        stats = httpdirectory.stats();

        assert_eq!(stats.parent_dir, 0);
        assert_eq!(stats.dirs, 1);
        assert_eq!(stats.files, 1);
        assert_eq!(stats.total_size, 3174);
        assert_eq!(stats.with_date, 0);
        assert_eq!(stats.without_date, 2);
    }
}
