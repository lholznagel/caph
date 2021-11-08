use crate::{ConnectError, LocationId};
use crate::zip::*;
use serde::{Deserialize, Serialize};

/// Wrapper for systems
pub struct ConnectSystemService {
    /// All system ids mapped to there name
    entries: Vec<SystemEntry>,
}

impl ConnectSystemService {
    /// Creates a new instance of the service
    ///
    /// # Params
    ///
    /// * `zip` -> Service for the zip file
    ///
    /// # Errors
    ///
    /// Fails when the file is not in the zip file or parsing the file fails.
    ///
    /// # Returns
    ///
    /// New instance
    ///
    pub fn new(zip: &mut SdeService) -> Result<Self, ConnectError> {
        let files = zip
            .file_names()
            .filter(|x| x.contains("solarsystem.staticdata"))
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        let mut entries = Vec::new();
        for file in files {
            let entry = zip.get_file(&file)?;
            entries.push(entry);
        }

        Ok(Self {
            entries
        })
    }

    /// Gets a list of all systems
    ///
    /// # Returns
    ///
    /// List of all system ids
    ///
    pub fn entries(&self) -> &Vec<SystemEntry> {
        &self.entries
    }
}

/// Represents a single system entry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemEntry {
    /// Id of the system
    #[serde(rename = "solarSystemID")]
    pub id: LocationId,
}
