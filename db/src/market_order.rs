mod fetch;
mod fetch_latest;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::fetch_latest::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::{CachemError, Parse, Storage};
use metrix_exporter::MetrixSender;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct MarketOrderCache {
    current: RwLock<HashMap<u32, Vec<MarketOrderEntry>>>,
    history: RwLock<HashMap<u32, Vec<MarketItemOrderId>>>,
    metrix: Option<MetrixSender>,
}

impl MarketOrderCache {
    pub const CAPACITY: usize = 1_000_000;

    const METRIC_INSERT_DURATION: &'static str = "caph_db::market_order::insert::duration";
    const METRIC_FETCH_DURATION: &'static str  = "caph_db::market_order::fetch::duration";

    pub async fn new(metrix_sender: MetrixSender) -> Result<Self, CachemError> {
        let mut _self = Self::load_from_file().await?;
        _self.metrix = Some(metrix_sender);
        Ok(_self)
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
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
