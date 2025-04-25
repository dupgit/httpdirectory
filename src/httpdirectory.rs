use crate::{HTTPDIR_USER_AGENT, error::HttpDirError};
use ureq::http::header::USER_AGENT;

use crate::httpdirectoryentry::HttpDirectoryEntry;

#[derive(Debug)]
pub struct HttpDirectory {
    entries: Vec<HttpDirectoryEntry>,
}

impl HttpDirectory {
    pub fn default() -> Self {
        HttpDirectory {
            entries: vec![],
        }
    }
    pub fn new(url: &str) -> Result<Self, HttpDirError> {
        let body: String = ureq::get(url).header(USER_AGENT, HTTPDIR_USER_AGENT).call()?.body_mut().read_to_string()?;

        Ok(HttpDirectory {
            entries: vec![],
        })
    }
}
