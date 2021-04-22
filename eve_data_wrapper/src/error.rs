use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EveConnectError {
    CannotParse,
    EnvError(String),
    IoError(std::io::Error),
    LoadingService,
    OAuthPayload(String),
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
    TooManyRetries(String),
    Unauthorized,
    ZipError(zip::result::ZipError),
}

impl Error for EveConnectError {}

impl fmt::Display for EveConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for EveConnectError {
    fn from(x: std::io::Error) -> Self {
        Self::IoError(x)
    }
}

impl From<serde_json::Error> for EveConnectError {
    fn from(x: serde_json::Error) -> Self {
        Self::JsonError(x)
    }
}

impl From<serde_yaml::Error> for EveConnectError {
    fn from(x: serde_yaml::Error) -> Self {
        Self::YamlError(x)
    }
}

impl From<reqwest::Error> for EveConnectError {
    fn from(x: reqwest::Error) -> Self {
        Self::ReqwestError(x)
    }
}

impl From<zip::result::ZipError> for EveConnectError {
    fn from(x: zip::result::ZipError) -> Self {
        Self::ZipError(x)
    }
}

