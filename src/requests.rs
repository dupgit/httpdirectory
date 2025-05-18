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
    pub async fn new() -> Result<Self, HttpDirError> {
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
    pub async fn get(&self, url: &str) -> Result<Response, HttpDirError> {
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
}

pub fn join_url(base: &str, dir: &str) -> Result<String, HttpDirError> {
    Ok(Url::parse(base)?.join(dir)?.to_string())
}
