#![doc = include_str!("../README.md")]
use const_format::formatcp;

pub(crate) mod requests;
pub(crate) mod scrape;

/// All errors that you might get from httpdirectory library
pub mod error;

/// Module that allows one to get http directories
/// in a structure with convenient methods
pub mod httpdirectory;

/// Module to deal with `HttpDirectoryEntry` enum that tells whether the
/// `Entry` is a Parent directory, a directory or a file.
pub mod httpdirectoryentry;

/// Module that helps storing all information about the entry (name, date, size and link)
pub mod entry;

pub mod stats;

/// User Agent used by httpdirectory that should be formatted
/// "httpdirectory/{}" where {} is the version of the library
pub const HTTPDIR_USER_AGENT: &str = formatcp!("httpdirectory/{}", env!("CARGO_PKG_VERSION"));
