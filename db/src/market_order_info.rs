mod fetch;
mod fetch_bulk;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::fetch_bulk::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use metrix_exporter::MetrixSender;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct MarketOrderInfoCache {
    cache: RwLock<HashMap<u64, MarketOrderInfoEntry>>,
    metrix: MetrixSender,
}

impl MarketOrderInfoCache {
    pub const CAPACITY: usize = 1_000_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct MarketOrderInfoEntry {
    /// Timestamp in seconds
    pub issued:       u64,
    pub order_id:     u64,
    // FIXME: this is the stationid and a u32
    pub location_id:  u64,
    pub system_id:    u32,
    pub item_id:      u32,
    pub volume_total: u32,
    pub price:        f32,
    /// true  -> buy
    /// false -> sell
    pub is_buy_order: bool,
}

impl MarketOrderInfoEntry {
    pub fn new(
        issued: u64,
        order_id: u64,
        location_id: u64,
        system_id: u32,
        item_id: u32,
        volume_total: u32,
        price: f32,
        is_buy_order: bool,
    ) -> Self {
        Self {
            issued,
            order_id,
            location_id,
            system_id,
            item_id,
            volume_total,
            price,
            is_buy_order,
        }
    }
}
