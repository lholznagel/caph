use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct SystemService {
    abyssal:        Vec<SolarsystemEntry>,
    eve:            Vec<SolarsystemEntry>,
    penalty:        Vec<SolarsystemEntry>,
    wormhole:       Vec<SolarsystemEntry>,

    constellations: HashMap<ConstellationId, ConstellationEntry>,
    regions:        HashMap<RegionId, RegionEntry>,
}

impl SystemService {
    const PATH_ABYSSAL:        &'static str = "sde/fsd/universe/abyssal/";
    const PATH_EVE:            &'static str = "sde/fsd/universe/eve/";
    const PATH_PENALTY:        &'static str = "sde/fsd/universe/penalty/";
    const PATH_WORMHOLE:       &'static str = "sde/fsd/universe/wormhole/";

    const PATH_CONSTELLATIONS: &'static str = "universe/constellations";
    const PATH_REGIONS:        &'static str = "universe/regions";

    pub(crate) async fn new(
        eve_client: EveClient,
        mut zip:    SdeZipArchive
    ) -> Result<Self, EveConnectError> {
        let mut abyssal  = Vec::new();
        let mut eve      = Vec::new();
        let mut penalty  = Vec::new();
        let mut wormhole = Vec::new();

        for file in zip.file_names().map(|x| x.to_string()) {
            if !file.contains("solarsystem.staticdata") {
                continue;
            }

            if file.contains(Self::PATH_ABYSSAL) {
                abyssal.push(file);
            } else if file.contains(Self::PATH_EVE) {
                eve.push(file);
            } else if file.contains(Self::PATH_PENALTY) {
                penalty.push(file);
            } else if file.contains(Self::PATH_WORMHOLE) {
                wormhole.push(file);
            }
        }

        let mut abyssal_entries  = Vec::with_capacity(abyssal.len());
        let mut eve_entries      = Vec::with_capacity(eve.len());
        let mut penalty_entries  = Vec::with_capacity(penalty.len());
        let mut wormhole_entries = Vec::with_capacity(wormhole.len());

        for path in abyssal {
            abyssal_entries.push(crate::parse_zip_file(&path, &mut zip)?);
        }

        for path in eve {
            eve_entries.push(crate::parse_zip_file(&path, &mut zip)?);
        }

        for path in penalty {
            penalty_entries.push(crate::parse_zip_file(&path, &mut zip)?);
        }

        for path in wormhole {
            wormhole_entries.push(crate::parse_zip_file(&path, &mut zip)?);
        }

        let constellations = Self::fetch_constellations(eve_client.clone()).await?;
        let regions = Self::fetch_regions(eve_client).await?;

        Ok(Self {
            abyssal:  abyssal_entries,
            eve:      eve_entries,
            penalty:  penalty_entries,
            wormhole: wormhole_entries,

            constellations,
            regions,
        })
    }

    pub fn constellations(&self) -> &HashMap<ConstellationId, ConstellationEntry> {
        &self.constellations
    }

    pub fn eve_systems(&self) -> &Vec<SolarsystemEntry> {
        &self.eve
    }

    pub fn regions(&self) -> &HashMap<RegionId, RegionEntry> {
        &self.regions
    }

    pub fn region_ids(&self) -> Vec<&RegionId> {
        self.regions.iter().map(|(id, _)| id).collect()
    }

    pub fn find_region_by_system<S: Into<SolarSystemId>>(
        &self,
        system: S
    ) -> Option<RegionId> {
        let system: SolarSystemId = system.into();
        self
            .constellations
            .iter()
            .find(|(_, e)| e.systems.contains(&system))
            .map(|(_, e)| e.region_id)
    }

    async fn fetch_constellations(
        eve_client: EveClient
    ) -> Result<HashMap<ConstellationId, ConstellationEntry>, EveConnectError> {
        let mut constellations = HashMap::new();
        let constellation_ids: Vec<ConstellationId> = eve_client
            .fetch_page(&Self::PATH_CONSTELLATIONS)
            .await?;
        for constellation_id in constellation_ids {
            let info = eve_client
                .fetch(&format!("{}/{}", &Self::PATH_CONSTELLATIONS, *constellation_id))
                .await?
                .json()
                .await?;
            constellations.insert(constellation_id, info);
        }
        Ok(constellations)
    }

