use crate::{
    HTTPDIR_USER_AGENT,
    error::{HttpDirError, ParseResultExt, ReqwestResultExt, Result},
};
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
    pub(crate) fn new(timeout_s: Option<u64>) -> Result<Self> {
        let client_builder;
        if let Some(timeout_s) = timeout_s {
            client_builder = Client::builder().user_agent(HTTPDIR_USER_AGENT).timeout(Duration::from_secs(timeout_s));
        } else {
            client_builder = Client::builder().user_agent(HTTPDIR_USER_AGENT);
        }

        let client = client_builder.build().with()?;
        trace!("New reqwest client: {client:?}");
        Ok(Request {
            client,
        })
    }

    /// Returns the content of an url if any
    ///
    /// # Errors
    ///
    /// Returns an error when the reqwest could not be made or that the server
    /// did not respond with a 200 HTTP status code.
    pub(crate) async fn get(&self, url: &str) -> Result<Response> {
        url::Url::parse(url).with_url(url)?;

        trace!("Requesting '{url}'");

        match self.client.get(url).send().await.with_url(url) {
            Ok(response) if response.status() == StatusCode::OK => Ok(response),
            Ok(response) => {
                error!("Error while retrieving url {url} content: {}", response.status());
                Err(HttpDirError::HttpResponse {
                    url: url.to_string(),
                    status_code: response.status(),
                })
            }
            Err(e) => Err(e),
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
