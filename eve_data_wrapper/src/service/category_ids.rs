use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CategoryService {
    categories: HashMap<CategoryId, CategoryEntry>,
}

impl CategoryService {
    const PATH: &'static str = "sde/fsd/categoryIDs.yaml";

    pub fn new(mut zip: SdeZipArchive) -> Result<Self, EveConnectError> {
        Ok(Self {
            categories: crate::parse_zip_file(Self::PATH, &mut zip)?,
        })
    }
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
