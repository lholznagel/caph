use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RaceService(pub HashMap<RaceId, RaceEntry>);

impl RaceService {
    const PATH: &'static str = "sde/fsd/races.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RaceEntry {
    #[serde(rename = "descriptionID")]
    #[serde(default)]
    pub description:  HashMap<String, String>,
    #[serde(rename = "nameID")]
    pub name:         HashMap<String, String>,
    #[serde(rename = "skills")]
    #[serde(default)]
    pub skills:       HashMap<TypeId, u32>,

    #[serde(rename = "iconID")]
    pub icon_id:      Option<IconId>,
    #[serde(rename = "shipTypeID")]
    pub ship_type_id: Option<TypeId>,
}
