mod fetch;
mod fetch_item_ids;
mod fetch_raw;
mod insert;
mod storage;

use crate::MarketOrderInfoCache;

pub use self::fetch::*;
pub use self::fetch_item_ids::*;
pub use self::fetch_raw::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use metrix_exporter::MetrixSender;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

type ItemId  = u32;
type OrderId = u64;

pub struct MarketOrderCache {
    cache: RwLock<HashMap<ItemId, HashMap<OrderId, Vec<MarketItemOrder>>>>,
    market_info: Arc<MarketOrderInfoCache>,
    metrix: MetrixSender,
}

impl MarketOrderCache {
    pub const CAPACITY: usize = 1_000_000;

    pub fn new(metrix: MetrixSender, market_info: Arc<MarketOrderInfoCache>) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            market_info,
            metrix,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct MarketOrderEntry {
    pub order_id:      u64,
    pub timestamp:     u64,
    pub volume_remain: u32,
    pub type_id:       u32,
}

#[derive(Debug, Parse)]
pub struct MarketOrderSaveEntry {
    pub item_id: u32,
    pub entries: Vec<MarketItemOrder>,
}

#[derive(Clone, Debug, Parse, PartialEq)]
pub struct MarketItemOrder {
    pub timestamp: u64,
    pub order_id: u64,
    pub volume: u32,
}
