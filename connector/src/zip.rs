use std::{io::Cursor, path::Path};

use zip::ZipArchive;

use crate::ConnectError;

/// Type alias for `ZipArchive<Cursor<Vec<u8>>>`
pub(crate) type CursorSdeZip = ZipArchive<Cursor<Vec<u8>>>;

/// Wrapper for managing the SDE.zip file
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
