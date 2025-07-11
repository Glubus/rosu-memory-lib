use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Error when reading memory
    MemoryRead(String),
    /// Error when parsing data
    Parse(String),
    /// Error when a feature is not available
    NotAvailable(String),
    /// Error when a file operation fails
    FileOperation(String),
    /// Error when an operation is not supported
    Unsupported(String),
    /// Other general errors
    Other(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MemoryRead(msg) => write!(f, "Memory read error: {msg}"),
            Error::Parse(msg) => write!(f, "Parse error: {msg}"),
            Error::NotAvailable(msg) => write!(f, "Not available: {msg}"),
            Error::FileOperation(msg) => write!(f, "File operation error: {msg}"),
            Error::Unsupported(msg) => write!(f, "Unsupported operation: {msg}"),
            Error::Other(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FileOperation(err.to_string())
    }
}

impl From<rosu_mem::error::ProcessError> for Error {
    fn from(err: rosu_mem::error::ProcessError) -> Self {
        Error::MemoryRead(err.to_string())
    }
}

impl From<rosu_mem::error::ParseSignatureError> for Error {
    fn from(err: rosu_mem::error::ParseSignatureError) -> Self {
        Error::Parse(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
