mod fetch;
mod insert;

use crate::{LeftRight, MarketItemOrder, MarketOrderEntry, MarketOrderInfoCache};

pub use self::fetch::*;
pub use self::insert::*;

use std::collections::HashMap;
use std::sync::Arc;

type ItemId  = u32;
type OrderId = u64;
pub type Internal = HashMap<ItemId, HashMap<OrderId, Vec<MarketItemOrder>>>;

#[allow(dead_code)]
pub struct MarketOrderCacheV2 {
    cache:       LeftRight<Internal>,
    market_info: Arc<MarketOrderInfoCache>,
}

impl MarketOrderCacheV2 {
    pub fn new(market_info: Arc<MarketOrderInfoCache>) -> Self {
        Self {
            cache:       LeftRight::default(),
            market_info,
        }
    }
}

