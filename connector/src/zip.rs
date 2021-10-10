use crate::ConnectError;

use serde::de::DeserializeOwned;
use std::{io::Cursor, path::Path};
use tracing::instrument;
use zip::ZipArchive;

/// Type alias for `ZipArchive<Cursor<Vec<u8>>>`
pub type CursorSdeZip = ZipArchive<Cursor<Vec<u8>>>;

/// Wrapper for managing the SDE.zip file
#[derive(Clone, Debug)]
pub struct SdeService(CursorSdeZip);

impl SdeService {
    /// URL to download the SDE.zip file
    const ZIP_URL:  &'static str = "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip";
    /// Location to store the downloaded zip file
    const ZIP_PATH: &'static str = "./sde.zip";

    /// Creates a new service.
    ///
    /// Initialy it checks if there is a `sde.zip` file present, if so the file
    /// is loaded.
    ///
    /// If there is no `sde.zip` present, it is downloaded from the URL and
    /// loaded.
    ///
    /// # Errors
    ///
    /// Errors if the file on disk couldn´t be read, the file download failed
    /// or if loading the zip file failed.
    ///
    /// # Returns
    ///
    /// New service instance
    ///
    pub async fn new() -> Result<Self, ConnectError> {
        let zip = if Path::new(Self::ZIP_PATH).exists() {
            std::fs::read("./sde.zip")
                .map_err(ConnectError::SdeReadError)
                .map(Cursor::new)?
        } else {
            Self::download_zip().await?
        };

        let zip = ZipArchive::new(zip)
            .map_err(ConnectError::SdeZipLoadError)?;
        Ok(Self (zip))
    }

    /// Takes a path and parses the file content into a defined structure.
    ///
    /// # Parameters
    ///
    /// * `T`    - Type the file should be parsed to (in most cases rust figures
    ///            out the type)
    /// * `path` - Path in the zip file for the file to parse
    ///
    /// # Errors
    ///
    /// When the file is not parsable or the path is not found in the zip.
    ///
    /// # Returns
    ///
    /// Parsed yaml version of the file, based on the generic parameter `T`
    ///
    #[instrument(level = "debug")]
    pub fn get_file<T>(
        &mut self,
        path: &str,
    ) -> Result<T, ConnectError>
        where T: DeserializeOwned {

        let mut file = self.0
            .by_name(path)
            .map_err(|_| ConnectError::SdeFileNotFound(path.into()))?;
        serde_yaml::from_reader(&mut file)
            .map_err(ConnectError::SdeParseError)
    }

    /// List of all filenames in the zip file
    ///
    /// # Returns
    ///
    /// List of all filenames
    ///
    pub fn file_names(&self) -> impl Iterator<Item = &str> {
        self.0.file_names()
    }

    /// Downloads the SDE.zip file.
    ///
    /// # Errors
    ///
    /// Failes when the server returns an error
    ///
    /// # Returns
    ///
    /// Downloaded SDE.zip file as [Cursor].
    ///
    async fn download_zip() -> Result<Cursor<Vec<u8>>, ConnectError> {
        reqwest::get(Self::ZIP_URL)
            .await
            .map_err(ConnectError::ReqwestError)?
            .bytes()
            .await
            .map(|x| x.to_vec())
            .map(Cursor::new)
            .map_err(ConnectError::SdeDownloadFailed)
    }
}
