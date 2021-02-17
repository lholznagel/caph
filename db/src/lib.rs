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

    InsertIdNames,

    InsertItems,

    InsertItemMaterials,

    FetchMarketOrder,
    InsertMarketOrders,

    FetchMarketOrderInfo,
    InsertMarketOrdersInfo,

    FetchRegions,
    InsertRegions,

    InsertStations,
}

impl Into<u16> for Actions {
    fn into(self) -> u16 {
        match self {
            Self::InsertBlueprints       => 1u16,

            Self::InsertIdNames          => 6u16,

            Self::InsertItems            => 11u16,

            Self::InsertItemMaterials    => 16u16,

            Self::FetchMarketOrder       => 20u16,
            Self::InsertMarketOrders     => 21u16,

            Self::FetchMarketOrderInfo   => 25u16,
            Self::InsertMarketOrdersInfo => 26u16,

            Self::FetchRegions           => 30u16,
            Self::InsertRegions          => 31u16,

            Self::InsertStations         => 36u16,
        }
    }
}

impl From<u16> for Actions {
    fn from(x: u16) -> Self {
        match x {
            1  => Actions::InsertBlueprints,

            6  => Actions::InsertIdNames,

            11 => Actions::InsertItems,

            16 => Actions::InsertItemMaterials,

            20 => Actions::FetchMarketOrder,
            21 => Actions::InsertMarketOrders,

            25 => Actions::FetchMarketOrderInfo,
            26 => Actions::InsertMarketOrdersInfo,

            30 => Actions::FetchRegions,
            31 => Actions::InsertRegions,

            36 => Actions::InsertStations,
            _  => panic!("Unrecognized actions {}", x),
        }
    }
}
