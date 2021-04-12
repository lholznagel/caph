use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct GroupService(pub HashMap<GroupId, GroupEntry>);

impl GroupService {
    const PATH: &'static str = "sde/fsd/groupIDs.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroupEntry {
    #[serde(rename = "anchorable")]
    pub anchorable:             bool,
    #[serde(rename = "anchored")]
    pub anchored:               bool,
    #[serde(rename = "categoryID")]
    pub category_id:            CategoryId,
    #[serde(rename = "fittableNonSingleton")]
    pub fittable_non_singleton: bool,
    #[serde(rename = "name")]
    pub name:                   HashMap<String, String>,
    #[serde(rename = "published")]
    pub published:              bool,
    #[serde(rename = "useBasePrice")]
    pub use_base_price:         bool,

    #[serde(rename = "iconID")]
    pub icon_id:                Option<IconId>,
}
