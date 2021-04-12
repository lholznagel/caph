use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StationServiceEntry {
    #[serde(rename = "serviceNameID")]
    pub name:        HashMap<String, String>,
    #[serde(rename = "descriptionID")]
    #[serde(default)]
    pub description: HashMap<String, String>,
}
