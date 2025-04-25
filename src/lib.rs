use const_format::formatcp;

pub mod httpdirectory;

pub(crate) mod httpdirectoryentry;

pub(crate) mod entry;

pub mod error;

/// User Agent for the whole program
pub const HTTPDIR_USER_AGENT: &str = formatcp!("httpdirectory/{}", env!("CARGO_PKG_VERSION"));
