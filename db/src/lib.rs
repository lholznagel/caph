mod blueprint;
mod id_name;
mod item_material;
mod item;
mod market_order;
mod market_order_info;
mod region;
mod station;
mod user;

pub use self::blueprint::*;
pub use self::id_name::*;
pub use self::item_material::*;
pub use self::item::*;
pub use self::market_order::*;
pub use self::market_order_info::*;
pub use self::region::*;
pub use self::station::*;
pub use self::user::*;

use cachem::Action;

#[derive(Debug, Action)]
pub enum Actions {
    FetchBlueprint,
    InsertBlueprints,

    FetchIdName,
    FetchIdNameBulk,
    InsertIdNames,

    FetchItem,
    InsertItems,

    FetchItemMaterial,
    InsertItemMaterials,

    FetchMarketOrder,
    FetchMarketOrderItemIds,
    FetchLatestMarketOrders,
    InsertMarketOrders,

    FetchMarketOrderInfo,
    FetchMarketOrderInfoBulk,
    InsertMarketOrdersInfo,

    FetchRegions,
    InsertRegions,

    FetchStation,
    InsertStations,

    FetchUser,
    InsertUser,

    Invalid,
}
