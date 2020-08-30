use std::error::Error;
use std::fmt;

pub(crate) type Result<T> = std::result::Result<T, EveError>;

#[derive(Debug)]
pub enum EveError {
    ApiError(eve_online_api::EveApiError),
}

impl Error for EveError {}

impl fmt::Display for EveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<eve_online_api::EveApiError> for EveError {
    fn from(x: eve_online_api::EveApiError) -> Self {
        Self::ApiError(x)
    }
}