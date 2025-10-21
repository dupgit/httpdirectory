use crate::HTTPDIR_USER_AGENT;
use crate::error::HttpDirError;
use log::{error, trace};
use reqwest::{Client, Response, StatusCode, Url};

#[derive(Debug)]
pub enum Request {
    Reqwest(Client),
    None,
}

impl Request {
    /// Returns a new client that will be used to make requests
    /// it now returns a Reqwest client by default
    ///
    /// # Errors
    ///
    /// Returns an error if the request client can not be built
    pub(crate) fn new() -> Result<Self, HttpDirError> {
        match Client::builder().user_agent(HTTPDIR_USER_AGENT).build() {
            Ok(client) => {
                trace!("new reqwest client: {client:?}");
                Ok(Request::Reqwest(client))
            }
            Err(e) => {
                error!("Error building a new reqwest client: {e}");
                Err(HttpDirError::HttpError(e))
            }
        }
    }

    /// Returns the content of an url if any and if the request engine has
    /// been selected
    ///
    /// # Errors
    ///
    /// Returns an error when no request engine has been selected or
    /// that the reqwest could not be made or that the server did not
    /// respond with a 200 HTTP status code.
    pub(crate) async fn get(&self, url: &str) -> Result<Response, HttpDirError> {
        match url::Url::parse(url) {
            Ok(_) => {
                trace!("Requesting '{url}'");
                match self {
                    Request::Reqwest(client) => match client.get(url).send().await {
                        Ok(response) => match response.status() {
                            StatusCode::OK => Ok(response),
                            _ => {
                                error!("Error while retrieving url {url} content: {}", response.status());
                                Err(HttpDirError::ContentError(format!(
                                    "Error while retrieving url {url} content: {}",
                                    response.status()
                                )))
                            }
                        },
                        Err(e) => Err(HttpDirError::HttpError(e)),
                    },
                    Request::None => Err(HttpDirError::NoHttpEngine),
                }
            }
            Err(e) => {
                error!("Error parsing url '{url}': {e}");
                Err(HttpDirError::ParseError(e))
            }
        }
    }
}

pub fn join_url(base: &str, dir: &str) -> Result<String, HttpDirError> {
    Ok(Url::parse(base)?.join(dir)?.to_string())
}
