use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request!",
            Self::InvalidEncoding => "Invalid Encoding!",
            Self::InvalidProtocol => "Invalid Protocol!",
            Self::InvalidMethod => "Invalid Method!",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

