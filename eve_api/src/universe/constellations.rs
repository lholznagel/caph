use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Constellation {
    pub constellation_id: ConstellationId,
    pub name: String,
    pub position: Position,
    pub region_id: RegionId,
    pub systems: Vec<SystemId>,
}

impl EveClient {
    fetch!(
        fetch_constellation,
        "universe/constellations",
        ConstellationId,
        Constellation
    );
}
