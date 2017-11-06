use std::error::Error;
use std::fmt;
use std::io;
use serde_json;
use tokio_serde_json;

#[derive(Debug)]
pub enum ServerError {
    Io(io::Error),
    Serde(serde_json::Error),
    Other(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error! ({:?})", self)
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        match *self {
            ServerError::Io(ref err) => err.description(),
            ServerError::Other(ref err) => err.as_str(),
            ServerError::Serde(ref err) => err.description()
        }
    }
}

impl From<io::Error> for ServerError {
    fn from(src: io::Error) -> Self {
        ServerError::Io(src)
    }
}

impl From<tokio_serde_json::Error> for ServerError {
    fn from(src: tokio_serde_json::Error) -> Self {
        match src {
            tokio_serde_json::Error::Io(err) => ServerError::Io(err),
            tokio_serde_json::Error::Serde(err) => ServerError::Serde(err),
        }
    }
}

impl From<ServerError> for io::Error {
    fn from(src: ServerError) -> Self {
        match src {
            ServerError::Io(err) => err,
            other => io::Error::new(io::ErrorKind::Other, other.description()),
        }
    }
}
