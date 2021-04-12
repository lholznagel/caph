use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BlueprintService(pub HashMap<TypeId, BlueprintEntry>);

impl BlueprintService {
    const PATH: &'static str = "sde/fsd/blueprints.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintEntry {
    #[serde(rename = "activities")]
    pub activities:           BlueprintActivity,
    #[serde(rename = "blueprintTypeID")]
    pub type_id:              TypeId,
    #[serde(rename = "maxProductionLimit")]
    pub max_production_limit: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintActivity {
    #[serde(rename = "copying")]
    pub copying:           Option<BlueprintAdditional>,
    #[serde(rename = "invention")]
    pub invention:         Option<BlueprintAdditional>,
    #[serde(rename = "manufacturing")]
    pub manufacturing:     Option<BlueprintAdditional>,
    #[serde(rename = "reaction")]
    pub reaction:          Option<BlueprintAdditional>,
    #[serde(rename = "research_material")]
    pub research_material: Option<BlueprintAdditional>,
    #[serde(rename = "research_time")]
    pub research_time:     Option<BlueprintAdditional>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintAdditional {
    #[serde(rename = "materials")]
    pub materials: Option<Vec<BlueprintMaterial>>,
    #[serde(rename = "products")]
    pub products:  Option<Vec<BlueprintMaterial>>,
    #[serde(rename = "skills")]
    pub skills:    Option<Vec<BlueprintSkill>>,
    #[serde(rename = "time")]
    pub time:      u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintMaterial {
    #[serde(rename = "quantity")]
    pub quantity:    u32,
    #[serde(rename = "typeID")]
    pub type_id:     TypeId,

    #[serde(rename = "probability")]
    pub probability: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintSkill {
    #[serde(rename = "level")]
    pub level:   u32,
    #[serde(rename = "typeID")]
    pub type_id: TypeId,
}
