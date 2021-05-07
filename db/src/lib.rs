mod blueprint;
mod id_name;
mod item;
mod item_material;
mod leftright;
mod market_order;
mod market_order_v2;
mod market_order_info;
mod system_region;
mod user;

pub use self::blueprint::*;
pub use self::id_name::*;
pub use self::item::*;
pub use self::item_material::*;
pub use self::leftright::*;
pub use self::market_order::*;
pub use self::market_order_v2::*;
pub use self::market_order_info::*;
pub use self::system_region::*;
pub use self::user::*;

use cachem::{Action, Parse};

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
    FetchRawMarketOrders,
    InsertMarketOrders,

    FetchMarketOrdersV2,
    InsertMarketOrdersV2,
    CommitMarketOrdersV2,

    FetchMarketOrderInfo,
    FetchMarketOrderInfoBulk,
    InsertMarketOrdersInfo,

    FetchSystemRegion,
    InsertSystemRegions,

    FetchUser,
    InsertUser,

    Invalid,
}

#[async_trait::async_trait]
pub trait Commit<T: Parse> {
    type Response;
    async fn commit(&self, input: T) -> Self::Response;
}

