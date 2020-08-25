use std::error::Error;
use std::fmt;

pub(crate) type Result<T> = std::result::Result<T, EveError>;

#[derive(Debug)]
pub enum EveError {
    ReqwestError(reqwest::Error),
}

impl Error for EveError {}

impl fmt::Display for EveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<reqwest::Error> for EveError {
    fn from(x: reqwest::Error) -> Self {
        EveError::ReqwestError(x)
    }
}
