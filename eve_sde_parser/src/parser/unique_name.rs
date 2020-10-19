use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UniqueName {
    #[serde(rename = "groupID")]
    pub group_id: u32,
    #[serde(rename = "itemID")]
    pub item_id: u32,
    #[serde(rename = "itemName")]
    pub item_name: String
}