    async fn fetch_regions(
        eve_client: EveClient
    ) -> Result<HashMap<RegionId, RegionEntry>, EveConnectError> {
        let mut regions = HashMap::new();
        let region_ids: Vec<RegionId> = eve_client
            .fetch_page(&Self::PATH_REGIONS)
            .await?;
        for region_id in region_ids {
            let info = eve_client
                .fetch(&format!("{}/{}", &Self::PATH_REGIONS, *region_id))
                .await?
                .json()
                .await?;
            regions.insert(region_id, info);
        }
        Ok(regions)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SolarsystemEntry {
    #[serde(rename = "border")]
    pub border:                       bool,
    #[serde(rename = "center")]
    pub center:                       Vec<f32>,
    #[serde(rename = "corridor")]
    pub corridor:                     bool,
    #[serde(rename = "fringe")]
    pub fringe:                       bool,
    #[serde(rename = "hub")]
    pub hub:                          bool,
    #[serde(rename = "international")]
    pub international:                bool,
    #[serde(rename = "luminosity")]
    pub luminosity:                   f32,
    #[serde(rename = "max")]
    pub max:                          Vec<f32>,
    #[serde(rename = "min")]
    pub min:                          Vec<f32>,
    #[serde(rename = "planets")]
    pub planets:                      HashMap<PlanetId, Planet>,
    #[serde(rename = "radius")]
    pub radius:                       f32,
    #[serde(rename = "regional")]
    pub regional:                     bool,
    #[serde(rename = "security")]
    pub security:                     f32,
    #[serde(rename = "solarSystemID")]
    pub solar_system_id:              SolarSystemId,
    #[serde(rename = "solarSystemNameID")]
    pub solar_system_name_id:         u32, // FIXME
    #[serde(rename = "stargates")]
    pub stargates:                    HashMap<StargateId, Stargate>,

    #[serde(rename = "descriptionID")]
    pub description_id:               Option<u32>,
    #[serde(rename = "disallowedAnchorCategories")]
    pub disallowed_anchor_categories: Option<Vec<TypeId>>, // FIXME validate
    #[serde(rename = "disallowedAnchorGroups")]
    pub disallowed_anchor_groups:     Option<Vec<u32>>, // FIXME
    #[serde(rename = "factionID")]
    pub faction_id:                   Option<u32>, // FIXME
    #[serde(rename = "secondarySun")]
    pub seconday_sun:                 Option<SecondarySun>,
    #[serde(rename = "securityClass")]
    pub security_class:               Option<String>,
    #[serde(rename = "star")]
    pub star:                         Option<Star>,
    #[serde(rename = "sunTypeID")]
    pub sun_type_id:                  Option<TypeId>,
    #[serde(rename = "visualEffect")]
    pub visual_effect:                Option<String>,
    #[serde(rename = "wormholeClassID")]
    pub wormhole_class_id:            Option<u32>, // FIXME validate
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Planet {
    #[serde(rename = "asteroidBelts")]
    #[serde(default)]
    pub asteroid_belts:    HashMap<u32, AsteroidBelt>, // FIXME: id
    #[serde(rename = "celestialIndex")]
    pub celestial_index:   u32,
    #[serde(rename = "moons")]
    #[serde(default)]
    pub moons:             HashMap<u32, Moon>, // FIXME: id
    #[serde(rename = "npcStations")]
    #[serde(default)]
    pub npc_stations:      HashMap<u32, NpcStation>,
    #[serde(rename = "planetAttributes")]
    pub planet_attributes: PlanetAttribute,
    #[serde(rename = "position")]
    pub position:          Vec<f32>,
    #[serde(rename = "radius")]
    pub radius:            u32,
    #[serde(rename = "statistics")]
    pub statistics:        PlanetStatistic,
    #[serde(rename = "typeID")]
    pub type_id:           TypeId,

    #[serde(rename = "planetNameID")]
    pub planet_name_id:    Option<u32>, // FIXME
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlanetAttribute {
    #[serde(rename = "heightMap1")]
    pub height_map1:   u32,
    #[serde(rename = "heightMap2")]
    pub height_map2:   u32,
    #[serde(rename = "population")]
    pub population:    bool,
    #[serde(rename = "shaderPreset")]
    pub shader_preset: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlanetStatistic {
    #[serde(rename = "density")]
    pub density:         f32,
    #[serde(rename = "eccentricity")]
    pub eccentricity:    f32,
    #[serde(rename = "escapeVelocity")]
    pub escape_velocity: f32,
    #[serde(rename = "fragmented")]
    pub fragmented:      bool,
    #[serde(rename = "life")]
    pub life:            f32,
    #[serde(rename = "locked")]
    pub locked:          bool,
    #[serde(rename = "massDust")]
    pub mass_dust:       f32,
    #[serde(rename = "massGas")]
    pub mass_gas:        f32,
    #[serde(rename = "orbitPeriod")]
    pub orbit_period:    f32,
    #[serde(rename = "orbitRadius")]
    pub orbit_radius:    f32,
    #[serde(rename = "pressure")]
    pub pressure:        f32,
    #[serde(rename = "radius")]
    pub radius:          f32,
    #[serde(rename = "rotationRate")]
    pub rotation_rate:   f32,
    #[serde(rename = "spectralClass")]
    pub spectral_class:  String,
    #[serde(rename = "surfaceGravity")]
    pub surface_gravity: f32,
    #[serde(rename = "temperature")]
    pub temperature:     f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AsteroidBelt {
    #[serde(rename = "position")]
    pub position:              Vec<f32>,
    #[serde(rename = "typeID")]
    pub type_id:               TypeId,

    #[serde(rename = "statistics")]
    pub statistics:            Option<PlanetStatistic>,
    #[serde(rename = "asteroidBeltNameID")]
    pub asteroid_belt_name_id: Option<u32>, // FIXME
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Moon {
    #[serde(rename = "npcStations")]
    #[serde(default)]
    pub npc_stations:      HashMap<u32, NpcStation>,
    #[serde(rename = "planetAttributes")]
    pub planet_attributes: PlanetAttribute,
    #[serde(rename = "position")]
    pub position:          Vec<f32>,
    #[serde(rename = "radius")]
    pub radius:            u32,
    #[serde(rename = "typeID")]
    pub type_id:           TypeId,

    #[serde(rename = "moonNameID")]
    pub moon_name_id:      Option<u32>, // FIXME
    #[serde(rename = "statistics")]
    pub statistics:        Option<PlanetStatistic>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcStation {
    #[serde(rename = "graphicID")]
    pub graphic_id:                 GraphicId,
    #[serde(rename = "isConquerable")]
    pub is_conquerable:             bool,
    #[serde(rename = "operationID")]
    pub operation_id:               OperationId,
    #[serde(rename = "ownerID")]
    pub owner_id:                   u32, // FIXME id
    #[serde(rename = "position")]
    pub position:                   Vec<f32>,
    #[serde(rename = "reprocessingEfficiency")]
    pub reprocessing_efficiency:    f32,
    #[serde(rename = "reprocessingHangarFlag")]
    pub reprocessing_hangar_flag:   u32, // FIXME: id
    #[serde(rename = "reprocessingStationsTake")]
    pub reprocessing_stations_take: f32,
    #[serde(rename = "typeID")]
    pub type_id:                    TypeId,
    #[serde(rename = "useOperationName")]
    pub use_operation_name:         bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stargate {
    #[serde(rename = "destination")]
    pub destination: StargateId,
    #[serde(rename = "position")]
    pub positition:  Vec<f32>,
    #[serde(rename = "typeID")]
    pub type_id:     TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Star {
    #[serde(rename = "id")]
    pub id:         StarId,
    #[serde(rename = "radius")]
    pub radius:     u32,
    #[serde(rename = "statistics")]
    pub statistics: StarStatistics,
    #[serde(rename = "typeID")]
    pub type_id:    TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StarStatistics {
    #[serde(rename = "age")]
    pub age:            f32,
    #[serde(rename = "life")]
    pub life:           f32,
    #[serde(rename = "locked")]
    pub locked:         bool,
    #[serde(rename = "luminosity")]
    pub luminosity:     f32,
    #[serde(rename = "radius")]
    pub radius:         f32,
    #[serde(rename = "spectralClass")]
    pub spectral_class: String,
    #[serde(rename = "temperature")]
    pub temperature:    f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SecondarySun {
    #[serde(rename = "effectBeaconTypeID")]
    pub effect_beacon_type_id: TypeId,
    #[serde(rename = "itemID")]
    pub item_id:               u64, // FIXME
    #[serde(rename = "position")]
    pub position:              Vec<f32>,
    #[serde(rename = "typeID")]
    pub type_id:               TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegionEntry {
    #[serde(rename = "constellations")]
    pub constellations: Vec<ConstellationId>,
    #[serde(rename = "name")]
    pub name:           String,
    #[serde(rename = "region_id")]
    pub region_id:      RegionId,

    #[serde(rename = "description")]
    pub description:    Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstellationEntry {
    #[serde(rename = "constellation_id")]
    pub constellation_id: ConstellationId,
    #[serde(rename = "name")]
    pub name:             String,
    #[serde(rename = "position")]
    pub position:         Position,
    #[serde(rename = "region_id")]
    pub region_id:        RegionId,
    #[serde(rename = "systems")]
    pub systems:          Vec<SolarSystemId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Position {
    #[serde(rename = "x")]
    pub x: f32,
    #[serde(rename = "y")]
    pub y: f32,
    #[serde(rename = "z")]
    pub z: f32,
}
