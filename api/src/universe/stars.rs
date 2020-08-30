use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct StarId(pub u32);

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct SolarSystemId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Star {
    pub age: u64,
    pub luminosity: f32,
    pub name: String,
    pub radius: u64,
    pub solar_system_id: SolarSystemId,
    pub spectral_class: String,
    pub temperature: u32,
    pub type_id: TypeId,
}

impl EveClient {
    fetch!(fetch_star, "universe/stars", StarId, Star);
}
