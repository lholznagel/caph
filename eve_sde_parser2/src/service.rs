mod blueprint;
mod category_ids;
mod corporation;
mod dogma;
mod group_ids;
mod meta_group;
mod name;
mod planet_schematic;
mod race;
mod research_agent;
mod skin;
mod station;
mod type_id;
mod type_material;

use crate::{SdeZipArchive, error::EveSdeParserError};

pub use self::blueprint::*;
pub use self::category_ids::*;
pub use self::corporation::*;
pub use self::dogma::*;
pub use self::group_ids::*;
pub use self::meta_group::*;
pub use self::name::*;
pub use self::planet_schematic::*;
pub use self::race::*;
pub use self::research_agent::*;
pub use self::skin::*;
pub use self::station::*;
pub use self::type_id::*;
pub use self::type_material::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum SdeServiceName {
    Blueprints,
    Categories,
    Corporations,
    Dogmas,
    Groups,
    MetaGroups,
    Names,
    PlanetSchematics,
    Races,
    ResearchAgents,
    Skins,
    Stations,
    TypeIds,
    TypeMaterials,
}

impl SdeServiceName {
    pub fn service(&self, zip: SdeZipArchive) -> Result<SdeService, EveSdeParserError> {
        let r = match self {
            Self::Blueprints => SdeService::Blueprints(BlueprintService::new(zip)?),
            Self::Categories => SdeService::Categories(CategoryService::new(zip)?),
            Self::Corporations => SdeService::Corporations(CorporationService::new(zip)?),
            Self::Dogmas => SdeService::Dogmas(DogmaService::new(zip)?),
            Self::Groups => SdeService::Groups(GroupService::new(zip)?),
            Self::MetaGroups => SdeService::MetaGroups(MetaGroupService::new(zip)?),
            Self::Names => SdeService::Names(NameService::new(zip)?),
            Self::PlanetSchematics => SdeService::PlanetSchematics(PlanceSchematicService::new(zip)?),
            Self::Races => SdeService::Races(RaceService::new(zip)?),
            Self::ResearchAgents => SdeService::ResearchAgents(ResearchAgentService::new(zip)?),
            Self::Skins => SdeService::Skins(SkinService::new(zip)?),
            Self::Stations => SdeService::Stations(StationService::new(zip)?),
            Self::TypeIds => SdeService::TypeIds(TypeIdService::new(zip)?),
            Self::TypeMaterials => SdeService::TypeMaterials(TypeMaterialService::new(zip)?),
        };
        Ok(r)
    }
}

#[derive(Clone)]
pub enum SdeService {
    Blueprints(BlueprintService),
    Categories(CategoryService),
    Corporations(CorporationService),
    Dogmas(DogmaService),
    Groups(GroupService),
    MetaGroups(MetaGroupService),
    Names(NameService),
    PlanetSchematics(PlanceSchematicService),
    Races(RaceService),
    ResearchAgents(ResearchAgentService),
    Skins(SkinService),
    Stations(StationService),
    TypeIds(TypeIdService),
    TypeMaterials(TypeMaterialService),
}
