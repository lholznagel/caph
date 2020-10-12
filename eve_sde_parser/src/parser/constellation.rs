use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Constellation {
    pub center: Vec<f32>,
    #[serde(rename = "constellationID")]
    pub constellation_id: u32,
    #[serde(rename = "factionID")]
    pub faction_id: Option<u32>,
    pub max: Vec<f32>,
    pub min: Vec<f32>,
    #[serde(rename = "nameID")]
    pub name_id: u32,
    pub radius: f32,
    #[serde(rename = "wormholeClassID")]
    pub wormhole_class_id: Option<u32>,
}
