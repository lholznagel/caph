use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CategoryService(pub HashMap<CategoryId, CategoryEntry>);

impl CategoryService {
    const PATH: &'static str = "sde/fsd/categoryIDs.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CategoryEntry {
    #[serde(rename = "name")]
    pub name:      HashMap<String, String>,
    #[serde(rename = "published")]
    pub published: bool,

    #[serde(rename = "iconID")]
    pub icon_id:   Option<IconId>,
}
