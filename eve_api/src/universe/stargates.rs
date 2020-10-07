use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct StargateId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Stargate {
    pub destination: StargateDestination,
    pub name: String,
    pub position: Position,
    pub stargate_id: StargateId,
    pub system_id: SystemId,
    pub type_id: TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StargateDestination {
    pub stargate_id: StargateId,
    pub system_id: SystemId,
}

impl EveClient {
    fetch!(fetch_stargate, "universe/stargates", StargateId, Stargate);
}
