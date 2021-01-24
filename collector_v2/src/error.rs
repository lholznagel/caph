//! Thin error wrapper that is used in the application

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
    DbConnectionPoolError(cachem_utils::CachemConnectionPoolError),
    // There was an error with the database protocol
    DbProtocolError(cachem_utils::CachemError),
}
impl std::error::Error for CollectorError {}

impl std::fmt::Display for CollectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) 
    }
}

impl From<cachem_utils::CachemConnectionPoolError> for CollectorError {
    fn from(x: cachem_utils::CachemConnectionPoolError) -> Self {
        Self::DbConnectionPoolError(x)
    }
}

impl From<cachem_utils::CachemError> for CollectorError {
    fn from(x: cachem_utils::CachemError) -> Self {
        Self::DbProtocolError(x)
    }
}
