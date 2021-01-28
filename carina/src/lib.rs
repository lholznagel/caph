mod blueprint;
mod file;
mod id_name;
mod item_material;
mod item;
mod market_order;
mod market_order_info;
mod region;
mod station;

pub use self::blueprint::*;
pub use self::file::*;
pub use self::id_name::*;
pub use self::item_material::*;
pub use self::item::*;
pub use self::market_order::*;
pub use self::market_order_info::*;
pub use self::region::*;
pub use self::station::*;

use cachem_utils::Parse;

#[macro_export]
macro_rules! parser_request {
    ($action:expr, $cache:expr, $struct:ident) => {
        #[async_trait::async_trait]
        impl ProtocolRequest for $struct {
            fn action(&self) -> u8 {
                $action.into()
            }

            fn cache_type(&self) -> u8 {
                $cache.into()
            }
        }
    };
}

#[derive(Debug, Default, Parse)]
pub struct EmptyResponse;

#[derive(Debug)]
pub enum Action {
    Fetch,
    Insert,
    Update,
    Delete,
    Lookup,
}

impl Into<u8> for Action {
    fn into(self) -> u8 {
        match self {
            Self::Fetch  => 0u8,
            Self::Insert => 1u8,
            Self::Update => 2u8,
            Self::Delete => 3u8,
            Self::Lookup => 4u8,
        }
    }
}

impl From<u8> for Action {
    fn from(x: u8) -> Self {
        match x {
            0   => Action::Fetch,
            1   => Action::Insert,
            2   => Action::Update,
            3   => Action::Delete,
            4   => Action::Lookup,
            _ => panic!("Unrecognized action {}", x),
        }
    }
}

#[derive(Debug)]
pub enum Caches {
    Blueprint,
    IdName,
    Item,
    ItemMaterial,
    MarketOrder,
    MarketOrderInfo,
    Region,
    Station,
}

impl Into<u8> for Caches {
    fn into(self) -> u8 {
        match self {
            Self::Blueprint          =>  0u8,
            Self::IdName             =>  1u8,
            Self::Item               =>  2u8,
            Self::ItemMaterial       =>  3u8,
            Self::MarketOrder        =>  7u8,
            Self::MarketOrderInfo    =>  8u8,
            Self::Region             =>  9u8,
            Self::Station            => 10u8,
        }
    }
}

impl From<u8> for Caches {
    fn from(x: u8) -> Self {
        match x {
            0  => Self::Blueprint,
            1  => Self::IdName,
            2  => Self::Item,
            3  => Self::ItemMaterial,
            7  => Self::MarketOrder,
            8  => Self::MarketOrderInfo,
            9  => Self::Region,
            10 => Self::Station,
            _ => panic!("Unrecognized cache type {}", x),
        }
    }
}
