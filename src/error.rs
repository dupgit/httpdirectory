use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum HttpDirError {
    /// Errors that are thrown by reqwest library.
    #[snafu(display("HTTP request failed for '{url}':\n -> {source}"))]
    HttpReqwestUrl {
        url: String,
        source: reqwest::Error,
    },

    #[snafu(display("Error building a new reqwest client:\n -> {source}"))]
    HttpReqwestBuilder {
        source: reqwest::Error,
    },

    #[snafu(display("Error retrieving content from '{url}': {status_code}"))]
    HttpResponse {
        url: String,
        status_code: reqwest::StatusCode,
    },

    /// Errors in regular expression (`filter_by_name()` may
    /// fail when used with a bad regular expression)
    #[snafu(display("Regular expression failed to compile '{regex}':\n -> {source}"))]
    RegexCompile {
        regex: String,
        source: regex::Error,
    },

    /// Parsing error when manipulating urls (`cd()` method
    /// does manipulates url for instance)
    #[snafu(display("Error while parsing url '{url}':\n -> {source}"))]
    Parse {
        url: String,
        source: url::ParseError,
    },

    #[snafu(display("Error while parsing content with selector '{selector}'"))]
    Selector {
        selector: String,
    },
}

pub type Result<T> = std::result::Result<T, HttpDirError>;

// Trait helper for Reqwest errors
pub(crate) trait ReqwestResultExt<T> {
    fn with_url(self, url: &str) -> Result<T>;
    fn with(self) -> Result<T>;
}

impl<T> ReqwestResultExt<T> for std::result::Result<T, reqwest::Error> {
    fn with(self) -> Result<T> {
        self.map_err(|source| HttpDirError::HttpReqwestBuilder {
            source,
        })
    }

    fn with_url(self, url: &str) -> Result<T> {
        self.map_err(|source| HttpDirError::HttpReqwestUrl {
            url: url.to_string(),
            source,
        })
    }
}

// Trait helper for Regex errors
pub trait RegexResultExt<T> {
    fn with_regex(self, regex: &str) -> Result<T>;
}

impl<T> RegexResultExt<T> for std::result::Result<T, regex::Error> {
    fn with_regex(self, regex: &str) -> Result<T> {
        self.map_err(|source| HttpDirError::RegexCompile {
            regex: regex.to_string(),
            source,
        })
    }
}

// Trait helper for Parse errors
pub trait ParseResultExt<T> {
    fn with_url(self, url: &str) -> Result<T>;
}

impl<T> ParseResultExt<T> for std::result::Result<T, url::ParseError> {
    fn with_url(self, url: &str) -> Result<T> {
        self.map_err(|source| HttpDirError::Parse {
            url: url.to_string(),
            source,
        })
    }
}

// Trait helper for Selector errors
pub trait SelectorResultExt<T> {
    fn with_selector(self, selector: &str) -> Result<T>;
}

impl<T> SelectorResultExt<T> for std::result::Result<T, scraper::error::SelectorErrorKind<'_>> {
    fn with_selector(self, selector: &str) -> Result<T> {
        self.map_err(|_source| HttpDirError::Selector {
            selector: selector.to_string(),
        })
    }
}
