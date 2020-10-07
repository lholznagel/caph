use std::error::Error;
use std::fmt;

pub(crate) type Result<T> = std::result::Result<T, EveApiError>;

#[derive(Debug)]
pub enum EveApiError {
    ReqwestError(surf::Error),
    TooManyRetries(String),
}

impl Error for EveApiError {}

impl fmt::Display for EveApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<surf::Error> for EveApiError {
    fn from(x: surf::Error) -> Self {
        EveApiError::ReqwestError(x)
    }
}
