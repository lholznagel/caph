mod blueprint;
mod id_name;
mod item_material;
mod item;
mod market_order;
mod market_order_info;
mod region;
mod station;

pub use self::blueprint::*;
pub use self::id_name::*;
pub use self::item_material::*;
pub use self::item::*;
pub use self::market_order::*;
pub use self::market_order_info::*;
pub use self::region::*;
pub use self::station::*;

#[derive(Debug)]
pub enum Actions {
    InsertBlueprints,

    FetchIdName,
    InsertIdNames,

    FetchItem,
    InsertItems,

    FetchItemMaterial,
    InsertItemMaterials,

    FetchMarketOrder,
    FetchLatestMarketOrders,
    InsertMarketOrders,

    FetchMarketOrderInfo,
    FetchMarketOrderInfoBulk,
    InsertMarketOrdersInfo,

    FetchRegions,
    InsertRegions,

    FetchStation,
    InsertStations,

    Invalid,
}

impl Into<u16> for Actions {
    fn into(self) -> u16 {
        match self {
            Self::InsertBlueprints         => 1u16,

            Self::FetchIdName              => 5u16,
            Self::InsertIdNames            => 6u16,

            Self::FetchItem                => 10u16,
            Self::InsertItems              => 11u16,

            Self::FetchItemMaterial        => 15u16,
            Self::InsertItemMaterials      => 16u16,
            
            Self::FetchMarketOrder         => 20u16,
            Self::FetchLatestMarketOrders  => 21u16,
            Self::InsertMarketOrders       => 22u16,

            Self::FetchMarketOrderInfo     => 25u16,
            Self::FetchMarketOrderInfoBulk => 26u16,
            Self::InsertMarketOrdersInfo   => 27u16,

            Self::FetchRegions             => 30u16,
            Self::InsertRegions            => 31u16,

            Self::FetchStation             => 35u16,
            Self::InsertStations           => 36u16,

            Self::Invalid                  => u16::MAX,
        }
    }
}

impl From<u16> for Actions {
    fn from(x: u16) -> Self {
        match x {
            1  => Actions::InsertBlueprints,

            5  => Actions::FetchIdName,
            6  => Actions::InsertIdNames,

            10 => Actions::FetchItem,
            11 => Actions::InsertItems,

            15 => Actions::FetchItemMaterial,
            16 => Actions::InsertItemMaterials,

            20 => Actions::FetchMarketOrder,
            21 => Actions::FetchLatestMarketOrders,
            22 => Actions::InsertMarketOrders,

            25 => Actions::FetchMarketOrderInfo,
            26 => Actions::FetchMarketOrderInfoBulk,
            27 => Actions::InsertMarketOrdersInfo,

            30 => Actions::FetchRegions,
            31 => Actions::InsertRegions,

            35 => Actions::FetchStation,
            36 => Actions::InsertStations,
            _  => {
                log::error!("Unrecognized actions {}", x);
                Actions::Invalid
            },
        }
    }
}
