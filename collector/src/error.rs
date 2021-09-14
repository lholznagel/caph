//! Thin error wrapper that is used in the application

/// All errors that can be thrown in this module
#[derive(Debug)]
pub enum CollectorError {
    /// Wrapper for chrono errors
    ChronoError,
    /// Error getting data from sd
    SdeError(caph_eve_data_wrapper::EveConnectError),
    /// Generic database error
    DatabaseError(sqlx::Error)
}
impl std::error::Error for CollectorError {}

impl std::fmt::Display for CollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) 
    }
}

impl From<sqlx::Error> for CollectorError {
    fn from(x: sqlx::Error) -> Self {
        Self::DatabaseError(x)
    }
}

impl From<caph_eve_data_wrapper::EveConnectError> for CollectorError {
    fn from(x: caph_eve_data_wrapper::EveConnectError) -> Self {
        Self::SdeError(x)
    }
}

impl From<chrono::ParseError> for CollectorError {
    fn from(_: chrono::ParseError) -> Self {
        Self::ChronoError
    }
}
