mod fetch;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct MarketOrderCache {
    current: RwLock<HashMap<u64, MarketOrderEntry>>,
    history: RwLock<HashMap<u32, Vec<MarketItemOrderId>>>,
}

impl MarketOrderCache {
    pub const CAPACITY: usize = 1_000_000;
}

#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct MarketOrderEntry {
    pub order_id:      u64,
    pub timestamp:     u64,
    pub volume_remain: u32,
    pub item_id:       u32,
}

impl MarketOrderEntry {
    pub fn new(
        order_id: u64,
        timestamp: u64,
        volume_remain: u32,
        item_id: u32,
    ) -> Self {
        Self {
            order_id,
            timestamp,
            volume_remain,
            item_id,
        }
    }
}

#[derive(Debug, Parse)]
pub struct MarketOrderSaveEntry {
    pub item_id: u32,
    pub entries: Vec<MarketItemOrderId>,
}

#[derive(Clone, Debug, Parse)]
pub struct MarketItemOrderId {
    pub timestamp: u64,
    pub order_id: u64,
    pub volume: u32,
}
