use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DogmaAttributeCategoryEntry {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "name")]
    pub name:        String,
}
