mod blueprint;
mod item_material;
mod item;
mod market_order;
mod market_order_info;
mod station;
mod user;

pub use self::blueprint::*;
pub use self::item_material::*;
pub use self::item::*;
pub use self::market_order::*;
pub use self::market_order_info::*;
pub use self::station::*;
pub use self::user::*;

use cachem::Action;

#[derive(Debug, Action)]
pub enum Actions {
    FetchBlueprint,
    InsertBlueprints,

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

    FetchStation,
    InsertStations,

    FetchUser,
    InsertUser,

    Invalid,
}
