/// Error that can be thrown by the application.
#[derive(Debug)]
pub enum Error {
    /// Thrown when a ENV is missing, contains the name of the missing ENV
    MissingEnv(String),
    /// Error when constructing a reqwest client fails
    CouldNotConstructClient(reqwest::Error),
    /// Error during request
    RequestError(reqwest::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
