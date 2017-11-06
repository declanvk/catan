use std::error::Error;
use std::io;
use std::fmt;
use futures::sync::mpsc;
use serde_json;
use tokio_serde_json;
use std::any;

#[derive(Debug)]
pub enum ClientError {
    Send(String),
    Io(io::Error),
    Serde(serde_json::error::Error),
    Other(String),
}

impl Error for ClientError {
    fn description(&self) -> &str {
        match *self {
            ClientError::Send(ref err) => err.as_str(),
            ClientError::Io(ref err) => err.description(),
            ClientError::Serde(ref err) => err.description(),
            ClientError::Other(ref err) => err.as_str(),
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client error! ({:?})", self)
    }
}

impl From<io::Error> for ClientError {
    fn from(src: io::Error) -> Self {
        ClientError::Io(src)
    }
}

impl From<tokio_serde_json::Error> for ClientError {
    fn from(src: tokio_serde_json::Error) -> Self {
        match src {
            tokio_serde_json::Error::Io(err) => ClientError::Io(err),
            tokio_serde_json::Error::Serde(err) => ClientError::Serde(err),
        }
    }
}

impl<T: any::Any> From<mpsc::SendError<T>> for ClientError {
    fn from(src: mpsc::SendError<T>) -> Self {
        ClientError::Send(src.description().to_owned())
    }
}

impl From<&'static str> for ClientError {
    fn from(src: &'static str) -> Self {
        ClientError::Other(src.to_owned())
    }
}

impl Into<io::Error> for ClientError {
    fn into(self) -> io::Error {
        match self {
            ClientError::Io(err) => err,
            other => io::Error::new(io::ErrorKind::Other, other.description()),
        }
    }
}
