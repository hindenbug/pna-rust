use failure::Fail;
use std::io;
use std::string;

/// Error type.
#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Removing non-existent key error.
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "Engine not found")]
    EngineNotFound,
    /// Error with a string message
    #[fail(display = "{}", _0)]
    StringError(String),
    // Sled error.
    #[fail(display = "{}", _0)]
    Sled(#[cause] sled::Error),
    /// Utf8 error.
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[fail(cause)] string::FromUtf8Error),
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::Sled(err)
    }
}

impl From<string::FromUtf8Error> for KvsError {
    fn from(err: string::FromUtf8Error) -> KvsError {
        KvsError::Utf8(err)
    }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;
