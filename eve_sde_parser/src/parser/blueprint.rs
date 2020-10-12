use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blueprint {
    pub activities: Activity,
    #[serde(rename = "blueprintTypeID")]
    pub blueprint_type_id: u32,
    #[serde(rename = "maxProductionLimit")]
    pub max_production_limit: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Activity {
    pub copying: Option<BlueprintAdditional>,
    pub invention: Option<BlueprintAdditional>,
    pub manufacturing: Option<BlueprintAdditional>,
    pub reaction: Option<BlueprintAdditional>,
    pub research_material: Option<BlueprintAdditional>,
    pub research_time: Option<BlueprintAdditional>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlueprintAdditional {
    pub materials: Option<Vec<Material>>,
    pub products: Option<Vec<Material>>,
    pub skills: Option<Vec<Skill>>,
    pub time: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
    pub quantity: u32,
    #[serde(rename = "typeID")]
    pub type_id: u32,
    pub probability: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    pub level: u32,
    #[serde(rename = "typeID")]
    pub type_id: u32,
}