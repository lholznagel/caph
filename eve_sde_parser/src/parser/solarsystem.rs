use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Solarsystem {
    pub border: bool,
    pub center: Vec<f32>,
    pub corridor: bool,
    #[serde(rename = "descriptionID")]
    pub description_id: Option<u32>,
    #[serde(rename = "disallowedAnchorCategories")]
    pub disallowed_anchor_categories: Option<Vec<usize>>,
    // for some reason there is one file that contains this line
    #[serde(rename = "disallowedAnchorGroups")]
    pub disallowed_anchor_groups: Option<Vec<usize>>,
    #[serde(rename = "factionID")]
    pub faction_id: Option<u32>,
    pub fringe: bool,
    pub hub: bool,
    pub international: bool,
    pub luminosity: f32,
    pub max: Vec<f32>,
    pub min: Vec<f32>,
    pub planets: HashMap<u32, Planet>,
    pub radius: f64,
    pub regional: bool,
    pub security: f64,
    #[serde(rename = "securityClass")]
    pub security_class: Option<String>,
    #[serde(rename = "secondarySun")]
    pub secondary_sun: Option<SecondarySun>,
    #[serde(rename = "solarSystemID")]
    pub solar_system_id: u32,
    #[serde(rename = "solarSystemNameID")]
    pub solar_system_name_id: u32,
    pub star: Option<Star>,
    pub stargates: HashMap<u32, Stargate>,
    #[serde(rename = "sunTypeID")]
    pub sun_type_id: Option<u32>,
    #[serde(rename = "wormholeClassID")]
    pub wormhole_class_id: Option<u32>,
    #[serde(rename = "visualEffect")]
    pub visual_effect: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Planet {
    #[serde(rename = "asteroidBelts")]
    pub asteroid_belts: Option<HashMap<u32, AsteroidBelt>>,
    #[serde(rename = "celestialIndex")]
    pub celestial_index: usize,
    pub moons: Option<HashMap<u32, Moon>>,
    #[serde(rename = "npcStations")]
    pub npc_stations: Option<HashMap<u32, NpcStation>>,
    #[serde(rename = "planetAttributes")]
    pub planet_attributes: PlanetAttributes,
    #[serde(rename = "planetNameID")]
    pub planet_name_id: Option<u32>,
    pub position: Vec<f32>,
    pub radius: usize,
    pub statistics: PlanetStatistics,
    #[serde(rename = "typeID")]
    pub type_id: u32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Moon {
    #[serde(rename = "moonNameID")]
    pub moon_name_id: Option<u32>,
    #[serde(rename = "planetAttributes")]
    pub planet_attributes: PlanetAttributes,
    pub position: Vec<f32>,
    pub radius: usize,
    pub statistics: Option<PlanetStatistics>,
    #[serde(rename = "npcStations")]
    pub npc_stations: Option<HashMap<u32, NpcStation>>,
    #[serde(rename = "typeID")]
    pub type_id: u32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcStation {
    #[serde(rename = "graphicID")]
    pub graphic_id: u32,
    #[serde(rename = "isConquerable")]
    pub is_conquerable: bool,
    #[serde(rename = "operationID")]
    pub operation_id: u32,
    #[serde(rename = "ownerID")]
    pub owner_id: u32,
    pub position: Vec<f32>,
    #[serde(rename = "reprocessingEfficiency")]
    pub reprocessing_efficiency: f32,
    #[serde(rename = "reprocessingHangarFlag")]
    pub reprocessing_hangar_flag: f32,
    #[serde(rename = "reprocessingStationsTake")]
    pub reprocessing_stations_take: f32,
    #[serde(rename = "typeID")]
    pub type_id: u32,
    #[serde(rename = "useOperationName")]
    pub use_operation_name: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AsteroidBelt {
    #[serde(rename = "asteroidBeltNameID")]
    pub asteroid_belt_name_id: Option<u32>,
    pub position: Vec<f32>,
    pub statistics: Option<PlanetStatistics>,
    #[serde(rename = "typeID")]
    pub type_id: u32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlanetAttributes {
    #[serde(rename = "heightMap1")]
    pub height_map1: usize,
    #[serde(rename = "heightMap2")]
    pub height_map2: usize,
    pub population: bool,
    #[serde(rename = "shaderPreset")]
    pub shader_preset: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlanetStatistics {
    pub density: f64,
    pub eccentricity: f64,
    #[serde(rename = "escapeVelocity")]
    pub escape_velocity: f64,
    pub fragmented: bool,
    pub life: f32,
    pub locked: bool,
    #[serde(rename = "massDust")]
    pub mass_dust: f64,
    #[serde(rename = "massGas")]
    pub mass_gas: f64,
    #[serde(rename = "orbitPeriod")]
    pub orbit_period: f64,
    #[serde(rename = "orbitRadius")]
    pub orbit_radius: f64,
    pub pressure: f64,
    pub radius: f64,
    #[serde(rename = "rotationRate")]
    pub rotation_rate: f64,
    #[serde(rename = "spectralClass")]
    pub spectral_class: String,
    #[serde(rename = "surfaceGravity")]
    pub surface_gravity: f64,
    pub temperature: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Star {
    pub id: u32,
    pub radius: usize,
    pub statistics: StarStatistic,
    #[serde(rename = "typeID")]
    pub type_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StarStatistic {
    pub age: f64,
    pub life: f64,
    pub locked: bool,
    pub luminosity: f64,
    pub radius: f64,
    #[serde(rename = "spectralClass")]
    pub spectral_class: String,
    pub temperature: f64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stargate {
    pub destination: u32,
    pub position: Vec<f32>,
    #[serde(rename = "typeID")]
    pub type_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SecondarySun {
    #[serde(rename = "effectBeaconTypeID")]
    pub effect_beacon_type_id: u32,
    #[serde(rename = "itemID")]
    pub item_id: u32,
    pub position: Vec<f32>,
    #[serde(rename = "typeID")]
    pub type_id: u32,
}