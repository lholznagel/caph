use crate::eve_client::*;
use crate::fetch;
use crate::universe::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct System {
    pub constellation_id: ConstellationId,
    pub name: String,
    pub position: Position,
    pub security_status: f32,
    pub system_id: SystemId,

    pub planets: Option<Vec<Planet>>,
    pub security_class: Option<String>,
    pub star_id: Option<StarId>,
    pub startgates: Option<Vec<StargateId>>,
    pub stations: Option<Vec<StationId>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Planet {
    pub asteroid_belts: Option<Vec<u32>>,
    pub moons: Option<Vec<u32>>,
    pub planet_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemJump {
    pub ship_jumps: u32,
    pub system_id: SystemId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemKill {
    pub npc_kills: u32,
    pub pod_kills: u32,
    pub ship_kills: u32,
    pub system_id: SystemId,
}

impl EveClient {
    fetch!(fetch_system, "universe/systems", SystemId, System);

    fetch!(fetch_system_jumps, "universe/system_jumps", Vec<SystemJump>);
    fetch!(fetch_system_kills, "universe/system_kills", Vec<SystemKill>);
}
