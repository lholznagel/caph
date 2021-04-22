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

    pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveConnectError> {
        Ok(Self {
            names:  crate::parse_zip_file(Self::NAME_PATH, &mut zip)?,
            unique: crate::parse_zip_file(Self::UNIQUE_NAME_PATH, &mut zip)?,
        })
    }

    pub fn collect_names(&self) -> HashMap<TypeId, String> {
        self
            .unique
            .iter()
            .map(|x| (x.item_id, x.name.clone()))
            .collect::<HashMap<_, _>>()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameEntry {
    #[serde(rename = "itemID")]
    pub item_id: TypeId,
    #[serde(rename = "itemName")]
    pub name:    String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UniqueNameEntry {
    #[serde(rename = "groupID")]
    pub grou_id: GroupId,
    #[serde(rename = "itemID")]
    pub item_id: TypeId,
    #[serde(rename = "itemName")]
    pub name:    String
}
