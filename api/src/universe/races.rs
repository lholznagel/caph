use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct RaceId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Race {
    pub alliance_id: AllianceId,
    pub description: String,
    pub name: String,
    pub race_id: RaceId,
}

impl EveClient {
    fetch!(fetch_races, "universe/races", Vec<Race>);
}
