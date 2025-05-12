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

impl HttpDirectory {
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

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
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
