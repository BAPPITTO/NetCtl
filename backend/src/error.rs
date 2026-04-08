use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    NetworkError(String),
    DatabaseError(String),
    StateError(String),
    XdpError(String),
    ConfigError(String),
    SystemCommand(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Error::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Error::StateError(msg) => write!(f, "State error: {}", msg),
            Error::XdpError(msg) => write!(f, "XDP error: {}", msg),
            Error::ConfigError(msg) => write!(f, "Config error: {}", msg),
            Error::SystemCommand(msg) => write!(f, "System command failed: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
