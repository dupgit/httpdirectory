use const_format::formatcp;

pub(crate) mod requests;

pub(crate) mod scrape;

/// All errors that you might get from httpdirectory
pub mod error;

/// HttpDirectory structure that allows one to get http directories
/// in a convenient structure
pub mod httpdirectory;

pub mod httpdirectoryentry;

pub mod entry;

/// User Agent used by httpdirectory
pub const HTTPDIR_USER_AGENT: &str = formatcp!("httpdirectory/{}", env!("CARGO_PKG_VERSION"));
