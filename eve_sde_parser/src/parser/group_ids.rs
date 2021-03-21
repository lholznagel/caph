use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupIds {
    pub anchorable: bool,
    pub anchored: bool,
    #[serde(rename = "categoryID")]
    pub category_id: u32,
    #[serde(rename = "fittableNonSingleton")]
    pub fittable_non_singleton: bool,
    #[serde(rename = "iconID")]
    pub icon_id: Option<u32>,
    pub name: HashMap<String, String>,
    pub published: bool,
    #[serde(rename = "useBasePrice")]
    pub use_base_price: bool,
}
