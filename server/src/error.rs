use std::error::Error;
use std::fmt;

use warp::{reject::Reject, Rejection};

#[derive(Debug)]
pub enum EveServerError {
    IoError(std::io::Error),
    SqlError(sqlx::Error),
}

impl Error for EveServerError {}

impl Reject for EveServerError {}

impl From<EveServerError> for Rejection {
    fn from(e: EveServerError) -> Self {
        warp::reject::custom(e)
    }
}

impl From<std::io::Error> for EveServerError {
    fn from(e: std::io::Error) -> Self {
        EveServerError::IoError(e)
    }
}

impl From<sqlx::Error> for EveServerError {
    fn from(e: sqlx::Error) -> Self {
        EveServerError::SqlError(e)
    }
}

impl fmt::Display for EveServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
