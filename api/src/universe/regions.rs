use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Region {
    pub constellations: Vec<ConstellationId>,
    pub name: String,
    pub region_id: RegionId,
    pub description: Option<String>,
}

impl EveClient {
    fetch!(fetch_region, "universe/regions", RegionId, Region);
}
