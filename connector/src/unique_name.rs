use crate::{ConnectError, ItemId};
use crate::zip::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Wrapper for stations
pub struct ConnectUniqueNameService {
    /// All station ids mapped to there name
    entries: HashMap<ItemId, String>,
}

impl ConnectUniqueNameService {
    /// Path to the station file
    const PATH: &'static str = "sde/bsd/invUniqueNames.yaml";

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
        let entries = zip
            .get_file::<Vec<UnqiueNameEntry>>(Self::PATH)?
            .into_iter()
            .map(|x| (x.item_id, x.name))
            .collect::<HashMap<_, _>>();

        Ok(Self {
            entries
        })
    }

    /// Gets a list of all unique names
    ///
    /// # Returns
    ///
    /// List of all unique names
    ///
    pub fn entries(&self) -> &HashMap<ItemId, String> {
        &self.entries
    }
}

/// Represents a single name entry
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnqiueNameEntry {
    /// ID of the item
    #[serde(rename = "itemID")]
    pub item_id: ItemId,
    /// Name of the station
    #[serde(rename = "itemName")]
    pub name:    String
}
