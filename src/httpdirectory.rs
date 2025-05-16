use crate::error::HttpDirError;
use crate::httpdirectoryentry::HttpDirectoryEntry;
use crate::requests::Request;
use crate::scrape::scrape_body;
use log::{debug, error};
use std::fmt;

#[derive(Debug)]
pub struct HttpDirectory {
    entries: Vec<HttpDirectoryEntry>,
    url: String,
    request: Request,
}

// @todo: implement sorting by size, date, name
// @todo: implement files(), dirs() and parent() functions
//        that will return respectively all files, all
//        directories and the parent directory
// @todo: implement cd() function to go to a specific
//        directory if possible
// @todo: implement a filter() function to keep only the
//        entries that fulfil a condition ?
impl HttpDirectory {
    #[must_use]
    pub fn default() -> Self {
        HttpDirectory {
            entries: vec![],
            url: "".to_string(),
            request: Request::None,
        }
    }

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

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

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
