use axum::body::{Bytes, Full};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use std::convert::Infallible;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    SerdeJsonError(serde_json::Error),
    DatabaseError(sqlx::Error),
    ConnectError(caph_connector::ConnectError),

    InvalidUser,
    NotFound,

    /// The address could not be parsed
    CouldNotParseServerListenAddr,
    /// Could not start server
    CouldNotStartServer,

    /// A transaction could not be established
    TransactionBeginNotSuccessfull(sqlx::Error),
    /// The transactin could not be commited
    TransactionCommitNotSuccessfull(sqlx::Error),
}

impl Error for ServerError {}

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
            ServerError::NotFound    => (StatusCode::NOT_FOUND, "Requested entry not found"),
            _ => {
                tracing::error!("Error {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
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
