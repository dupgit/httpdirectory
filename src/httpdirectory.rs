use crate::error::HttpDirError;
use crate::httpdirectoryentry::HttpDirectoryEntry;
use crate::requests::Request;
use crate::scrape::scrape_body;
use log::error;
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
// @todo: implement a filter() function to keep only the
//        entries that fulfil a condition ?
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
        let entries = match scrape_body(&response.text().await?) {
            Ok(entries) => entries,
            Err(e) => {
                error!("Error getting entries: {e}");
                vec![]
            }
        };

        Ok(HttpDirectory {
            entries,
            url: url.to_string(),
            request: client,
        })
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

// @todo: ? implement an iterator ?
mod tests {
    use super::*;

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
        httpdir.entries.push(HttpDirectoryEntry::new("dir2", "2025-02-16 13:37", "-", "dir2/"));
        httpdir.entries.push(HttpDirectoryEntry::new("dir3", "2025-03-01 07:11", "-", "dir3/"));
        httpdir.entries.push(HttpDirectoryEntry::new("dir4", "2025-01-02 12:32", "-", "dir4/"));
        httpdir.entries.push(HttpDirectoryEntry::new("file1", "1987-10-09 04:37", "123", "file1/"));
        httpdir.entries.push(HttpDirectoryEntry::new("file2", "2023-01-01 00:00", "2345", "file2/"));
        httpdir.entries.push(HttpDirectoryEntry::new("file3", "2025-07-17 23:59", "67K", "file3/"));
        httpdir.entries.push(HttpDirectoryEntry::new("file4", "2024-12-08 08:22", "123M", "file4/"));
        httpdir.entries.push(HttpDirectoryEntry::new("parent directory", "2025-01-26 12:54", "-", "../"));

        httpdir
    }

    #[test]
    fn test_httpdirectory_dirs() {
        let httpdir = prepare_httpdir().dirs();
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 4);

        assert_entry(&entries[0], false, true, false, "dir1", 0, 2025, 01, 26, 12, 54);
        assert_entry(&entries[1], false, true, false, "dir2", 0, 2025, 02, 16, 13, 37);
        assert_entry(&entries[2], false, true, false, "dir3", 0, 2025, 03, 01, 07, 11);
        assert_entry(&entries[3], false, true, false, "dir4", 0, 2025, 01, 02, 12, 32);
    }

    #[test]
    fn test_httpdirectory_files() {
        let httpdir = prepare_httpdir().files();
        let entries = httpdir.entries();

        assert_eq!(entries.len(), 4);

        assert_entry(&entries[0], false, false, true, "file1", 123, 1987, 10, 09, 04, 37);
        assert_entry(&entries[1], false, false, true, "file2", 2345, 2023, 01, 01, 00, 00);
        assert_entry(&entries[2], false, false, true, "file3", 68608, 2025, 07, 17, 23, 59);
        assert_entry(&entries[3], false, false, true, "file4", 128974848, 2024, 12, 08, 08, 22);
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
