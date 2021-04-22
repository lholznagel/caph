//! Thin error wrapper that is used in the application

use cachem::CachemError;

/// All errors that can be thrown in this module
#[derive(Debug)]
pub enum CollectorError {
    SdeError(caph_eve_data_wrapper::EveConnectError),
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

impl From<caph_eve_data_wrapper::EveConnectError> for CollectorError {
    fn from(x: caph_eve_data_wrapper::EveConnectError) -> Self {
        Self::SdeError(x)
    }
}
