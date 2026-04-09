use std::fmt;

/// Central error type for the backend
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::StateError(msg) => write!(f, "State error: {}", msg),
            Self::XdpError(msg) => write!(f, "XDP error: {}", msg),
            Self::ConfigError(msg) => write!(f, "Config error: {}", msg),
            Self::SystemCommand(msg) => write!(f, "System command failed: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Generic result type for backend operations
pub type Result<T> = std::result::Result<T, Error>;