use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EveApiError {
    ReqwestError(reqwest::Error),
    TooManyRetries(String),
    CouldNotPerform(String),
    EnvError(String),
    OAuthPayload(String),
    ParseError(serde_json::Error),
    Unauthorized,
}

impl Error for EveApiError {}

impl fmt::Display for EveApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<reqwest::Error> for EveApiError {
    fn from(x: reqwest::Error) -> Self {
        EveApiError::ReqwestError(x)
    }
}

impl From<serde_json::Error> for EveApiError {
    fn from(x: serde_json::Error) -> Self {
        Self::ParseError(x)
    }
}
