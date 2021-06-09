mod blueprint;
mod item;
mod market_info;
mod market_order;
mod name;
mod reprocess;
mod schematic;
mod system_region;
mod user;

pub use self::blueprint::*;
pub use self::item::*;
pub use self::market_info::*;
pub use self::market_order::*;
pub use self::name::*;
pub use self::reprocess::*;
pub use self::schematic::*;
pub use self::system_region::*;
pub use self::user::*;

pub enum CacheName {
    Blueprint,
    Item,
    MarketInfo,
    MarketOrder,
    Name,
    Reprocess,
    Schematic,
    SystemRegion,
    User,
}

impl Into<u8> for CacheName {
    fn into(self) -> u8 {
        match self {
            Self::Blueprint    => 0,
            Self::Item         => 1,
            Self::MarketInfo   => 2,
            Self::MarketOrder  => 3,
            Self::Name         => 4,
            Self::Reprocess    => 5,
            Self::Schematic    => 6,
            Self::SystemRegion => 7,
            Self::User         => 8,
        }
    }
}
