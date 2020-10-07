use crate::eve_client::*;
use crate::fetch;
use crate::universe::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct CorporationId(pub u32);

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct FactionId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Faction {
    pub corporation_id: CorporationId,
    pub description: String,
    pub faction_id: FactionId,
    pub us_unique: bool,
    pub name: String,
    pub size_factor: f32,
    pub station_count: u32,
    pub station_system_count: u32,

    pub militia_corporation_id: Option<CorporationId>,
    pub solar_system_id: Option<SolarSystemId>,
}

impl EveClient {
    fetch!(fetch_faction, "universe/factions", Vec<Faction>);
}
