use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MetaGroupService {
    groups: HashMap<u32, MetaGroupEntry>,
}

impl MetaGroupService {
    const PATH: &'static str = "sde/fsd/metaGroups.yaml";

    pub fn new(mut zip: SdeZipArchive) -> Result<Self, EveConnectError> {
        Ok(Self {
            groups: crate::parse_zip_file(Self::PATH, &mut zip)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetaGroupEntry {
    #[serde(rename = "descriptionID")]
    #[serde(default)]
    pub description: HashMap<String, String>,
    #[serde(rename = "nameID")]
    pub name:        HashMap<String, String>,

    #[serde(rename = "iconID")]
    pub icon_id:     Option<IconId>,
    #[serde(rename = "iconSuffix")]
    pub icon_suffix: Option<String>,
}
