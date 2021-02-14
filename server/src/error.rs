use std::error::Error;
use std::fmt;

use warp::reject::Reject;

#[derive(Debug)]
pub enum EveServerError {
    IoError(std::io::Error),
    CachemError(cachem::CachemError),
}

impl Error for EveServerError {}

impl Reject for EveServerError {}

impl From<std::io::Error> for EveServerError {
    fn from(e: std::io::Error) -> Self {
        EveServerError::IoError(e)
    }
}

impl From<cachem::CachemError> for EveServerError {
    fn from(e: cachem::CachemError) -> Self {
        EveServerError::CachemError(e)
    }
}

impl fmt::Display for EveServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
