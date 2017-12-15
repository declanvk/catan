use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use redis_async::error::Error as RedisError;
use std::net::AddrParseError;
use prost::{DecodeError, EncodeError};
use glob::{GlobError, PatternError};

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug)]
pub enum ServerError {
    Io(IoError),
    Redis(RedisError),
    Custom(String),
    AddrParse(AddrParseError),
    RespParse(String),
    ProstDecode(DecodeError),
    ProstEncode(EncodeError),
    Glob(GlobError),
    GlobPattern(PatternError),
    UuidGeneration,
    ServicePreconditionsNotMet,
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
            ServerError::Redis(ref err) => err.description(),
            ServerError::Custom(ref err) => err.as_ref(),
            ServerError::AddrParse(ref err) => err.description(),
            ServerError::RespParse(ref err) => err.as_ref(),
            ServerError::ProstDecode(ref err) => err.description(),
            ServerError::ProstEncode(ref err) => err.description(),
            ServerError::Glob(ref err) => err.description(),
            ServerError::GlobPattern(ref err) => err.description(),
            ServerError::UuidGeneration => "Uuid generator produced None value",
            ServerError::ServicePreconditionsNotMet => "Service preconditions were not met",
        }
    }
}

impl From<IoError> for ServerError {
    fn from(src: IoError) -> Self {
        ServerError::Io(src)
    }
}

impl From<RedisError> for ServerError {
    fn from(src: RedisError) -> Self {
        ServerError::Redis(src)
    }
}

impl From<AddrParseError> for ServerError {
    fn from(src: AddrParseError) -> Self {
        ServerError::AddrParse(src)
    }
}

impl From<EncodeError> for ServerError {
    fn from(src: EncodeError) -> Self {
        ServerError::ProstEncode(src)
    }
}

impl From<DecodeError> for ServerError {
    fn from(src: DecodeError) -> Self {
        ServerError::ProstDecode(src)
    }
}

impl From<GlobError> for ServerError {
    fn from(src: GlobError) -> Self {
        ServerError::Glob(src)
    }
}

impl From<PatternError> for ServerError {
    fn from(src: PatternError) -> Self {
        ServerError::GlobPattern(src)
    }
}