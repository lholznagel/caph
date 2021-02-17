//! Thin error wrapper that is used in the application

use cachem::CachemError;

/// Type wrapper for a result
pub type CollectorResult<T> = std::result::Result<T, CollectorError>;

/// All errors that can be thrown in this module
#[derive(Debug)]
pub enum CollectorError {
    // There was an error downloading the sde.zip file
    DownloadSdeZip,
    // There was an error parsing the sde.zip file
    SdeParserError(caph_eve_sde_parser::EveSdeParserError),
    // There was an error with the database connection pool
    DbConnectionPoolError(cachem::CachemError),
    // There was an error with the database protocol
    DbProtocolError(cachem::CachemError),
}
impl std::error::Error for CollectorError {}

impl std::fmt::Display for CollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) 
    }
}

impl From<cachem::CachemError> for CollectorError {
    fn from(x: cachem::CachemError) -> Self {
        match x {
            CachemError::ConnectionPoolError(_) => Self::DbConnectionPoolError(x),
            _ => Self::DbProtocolError(x),
        }
    }
}
