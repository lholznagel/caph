use std::{net::AddrParseError, error::Error};
use std::fmt;

impl Error for CollectorError {}

#[derive(Debug)]
pub enum CollectorError {
    ClockRunsBackwards,
    DownloadSdeZip,
    ParseError(AddrParseError),
    SdeParserError(caph_eve_sde_parser::EveSdeParserError),
    SqlxError(sqlx::Error)
}

impl From<sqlx::Error> for CollectorError {
    fn from(e: sqlx::Error) -> Self {
        Self::SqlxError(e)
    }
}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}