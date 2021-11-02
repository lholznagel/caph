use crate::ConnectError;
use crate::TypeId;
use crate::zip::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wrapper for schematics
pub struct ConnectSchematicService {
    /// Cache of all entries that are in the zip file
    entries: HashMap<TypeId, SchematicEntry>,
}

impl ConnectSchematicService {
    /// Path in the zip file
    const PATH: &'static str = "sde/fsd/planetSchematics.yaml";

    /// Creates a new instance of the service
    ///
    /// # Params
    ///
    /// * `zip` -> Service for the zip file
    ///
    /// # Errors
    ///
    /// Fails when the file is not in the zip or cannot be parsed.
    ///
    /// # Returns
    ///
    /// New instance
    ///
    pub fn new(zip: &mut SdeService) -> Result<Self, ConnectError> {
        let entries: HashMap<TypeId, SchematicEntry> = zip.get_file(Self::PATH)?;

        Ok(ConnectSchematicService {
            entries,
        })
    }

    /// Gets the list of all entries
    ///
    /// # Returns
    ///
    /// List of all entries
    ///
    pub fn entries(&self) -> &HashMap<TypeId, SchematicEntry> {
        &self.entries
    }
}

/// Model representing a schematic
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicEntry {
    /// Time it takes for a single cycle
    #[serde(rename = "cycleTime")]
    pub cycle_time: i32,
    /// Different translations
    #[serde(rename = "nameID")]
    pub name:       HashMap<String, String>,
    /// Pins stuff can connect to
    #[serde(rename = "pins")]
    pub pins:       Vec<TypeId>,
    /// Input and output types
    #[serde(rename = "types")]
    pub types:      HashMap<TypeId, SchematicType>,
}

impl SchematicEntry {
    /// Gets the product of the scheamtic
    ///
    /// # Returns
    ///
    /// TypeId of the product
    ///
    pub fn product(&self) -> Option<(TypeId, SchematicType)> {
        self
            .types
            .iter()
            .find(|(_, x)| !x.is_input)
            .map(|(x, y)| (*x, *y))
    }

    /// Gets all required materials of the scheamtic
    ///
    /// # Returns
    ///
    /// TypeIds of all required materials
    ///
    pub fn materials(&self) -> HashMap<TypeId, SchematicType> {
        self
            .types
            .iter()
            .filter(|(_, x)| x.is_input)
            .map(|(x, y)| (*x, *y))
            .collect::<HashMap<_, _>>()
    }
}

/// Represents a single input or output item
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicType {
    /// Defines if the item is a input or output
    #[serde(rename = "isInput")]
    pub is_input: bool,
    /// Quantity that is required or produced
    #[serde(rename = "quantity")]
    pub quantity: i32,
}
