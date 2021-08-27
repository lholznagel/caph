mod alliance_fittings;
mod blueprint;
mod character_asset;
mod character_blueprint;
mod character_fitting;
mod corporation_blueprint;
mod industry_cost;
mod item;
mod item_dogma;
mod login;
mod market_info;
mod market_order;
mod market_price;
mod name;
mod project;
mod reprocess;
mod schematic;
mod system_region;
mod user;

pub use self::alliance_fittings::*;
pub use self::blueprint::*;
pub use self::character_asset::*;
pub use self::character_blueprint::*;
pub use self::character_fitting::*;
pub use self::corporation_blueprint::*;
pub use self::industry_cost::*;
pub use self::item::*;
pub use self::item_dogma::*;
pub use self::login::*;
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
    AllianceFitting,
    Blueprint,
    CharacterAsset,
    CharacterBlueprint,
    CharacterFitting,
    CorporationBlueprint,
    IndustryCost,
    Item,
    ItemDogma,
    Login,
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
            Self::AllianceFitting      => 0,
            Self::Blueprint            => 1,
            Self::CharacterAsset       => 2,
            Self::CharacterBlueprint   => 3,
            Self::CharacterFitting     => 4,
            Self::CorporationBlueprint => 5,
            Self::IndustryCost         => 6,
            Self::Item                 => 7,
            Self::ItemDogma            => 8,
            Self::Login                => 9,
            Self::MarketInfo           => 10,
            Self::MarketOrder          => 11,
            Self::MarketPrice          => 12,
            Self::Name                 => 13,
            Self::Project              => 14,
            Self::Reprocess            => 15,
            Self::Schematic            => 16,
            Self::SystemRegion         => 17,
            Self::User                 => 18,
        }
    }
}
