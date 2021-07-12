mod blueprint;
mod category_ids;
mod character;
mod corporation;
mod dogma;
mod group_ids;
mod industry;
mod market;
mod meta_group;
mod name;
mod planet_schematic;
mod race;
mod research_agent;
mod skin;
mod station;
mod system;
mod typ;

use crate::{SdeZipArchive, error::EveConnectError, eve_client::EveClient};

pub use self::blueprint::*;
pub use self::category_ids::*;
pub use self::character::*;
pub use self::corporation::*;
pub use self::dogma::*;
pub use self::group_ids::*;
pub use self::industry::*;
pub use self::market::*;
pub use self::meta_group::*;
pub use self::name::*;
pub use self::planet_schematic::*;
pub use self::race::*;
pub use self::research_agent::*;
pub use self::skin::*;
pub use self::station::*;
pub use self::system::*;
pub use self::typ::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum ServiceGroupName {
    Blueprints,
    Categories,
    Character,
    Corporations,
    Dogmas,
    Groups,
    Industry,
    Market,
    MetaGroups,
    Names,
    PlanetSchematics,
    Races,
    ResearchAgents,
    Skins,
    Stations,
    Systems,
    Types,
}

impl ServiceGroupName {
    pub async fn service(
        &self,
        eve_client: EveClient,
        zip: SdeZipArchive
    ) -> Result<ServiceGroup, EveConnectError> {
        let r = match self {
            Self::Blueprints => ServiceGroup::Blueprints(BlueprintService::new(zip)?),
            Self::Categories => ServiceGroup::Categories(CategoryService::new(zip)?),
            Self::Character => ServiceGroup::Character(CharacterService::new(eve_client, zip)?),
            Self::Corporations => ServiceGroup::Corporations(CorporationService::new(zip)?),
            Self::Dogmas => ServiceGroup::Dogmas(DogmaService::new(zip)?),
            Self::Groups => ServiceGroup::Groups(GroupService::new(zip)?),
            Self::Industry => ServiceGroup::Industry(IndustryService::new(eve_client, zip)?),
            Self::Market => ServiceGroup::Market(MarketService::new(eve_client, zip)?),
            Self::MetaGroups => ServiceGroup::MetaGroups(MetaGroupService::new(zip)?),
            Self::Names => ServiceGroup::Names(NameService::new(zip)?),
            Self::PlanetSchematics => ServiceGroup::PlanetSchematics(PlanceSchematicService::new(zip)?),
            Self::Races => ServiceGroup::Races(RaceService::new(zip)?),
            Self::ResearchAgents => ServiceGroup::ResearchAgents(ResearchAgentService::new(zip)?),
            Self::Skins => ServiceGroup::Skins(SkinService::new(zip)?),
            Self::Stations => ServiceGroup::Stations(StationService::new(zip)?),
            Self::Systems => ServiceGroup::Systems(SystemService::new(eve_client, zip).await?),
            Self::Types => ServiceGroup::Types(TypeService::new(zip)?),
        };
        Ok(r)
    }
}

#[derive(Clone)]
pub enum ServiceGroup {
    Blueprints(BlueprintService),
    Categories(CategoryService),
    Character(CharacterService),
    Corporations(CorporationService),
    Dogmas(DogmaService),
    Groups(GroupService),
    Industry(IndustryService),
    Market(MarketService),
    MetaGroups(MetaGroupService),
    Names(NameService),
    PlanetSchematics(PlanceSchematicService),
    Races(RaceService),
    ResearchAgents(ResearchAgentService),
    Skins(SkinService),
    Stations(StationService),
    Systems(SystemService),
    Types(TypeService),
}
