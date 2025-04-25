use std::error;
use std::fmt;

#[derive(Debug)]
pub enum HttpDirError {
    HttpError(ureq::Error),
}

impl fmt::Display for HttpDirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for HttpDirError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            HttpDirError::HttpError(err) => Some(err),
        }
    }
}

impl From<ureq::Error> for HttpDirError {
    fn from(error: ureq::Error) -> Self {
        HttpDirError::HttpError(error)
    }
}

#[cfg(test)]
mod test_error {
    use crate::error::HttpDirError;

    #[test]
    fn test_display_http_dir_error() {
        let error = ureq::Error::ConnectionFailed;
        let err = HttpDirError::HttpError(error);
        let display = format!("{err}");
        assert_eq!(display, "HttpError(ConnectionFailed)");
    }
}
