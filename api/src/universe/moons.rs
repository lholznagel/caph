use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct MoonId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Moon {
    pub moon_id: MoonId,
    pub name: String,
    pub position: Position,
    pub system_id: SystemId,
}

impl EveClient {
    fetch!(fetch_moon, "universe/moons", MoonId, Moon);
}
