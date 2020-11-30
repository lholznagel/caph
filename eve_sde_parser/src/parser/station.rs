use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Station {
    #[serde(rename = "constellationID")]
    pub constellation_id: u32,
    #[serde(rename = "corporationID")]
    pub corporation_id: u32,
    #[serde(rename = "dockingCostPerVolume")]
    pub docking_cost_per_volume: f32,
    #[serde(rename = "maxShipVolumeDockable")]
    pub max_ship_volume_dockable: u64,
    #[serde(rename = "officeRentalCost")]
    pub office_rental_cost: u32,
    #[serde(rename = "operationID")]
    pub operation_id: u32,
    #[serde(rename = "regionID")]
    pub region_id: u32,
    #[serde(rename = "reprocessingEfficiency")]
    pub reprocessing_efficiency: f32,
    #[serde(rename = "reprocessingHangarFlag")]
    pub reprocessing_hangar_flag: u32,
    #[serde(rename = "reprocessingStationsTake")]
    pub reprocessing_stations_take: f32,
    pub security: f32,
    #[serde(rename = "solarSystemID")]
    pub solar_system_id: u32,
    #[serde(rename = "stationID")]
    pub station_id: u32,
    #[serde(rename = "stationName")]
    pub station_name: String,
    #[serde(rename = "stationTypeID")]
    pub station_type_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
