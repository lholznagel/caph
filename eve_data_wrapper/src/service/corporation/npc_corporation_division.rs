use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcCorporationDivisionEntry {
    #[serde(rename = "descriptionID")]
    #[serde(default)]
    pub description:         HashMap<String, String>,
    #[serde(rename = "leaderTypeNameID")]
    pub leader_name:         HashMap<String, String>,
    #[serde(rename = "nameID")]
    pub name:                HashMap<String, String>,

    #[serde(rename = "description")]
    pub description_general: Option<String>,
}
