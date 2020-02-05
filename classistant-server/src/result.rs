use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidReturnId,
    EmptyResponse,
    FieldNotFound,
    UserAlreadyExists,
    UserNotExists,
    GroupNotExists,
    PermissionDenied,
    WrongPassword,
    OperatorUserNotInGroup,
    DestUserNotInGroup,
    Io(io::Error),
    MySQL(mysql::Error),
    Base64Decode(base64::DecodeError),
    SerdeJson(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(src: io::Error) -> Error {
        Error::Io(src)
    }
}

impl From<mysql::Error> for Error {
    fn from(src: mysql::Error) -> Error {
        Error::MySQL(src)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(src: base64::DecodeError) -> Error {
        Error::Base64Decode(src)
    }
}

impl From<serde_json::Error> for Error {
    fn from(src: serde_json::Error) -> Error {
        Error::SerdeJson(src)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
