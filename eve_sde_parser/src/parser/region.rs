use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Region {
    pub center: Vec<f32>,
    #[serde(rename = "descriptionID")]
    pub description_id: Option<u32>,
    #[serde(rename = "factionID")]
    pub faction_id: Option<u32>,
    pub max: Vec<f32>,
    pub min: Vec<f32>,
    #[serde(rename = "nameID")]
    pub name_id: u32,
    pub nebula: u32,
    #[serde(rename = "regionID")]
    pub region_id: u32,
    #[serde(rename = "wormholeClassID")]
    pub wormhole_class_id: Option<u32>,
}
