use crate::eve_client::*;
use crate::fetch;
use crate::universe::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct BloodlineId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bloodline {
    pub bloodline_id: BloodlineId,
    pub charisma: u32,
    pub corporation_id: CorporationId,
    pub description: String,
    pub intelligence: u32,
    pub memory: u32,
    pub name: String,
    pub perception: u32,
    pub race_id: RaceId,
    pub ship_type_id: u32,
    pub willpower: u32,
}

impl EveClient {
    fetch!(fetch_bloodlines, "universe/bloodlines", Vec<Bloodline>);
}
