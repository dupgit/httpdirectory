use crate::error::HttpDirError;
use crate::httpdirectoryentry::HttpDirectoryEntry;
use crate::requests::{Request, join_url};
use crate::scrape::scrape_body;
use log::{debug, error};
use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub struct HttpDirectory {
    entries: Vec<HttpDirectoryEntry>,
    url: String,
    request: Request,
}

// @todo: implement sorting by size, date, name
// @todo: implement cd() function to go to a specific
//        directory if possible
impl HttpDirectory {
    /// Returns an `HttpDirectory` initialized with default
    /// values (empty vector, empty url and no HttpEngine)
    #[must_use]
    pub fn default() -> Self {
        HttpDirectory {
            entries: vec![],
            url: "".to_string(),
            request: Request::None,
        }
    }

    /// Crawls the `url` and returns (if no error occurred) the
    /// HttpDirectory of that url
    pub async fn new(url: &str) -> Result<Self, HttpDirError> {
        let client = Request::new().await?;
        let response = client.get(url).await?;

        let entries = get_entries_from_body(&response.text().await?).await;
        Ok(HttpDirectory {
            entries,
            url: url.to_string(),
            request: client,
        })
    }

    /// Change directory if possible to dir (from url) and gets the new
    /// `HttpDirectory` listing if any and returns it.
    pub async fn cd(&mut self, dir: &str) -> Result<&Self, HttpDirError> {
        let url = join_url(&self.url, dir)?;
        debug!("cd is going to {url}");
        let response = self.request.get(&url).await?;
        let entries = get_entries_from_body(&response.text().await?).await;
        self.entries = entries;
        self.url = url;
        Ok(self)
    }

    /// Returns only directories of the `HttpDirectory` listing
    #[must_use]
    pub fn dirs(mut self) -> Self {
        self.entries = self.entries.into_iter().filter(|e| e.is_directory()).collect();
        self
    }

    /// Returns only files of the `HttpDirectory` listing
    #[must_use]
    pub fn files(mut self) -> Self {
        self.entries = self.entries.into_iter().filter(|e| e.is_file()).collect();
        self
    }

    /// Returns only the parent directory the `HttpDirectory` listing
    #[must_use]
    pub fn parent_directory(mut self) -> Self {
        self.entries = self.entries.into_iter().filter(|e| e.is_parent_directory()).collect();
        self
    }

    /// Filters the `HttpDirectory` listing by filtering names of each
    /// entry with the `regex` regular expression.
    pub fn filter_by_name(mut self, regex: &str) -> Result<Self, HttpDirError> {
        let re = match Regex::new(regex) {
            Ok(re) => re,
            Err(e) => return Err(HttpDirError::Regex(e)),
        };

        self.entries = self.entries.into_iter().filter(|e| e.is_match_by_name(&re)).collect();

        Ok(self)
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

async fn get_entries_from_body(body: &str) -> Vec<HttpDirectoryEntry> {
    match scrape_body(body) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Error getting entries: {e}");
            vec![]
        }
    }
}

// @todo: ? implement an iterator ?
mod tests {
    use super::*;
    use crate::httpdirectoryentry::assert_entry;

    #[test]
    fn test_httpdirectory_default() {
        let httpdir = HttpDirectory::default();

        assert!(httpdir.entries.is_empty());
        assert_eq!(httpdir.url, "".to_string());
        match httpdir.request {
            Request::Reqwest(request) => panic!("{request:?} should be None"),
            Request::None => (),
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
    fn test_httpdirectory_dirs() {
        let httpdir = prepare_httpdir().dirs();
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 4);

        assert_entry(&entries[0], false, true, false, "dir1", 0, 2025, 01, 26, 12, 54);
        assert_entry(&entries[1], false, true, false, "test2", 0, 2025, 02, 16, 13, 37);
        assert_entry(&entries[2], false, true, false, "debian3", 0, 2025, 03, 01, 07, 11);
        assert_entry(&entries[3], false, true, false, "entry4", 0, 2025, 01, 02, 12, 32);
    }

    #[test]
    fn test_httpdirectory_files() {
        let httpdir = prepare_httpdir().files();
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 4);

        assert_entry(&entries[0], false, false, true, "file1", 123, 1987, 10, 09, 04, 37);
        assert_entry(&entries[1], false, false, true, "files2", 2345, 2023, 01, 01, 00, 00);
        assert_entry(&entries[2], false, false, true, "entry3", 68608, 2025, 07, 17, 23, 59);
        assert_entry(&entries[3], false, false, true, "debian4", 128974848, 2024, 12, 08, 08, 22);
    }

    #[test]
    fn test_httpdirectory_filter_by_name_simple_regex() {
        // unwrap here is ok since we know this should not return anything else
        // than Ok(httpdir) if it does it should panic as the test failed.
        let httpdir = prepare_httpdir().filter_by_name("debian").unwrap();
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 2);

        assert_entry(&entries[0], false, true, false, "debian3", 0, 2025, 03, 01, 07, 11);
        assert_entry(&entries[1], false, false, true, "debian4", 128974848, 2024, 12, 08, 08, 22);
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
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 5);

        assert_entry(&entries[0], false, true, false, "debian3", 0, 2025, 03, 01, 07, 11);
        assert_entry(&entries[1], false, true, false, "entry4", 0, 2025, 01, 02, 12, 32);
        assert_entry(&entries[2], false, false, true, "file1", 123, 1987, 10, 09, 04, 37);
        assert_entry(&entries[3], false, false, true, "entry3", 68608, 2025, 07, 17, 23, 59);
        assert_entry(&entries[4], false, false, true, "debian4", 128974848, 2024, 12, 08, 08, 22);
    }

    #[test]
    fn test_httpdirectory_parent_directory() {
        let httpdir = prepare_httpdir().parent_directory();
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 1);

        assert_entry(&entries[0], true, false, true, "../", 0, 0, 0, 0, 0, 0);
    }

    #[test]
    fn test_httpdirectory_nothing() {
        let httpdir = prepare_httpdir().dirs().files();
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 0);
    }
}
