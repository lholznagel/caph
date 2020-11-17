use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schematic {
    #[serde(rename = "cycleTime")]
    pub cycle_time: u32,
    #[serde(rename = "nameID")]
    pub name: HashMap<String, String>,
    pub pins: Vec<u32>,
    pub types: HashMap<u32, SchematicType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SchematicType {
    #[serde(rename = "isInput")]
    pub is_input: bool,
    pub quantity: u32
}