use crate::{ConnectError, StationId, SystemId};
use crate::zip::*;
use serde::{Deserialize, Serialize};

/// Wrapper for stations
pub struct ConnectStationService {
    /// All station ids mapped to there name
    entries: Vec<StationEntry>,
}

impl ConnectStationService {
    /// Path to the station file
    const PATH: &'static str = "sde/bsd/staStations.yaml";

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
        let entries = zip.get_file(Self::PATH)?;

        Ok(ConnectStationService {
            entries
        })
    }

    /// Gets a list of all stations and their names
    ///
    /// # Returns
    ///
    /// List of all stations
    ///
    pub fn entries(&self) -> &Vec<StationEntry> {
        &self.entries
    }
}

/// Represents a single group entry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationEntry {
    /// Id of the station
    #[serde(rename = "stationID")]
    pub id:              StationId,
    /// Name of the station
    #[serde(rename = "stationName")]
    pub name:            String,
    /// Solar system this station is located in
    #[serde(rename = "solarSystemID")]
    pub solar_system_id: SystemId
}
