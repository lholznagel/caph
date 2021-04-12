use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EveSdeParserError {
    LoadingService,
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    SerdeError(serde_yaml::Error),
    ZipError(zip::result::ZipError),
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

impl From<reqwest::Error> for EveSdeParserError {
    fn from(x: reqwest::Error) -> Self {
        Self::ReqwestError(x)
    }
}

impl From<serde_yaml::Error> for EveSdeParserError {
    fn from(x: serde_yaml::Error) -> Self {
        Self::SerdeError(x)
    }
}

impl From<zip::result::ZipError> for EveSdeParserError {
    fn from(x: zip::result::ZipError) -> Self {
        Self::ZipError(x)
    }
}
