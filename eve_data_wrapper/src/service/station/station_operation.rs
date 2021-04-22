use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StationOperationEntry {
    #[serde(rename = "activityID")]
    pub activity_id:          ActivityId,
    #[serde(rename = "border")]
    pub border:               f32,
    #[serde(rename = "corridor")]
    pub corridor:             f32,
    #[serde(rename = "descriptionID")]
    #[serde(default)]
    pub description:          HashMap<String, String>,
    #[serde(rename = "fringe")]
    pub fringe:               f32,
    #[serde(rename = "hub")]
    pub hub:                  f32,
    #[serde(rename = "manufacturingFactor")]
    pub manufacturing_factor: f32,
    #[serde(rename = "operationNameID")]
    pub operation_name:       HashMap<String, String>,
    #[serde(rename = "ratio")]
    pub ratio:                f32,
    #[serde(rename = "researchFactor")]
    pub researche_factor:     f32,
    #[serde(rename = "services")]
    pub services:             Vec<ServiceId>,
    #[serde(rename = "stationTypes")]
    #[serde(default)]
    pub station_types:        Option<HashMap<u8, TypeId>>,
}
