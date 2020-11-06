use crate::eve_client::*;
use crate::universe::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct IconId(pub u32);

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct AncestryId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ancestry {
    pub bloodline_id: BloodlineId,
    pub description: String,
    pub id: AncestryId,
    pub name: String,

    pub icon_id: Option<IconId>,
    pub short_description: Option<String>
}

impl EveClient {
    fetch!(fetch_ancestries, "universe/ancestries", Vec<Ancestry>);
}
