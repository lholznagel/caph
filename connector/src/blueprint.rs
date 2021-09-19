use crate::ConnectError;
use crate::TypeId;
use crate::zip::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Wrapper for blueprints
pub struct ConnectBlueprintService {
    /// Cache of all entries that are in the zip file
    entries: HashMap<TypeId, BlueprintEntry>
}

impl ConnectBlueprintService {
    /// Path in the zip file
    const PATH: &'static str = "sde/fsd/blueprints.yaml";

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

        Ok(ConnectBlueprintService {
            entries
        })
    }

    /// Gets the list of all entries
    ///
    /// # Returns
    ///
    /// List of all entries
    ///
    pub fn entries(&self) -> &HashMap<TypeId, BlueprintEntry> {
        &self.entries
    }
}

/// Blueprint model from SDE
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintEntry {
    /// Contains all available activities for this blueprint
    #[serde(rename = "activities")]
    pub activities:           BlueprintActivity,
    /// Id of the blueprint
    #[serde(rename = "blueprintTypeID")]
    pub type_id:              TypeId,
    /// Maximum number of runs
    #[serde(rename = "maxProductionLimit")]
    pub max_production_limit: i32,
}

/// Contains a list of all activities that a blueprint can have
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintActivity {
    /// Information about the copy activity
    #[serde(rename = "copying")]
    pub copying:           Option<BlueprintActivityInfo>,
    /// Information about the invention activity
    #[serde(rename = "invention")]
    pub invention:         Option<BlueprintActivityInfo>,
    /// Information about the manufacturing activity
    #[serde(rename = "manufacturing")]
    pub manufacturing:     Option<BlueprintActivityInfo>,
    /// Information about the reaction activity
    #[serde(rename = "reaction")]
    pub reaction:          Option<BlueprintActivityInfo>,
    /// Information about the research material activity
    #[serde(rename = "research_material")]
    pub research_material: Option<BlueprintActivityInfo>,
    /// Information about the research time activity
    #[serde(rename = "research_time")]
    pub research_time:     Option<BlueprintActivityInfo>,
}

/// Hold the information about an activity
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintActivityInfo {
    /// List of required materials
    #[serde(rename = "materials", default)]
    pub materials: Vec<BlueprintMaterial>,
    /// List of products that are produced
    #[serde(rename = "products", default)]
    pub products:  Vec<BlueprintMaterial>,
    /// List of skills that are required for this activity
    #[serde(rename = "skills", default)]
    pub skills:    Vec<BlueprintSkill>,
    /// Time it takes to finish the activity
    #[serde(rename = "time")]
    pub time:      i32,
}

/// Represents a material required for some of the activities
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintMaterial {
    /// Required amount
    #[serde(rename = "quantity")]
    pub quantity:    i32,
    /// Item that is required
    #[serde(rename = "typeID")]
    pub type_id:     TypeId,

    /// Probability that the product is produced
    #[serde(rename = "probability")]
    pub probability: Option<f32>,
}

/// Represents the skills required
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintSkill {
    /// Level of the skill
    #[serde(rename = "level")]
    pub level:   i32,
    /// Id of the skill
    #[serde(rename = "typeID")]
    pub type_id: TypeId,
}
