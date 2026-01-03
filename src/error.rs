use scraper::error::SelectorErrorKind;
use std::fmt;

/// Errors handling
#[derive(Debug)]
pub enum HttpDirError {
    /// Errors that are thrown by reqwest library.
    HttpError(reqwest::Error),

    /// Errors on the content of a retrieved url
    /// for instance when there is no content at all.
    ContentError(String),

    /// Errors in regular expression (`filter_by_name()` may
    /// fail when used with a bad regular expression)
    Regex(regex::Error),

    /// Parsing error when manipulating urls (`cd()` method
    /// does manipulates url for instance)
    ParseError(url::ParseError),

    ScrapeError(String),
}

impl fmt::Display for HttpDirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpDirError::HttpError(e) => write!(f, "Error: {e}"),
            HttpDirError::ContentError(e) => write!(f, "Error: {e}"),
            HttpDirError::Regex(e) => write!(f, "Error: {e}"),
            HttpDirError::ParseError(e) => write!(f, "Error: {e}"),
            HttpDirError::ScrapeError(e) => write!(f, "{e}"),
        }
    }
}

impl From<reqwest::Error> for HttpDirError {
    fn from(error: reqwest::Error) -> Self {
        HttpDirError::HttpError(error)
    }
}

impl From<regex::Error> for HttpDirError {
    fn from(error: regex::Error) -> Self {
        HttpDirError::Regex(error)
    }
}

impl From<String> for HttpDirError {
    fn from(error: String) -> Self {
        HttpDirError::ContentError(error)
    }
}

impl From<url::ParseError> for HttpDirError {
    fn from(error: url::ParseError) -> Self {
        HttpDirError::ParseError(error)
    }
}

impl From<SelectorErrorKind<'_>> for HttpDirError {
    fn from(sek: SelectorErrorKind<'_>) -> Self {
        HttpDirError::ContentError(format!("scraper selector error: {sek}").to_string())
    }
}
