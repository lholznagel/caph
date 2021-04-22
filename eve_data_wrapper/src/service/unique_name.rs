use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UniqueNameService(Vec<UniqueNameEntry>);

impl UniqueNameService {
    const PATH: &'static str = "sde/bsd/invUniqueNames.yaml";

    service_gen!();
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
