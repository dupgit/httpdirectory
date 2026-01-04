use crate::HTTPDIR_USER_AGENT;
use crate::error::HttpDirError;
use reqwest::{Client, Response, StatusCode};
use std::time::Duration;
use tracing::{error, trace};

#[derive(Debug, Default)]
pub(crate) struct Request {
    client: Client,
}

impl Request {
    /// Returns a new reqwest client that will be used to make
    /// HTTP requests. `timeout_s` optionally defines a global
    /// request timeout in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if the request client can not be built
    pub(crate) fn new(timeout_s: Option<u64>) -> Result<Self, HttpDirError> {
        let client_builder;
        if let Some(timeout_s) = timeout_s {
            client_builder = Client::builder().user_agent(HTTPDIR_USER_AGENT).timeout(Duration::from_secs(timeout_s));
        } else {
            client_builder = Client::builder().user_agent(HTTPDIR_USER_AGENT);
        }

        match client_builder.build() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bad_url() {
        match Request::new(None) {
            Ok(client) => assert!(client.get("this_is_not_a valid_url").await.is_err()),
            Err(e) => panic!("This test failed: {e}"),
        }
    }

    #[tokio::test]
    async fn test_url_does_not_exists() {
        match Request::new(None) {
            Ok(client) => assert!(client.get("https://this-does-not-exists.org/").await.is_err()),
            Err(e) => panic!("This test failed: {e}"),
        }
    }
}
