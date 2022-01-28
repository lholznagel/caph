use axum::body::BoxBody;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use hmac::digest::InvalidLength;
use serde_json::json;
use std::env::VarError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    SerdeJsonError(serde_json::Error),
    Database(sqlx::Error),
    DatabaseError(sqlx::Error),
    ConnectError(caph_connector::ConnectError),

    /// General non specified error
    GenericError(String),

    InvalidUser,
    NotFound,
    Unauthorized,
    BadRequest,

    /// The address could not be parsed
    CouldNotParseServerListenAddr,
    /// Could not start server
    CouldNotStartServer,

    /// A transaction could not be established
    TransactionBeginNotSuccessfull(sqlx::Error),
    /// The transactin could not be commited
    TransactionCommitNotSuccessfull(sqlx::Error),

    FromReqestError(axum::extract::rejection::ExtensionRejection),

    CaphCoreProject(caph_core::ProjectError),
    CaphCoreMarket(caph_core::MarketError),

    MissingEnvSecretKey(VarError),
    HmacInitError(InvalidLength),
    InvalidBase64(base64::DecodeError)
}

impl Error for ServerError {}

impl From<caph_core::ProjectError> for ServerError {
    fn from(x: caph_core::ProjectError) -> Self {
        Self::CaphCoreProject(x)
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
    fn into_response(self) -> axum::http::Response<BoxBody> {
        let (status, msg) = match self {
            ServerError::BadRequest   => (StatusCode::BAD_REQUEST, "Bad Request"),
            ServerError::InvalidUser  => (StatusCode::FORBIDDEN, "Forbidden"),
            ServerError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ServerError::NotFound     => (StatusCode::NOT_FOUND, "Requested entry not found"),
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
#[deprecated]
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
