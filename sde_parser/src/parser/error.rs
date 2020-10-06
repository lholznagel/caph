use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, EveSdeParserError>;

#[derive(Debug)]
pub enum EveSdeParserError {
    InvalidCompression,
    InvalidFileFormat,
    InvalidFilename,
    IoError(std::io::Error),
}

impl Error for EveSdeParserError {}

impl fmt::Display for EveSdeParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for EveSdeParserError {
    fn from(x: std::io::Error) -> Self {
        Self::IoError(x)
    }
}

impl From<std::string::FromUtf8Error> for EveSdeParserError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self::InvalidFilename
    }
}