use std::error::Error;
use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub enum EveServerError {
    EveConnectError(caph_eve_data_wrapper::EveConnectError),
    CachemError(cachem::CachemError),
    SerdeJsonError(serde_json::Error),
    InvalidUser,
    BlueprintNotFound,
    TypeNotFound,
}

impl Error for EveServerError {}

impl Reject for EveServerError {}

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

impl From<serde_json::Error> for EveServerError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl fmt::Display for EveServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
