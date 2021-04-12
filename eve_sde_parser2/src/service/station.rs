mod station_operation;
mod station_service;

pub use self::station_operation::*;
pub use self::station_service::*;

use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct StationService {
    pub operations: HashMap<OperationId, StationOperationEntry>,
    pub services:   HashMap<StationId, StationServiceEntry>,
    pub stations:   Vec<StationEntry>,
}

impl StationService {
    const PATH_OPERATIONS: &'static str = "sde/fsd/stationOperations.yaml";
    const PATH_SERVICES:   &'static str = "sde/fsd/stationServices.yaml";
    const PATH_STATION:    &'static str = "sde/bsd/staStations.yaml";

    pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveSdeParserError> {
        Ok(Self {
            operations: service_file_gen!(zip, Self::PATH_OPERATIONS),
            services:   service_file_gen!(zip, Self::PATH_SERVICES),
            stations:   service_file_gen!(zip, Self::PATH_STATION),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StationEntry {
    #[serde(rename = "constellationID")]
    pub constellation_id:           ConstellationId,
    #[serde(rename = "corporationID")]
    pub corporation_id:             CorporationId,
    #[serde(rename = "dockingCostPerVolume")]
    pub docking_cost_per_volume:    f32,
    #[serde(rename = "maxShipVolumeDockable")]
    pub max_ship_volume_dockable:   u64,
    #[serde(rename = "officeRentalCost")]
    pub office_rental_cost:         u32,
    #[serde(rename = "operationID")]
    pub operation_id:               OperationId,
    #[serde(rename = "regionID")]
    pub region_id:                  RegionId,
    #[serde(rename = "reprocessingEfficiency")]
    pub reprocessing_efficiency:    f32,
    #[serde(rename = "reprocessingHangarFlag")]
    pub reprocessing_hangar_flag:   u32,
    #[serde(rename = "reprocessingStationsTake")]
    pub reprocessing_stations_take: f32,
    #[serde(rename = "security")]
    pub security:                   f32,
    #[serde(rename = "solarSystemID")]
    pub solar_system_id:            SolarSystemId,
    #[serde(rename = "stationID")]
    pub station_id:                 StationId,
    #[serde(rename = "stationName")]
    pub station_name:               String,
    #[serde(rename = "stationTypeID")]
    pub station_type_id:            TypeId,
    #[serde(rename = "x")]
    pub x:                          f32,
    #[serde(rename = "y")]
    pub y:                          f32,
    #[serde(rename = "z")]
    pub z:                          f32,
}
