use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct NameService {
    pub names:  Vec<NameEntry>,
    pub unique: Vec<UniqueNameEntry>,
}

impl NameService {
    const NAME_PATH:        &'static str = "sde/bsd/invNames.yaml";
    const UNIQUE_NAME_PATH: &'static str = "sde/bsd/invUniqueNames.yaml";

    pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveSdeParserError> {
        Ok(Self {
            names:  service_file_gen!(zip, Self::NAME_PATH),
            unique: service_file_gen!(zip, Self::UNIQUE_NAME_PATH),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameEntry {
    #[serde(rename = "itemID")]
    pub item_id: u32, // FIXME
    #[serde(rename = "itemName")]
    pub name:    String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UniqueNameEntry {
    #[serde(rename = "groupID")]
    pub grou_id: GroupId,
    #[serde(rename = "itemID")]
    pub item_id: u32, // FIXME
    #[serde(rename = "itemName")]
    pub name:    String
}
