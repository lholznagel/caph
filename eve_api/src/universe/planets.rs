use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct PlanetId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Planet {
    pub name: String,
    pub planet_id: PlanetId,
    pub position: Position,
    pub system_id: SystemId,
    pub type_id: TypeId,
}

impl EveClient {
    fetch!(fetch_planet, "universe/planets", PlanetId, Planet);
}
