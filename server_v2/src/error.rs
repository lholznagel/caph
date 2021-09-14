use std::convert::Infallible;
use std::error::Error;
use std::fmt;

use axum::Json;
use axum::body::{Bytes, Full};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

#[derive(Debug)]
pub enum ServerError {
    EveConnectError(caph_eve_data_wrapper::EveConnectError),
    SerdeJsonError(serde_json::Error),
    DatabaseError(sqlx::Error),
    ConnectError(caph_connector::ConnectError),

    InvalidUser
}

impl Error for ServerError {}

impl From<caph_eve_data_wrapper::EveConnectError> for ServerError {
    fn from(e: caph_eve_data_wrapper::EveConnectError) -> Self {
        Self::EveConnectError(e)
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError(e)
    }
}

impl From<caph_connector::ConnectError> for ServerError {
    fn from(e: caph_connector::ConnectError) -> Self {
        Self::ConnectError(e)
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for ServerError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let (status, msg) = match self {
            ServerError::InvalidUser => (StatusCode::FORBIDDEN, "Forbidden"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        };

        let body = Json(json!({
            "error": msg
        }));

        (status, body).into_response()
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
