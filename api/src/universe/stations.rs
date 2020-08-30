use crate::eve_client::*;
use crate::fetch;
use crate::universe::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct StationId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Station {
    pub max_dockable_ship_volume: u32,
    pub name: String,
    pub office_rental_cost: f32,
    pub position: Position,
    pub reprocessing_efficiency: f32,
    pub reprocessing_stations_take: f32,
    pub services: Vec<String>,
    pub station_id: StationId,
    pub system_id: SystemId,
    pub type_id: TypeId,

    pub owner: Option<u32>,
    pub race_id: Option<RaceId>,
}

impl EveClient {
    fetch!(fetch_station, "universe/stations", StationId, Station);
}
