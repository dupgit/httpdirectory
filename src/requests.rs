use crate::HTTPDIR_USER_AGENT;
use crate::error::HttpDirError;
use log::{error, trace};
use reqwest::{Client, Response, StatusCode, Url};

#[derive(Debug, Default)]
pub(crate) struct Request {
    client: Client,
}

impl Request {
    /// Returns a new reqwest client that will be used to make
    /// HTTP requests
    ///
    /// # Errors
    ///
    /// Returns an error if the request client can not be built
    pub(crate) fn new() -> Result<Self, HttpDirError> {
        match Client::builder().user_agent(HTTPDIR_USER_AGENT).build() {
            Ok(client) => {
                trace!("new reqwest client: {client:?}");
                Ok(Request {
                    client,
                })
            }
            Err(e) => {
                error!("Error building a new reqwest client: {e}");
                Err(HttpDirError::HttpError(e))
            }
        }
    }

    /// Returns the content of an url if any
    ///
    /// # Errors
    ///
    /// Returns an error when the reqwest could not be made or that the server
    /// did not respond with a 200 HTTP status code.
    pub(crate) async fn get(&self, url: &str) -> Result<Response, HttpDirError> {
        match url::Url::parse(url) {
            Ok(_) => {
                trace!("Requesting '{url}'");
                match self.client.get(url).send().await {
                    Ok(response) if response.status() == StatusCode::OK => Ok(response),
                    Ok(response) => {
                        error!("Error while retrieving url {url} content: {}", response.status());
                        Err(HttpDirError::ContentError(format!(
                            "Error while retrieving url {url} content: {}",
                            response.status()
                        )))
                    }
                    Err(e) => Err(HttpDirError::HttpError(e)),
                }
            }
            Err(e) => {
                error!("Error parsing url '{url}': {e}");
                Err(HttpDirError::ParseError(e))
            }
        }
    }
}

pub(crate) fn join_url(base: &str, dir: &str) -> Result<String, HttpDirError> {
    Ok(Url::parse(base)?.join(dir)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bad_url() {
        match Request::new() {
            Ok(client) => assert!(client.get("this_is_not_a valid_url").await.is_err()),
            Err(e) => panic!("This test failed: {e}"),
        }
    }

    #[tokio::test]
    async fn test_url_does_not_exists() {
        match Request::new() {
            Ok(client) => assert!(client.get("https://this-does-not-exists.org/").await.is_err()),
            Err(e) => panic!("This test failed: {e}"),
        }
    }
}
