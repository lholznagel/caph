use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct AsteroidBeltId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AsteroidBelt {
    pub name: String,
    pub position: Position,
    pub system_id: SystemId,
}

impl EveClient {
    fetch!(
        fetch_asteroid_belt,
        "universe/asteroid_belts",
        AsteroidBeltId,
        AsteroidBelt
    );
}
