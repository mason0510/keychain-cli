use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    KeychainError(String),
    #[allow(dead_code)]
    EnvParseError(String),
    ValidationError(String),
    #[allow(dead_code)]
    ConfigError(String),
    JsonError(serde_json::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO error: {}", e),
            Error::KeychainError(msg) => write!(f, "Keychain error: {}", msg),
            Error::EnvParseError(msg) => write!(f, "Environment parse error: {}", msg),
            Error::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Error::ConfigError(msg) => write!(f, "Config error: {}", msg),
            Error::JsonError(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::JsonError(err)
    }
}
