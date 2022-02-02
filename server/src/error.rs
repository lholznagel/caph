use axum::body::BoxBody;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use hmac::digest::InvalidLength;
use serde_json::json;
use std::env::VarError;
use std::fmt;

// TODO: Rename to Error
#[derive(Debug)]
pub enum Error {
    /// Contains all errors that can come from the appraisal library
    AppraisalError(appraisal::Error),

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

    MissingEnvSecretKey(VarError),
    HmacInitError(InvalidLength),
    InvalidBase64(base64::DecodeError)
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError(e)
    }
}

impl From<caph_connector::ConnectError> for Error {
    fn from(e: caph_connector::ConnectError) -> Self {
        Self::ConnectError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::http::Response<BoxBody> {
        let (status, msg) = match self {
            Error::BadRequest   => (StatusCode::BAD_REQUEST, "Bad Request"),
            Error::InvalidUser  => (StatusCode::FORBIDDEN, "Forbidden"),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            Error::NotFound     => (StatusCode::NOT_FOUND, "Requested entry not found"),
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
