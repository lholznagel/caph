use std::error::Error;
use std::fmt;

// pub type Result<T> = std::result::Result<T, EveServerError>;

#[derive(Debug)]
pub enum EveServerError {
    IoError(std::io::Error),
}

impl Error for EveServerError {}

impl fmt::Display for EveServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for EveServerError {
    fn from(x: std::io::Error) -> Self {
        EveServerError::IoError(x)
    }
}
