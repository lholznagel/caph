use crate::ConnectError;
use crate::TypeId;
use crate::zip::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wrapper for reprocessing material
pub struct ConnectReprocessService {
    /// Cache of all entries that are in the zip file
    entries: HashMap<TypeId, ReprocessEntry>
}

impl ConnectReprocessService {
    /// Path in the zip file
    const PATH: &'static str = "sde/fsd/typeMaterials.yaml";

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
        let entries = zip.get_file(Self::PATH)?;

        Ok(Self {
            entries
        })
    }

    /// Gets the list of all entries
    ///
    /// # Returns
    ///
    /// List of all entries
    ///
    pub fn entries(&self) -> &HashMap<TypeId, ReprocessEntry> {
        &self.entries
    }
}

/// Represents the materials for a type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReprocessEntry {
    /// List of all materials that are recoverd when reprocessing
    #[serde(rename = "materials")]
    pub materials: Vec<MaterialEntry>,
}

/// Description of a type material
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialEntry {
    /// Type id of the material
    #[serde(rename = "materialTypeID")]
    pub type_id:  TypeId,
    /// Possible quantity of the material after reprocessing
    #[serde(rename = "quantity")]
    pub quantity:  i32
}
