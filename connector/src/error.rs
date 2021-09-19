/// Holds all possible errors that can occur in this library.
///
/// Besides that it contains helper functions for easier construction of errors.
///
#[derive(Debug)]
pub enum ConnectError {
    /// An ENV was not set, contains which variable is missing
    EnvNotSet(String),
    /// The EVE-Client could not be constructed
    CouldNotConstructClient(reqwest::Error),
    /// Could not parse the given url
    UrlParseError,
    /// The request to the given URL failed 3 times in a row
    TooManyRetries(String),

    /// Generic reqwest error
    ReqwestError(reqwest::Error),

    /// The payload could not be decoded
    OAuthPayloadDecode(base64::DecodeError),
    /// Could not parse the decoded payload
    OAuthParseError(serde_json::Error),
    /// Failed to parse the character id
    OAuthParseCharacterId(std::num::ParseIntError),

    /// Downloading the SDE.zip file failed
    SdeDownloadFailed(reqwest::Error),
    /// The requested file could not be found in the zip
    SdeFileNotFound(String),
    /// Error while parsing the file into yaml
    SdeParseError(serde_yaml::Error),
    /// Reading the SDE.zip file from disk failed
    SdeReadError(std::io::Error),
    /// Loading the SDE.zip file failed
    SdeZipLoadError(zip::result::ZipError),
}

impl ConnectError {
    /// Returns an error that the ENV `EVE_USER_AGENT` is not set
    ///
    pub fn env_user_agent() -> Self {
        Self::EnvNotSet("ENV 'EVE_USER_AGENT' is not set!".into())
    }

    /// Returns an error that the ENV `EVE_CALLBACK` is not set
    ///
    pub fn env_callback() -> Self {
        Self::EnvNotSet("ENV 'EVE_CALLBACK' is not set!".into())
    }

    /// Returns an error that the ENV `EVE_CLIENT_ID` is not set
    ///
    pub fn env_client_id() -> Self {
        Self::EnvNotSet("ENV 'EVE_CLIENT_ID' is not set!".into())
    }

    /// Returns an error that the ENV `EVE_SECRET_KEY` is not set
    ///
    pub fn env_secret_key() -> Self {
        Self::EnvNotSet("ENV 'EVE_SECRET_KEY' is not set!".into())
    }
}
