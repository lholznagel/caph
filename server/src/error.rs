use std::error::Error;
use std::fmt;

use warp::reject::Reject;

#[derive(Debug)]
pub enum EveServerError {
    EveConnectError(caph_eve_data_wrapper::EveConnectError),
    IoError(std::io::Error),
    CachemError(cachem::CachemError),
    UserNotFound,
    NotFound,
}

impl Error for EveServerError {}

impl Reject for EveServerError {}

impl From<std::io::Error> for EveServerError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<cachem::CachemError> for EveServerError {
    fn from(e: cachem::CachemError) -> Self {
        Self::CachemError(e)
    }
}

impl From<caph_eve_data_wrapper::EveConnectError> for EveServerError {
    fn from(e: caph_eve_data_wrapper::EveConnectError) -> Self {
        Self::EveConnectError(e)
    }
}

impl fmt::Display for EveServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
