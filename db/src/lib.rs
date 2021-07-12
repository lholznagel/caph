mod blueprint;
mod character_asset;
mod character_blueprint;
mod character_fitting;
mod corporation_blueprint;
mod industry_cost;
mod item;
mod market_info;
mod market_order;
mod market_price;
mod name;
mod project;
mod reprocess;
mod schematic;
mod system_region;
mod user;

pub use self::blueprint::*;
pub use self::character_asset::*;
pub use self::character_blueprint::*;
pub use self::character_fitting::*;
pub use self::corporation_blueprint::*;
pub use self::industry_cost::*;
pub use self::item::*;
pub use self::market_info::*;
pub use self::market_order::*;
pub use self::market_price::*;
pub use self::name::*;
pub use self::project::*;
pub use self::reprocess::*;
pub use self::schematic::*;
pub use self::system_region::*;
pub use self::user::*;

pub enum CacheName {
    Blueprint,
    CharacterAsset,
    CharacterBlueprint,
    CharacterFitting,
    CorporationBlueprint,
    IndustryCost,
    Item,
    MarketInfo,
    MarketOrder,
    MarketPrice,
    Name,
    Project,
    Reprocess,
    Schematic,
    SystemRegion,
    User,
}

impl Into<u8> for CacheName {
    fn into(self) -> u8 {
        match self {
            Self::Blueprint            => 0,
            Self::CharacterAsset       => 1,
            Self::CharacterBlueprint   => 2,
            Self::CharacterFitting     => 3,
            Self::CorporationBlueprint => 4,
            Self::IndustryCost         => 5,
            Self::Item                 => 6,
            Self::MarketInfo           => 7,
            Self::MarketOrder          => 8,
            Self::MarketPrice          => 9,
            Self::Name                 => 10,
            Self::Project              => 11,
            Self::Reprocess            => 12,
            Self::Schematic            => 13,
            Self::SystemRegion         => 14,
            Self::User                 => 15,
        }
    }
}
