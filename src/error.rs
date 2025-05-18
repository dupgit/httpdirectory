use std::error;
use std::fmt;
// use std::fmt::write;

/// Errors handling
#[derive(Debug)]
pub enum HttpDirError {
    /// Errors that are thrown by reqwest library.
    HttpError(reqwest::Error),

    /// Http engine has not been selected. (Note that this
    /// engine is selected as reqwest by default for now)
    NoHttpEngine,

    /// Errors on the content of a retrieved url
    /// for instance when there is no content at all.
    ContentError(String),

    Regex(regex::Error),
}

impl fmt::Display for HttpDirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpDirError::HttpError(e) => write!(f, "Error: {e}"),
            HttpDirError::ContentError(e) => write!(f, "Error: {e}"),
            HttpDirError::NoHttpEngine => write!(f, "Error no http engine has been selected"),
            HttpDirError::Regex(e) => write!(f, "Error: {e}"),
        }
    }
}

impl error::Error for HttpDirError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            HttpDirError::HttpError(err) => Some(err),
            HttpDirError::ContentError(err) => Some(Err(err).unwrap()),
            HttpDirError::NoHttpEngine => Some(Err(()).unwrap()),
            HttpDirError::Regex(err) => Some(err),
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

#[cfg(test)]
mod test_error {
    use crate::error::HttpDirError;
}
