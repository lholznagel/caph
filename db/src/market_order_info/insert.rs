use std::collections::HashMap;
use std::time::Instant;

use crate::{Actions, MarketOrderInfoCache, MarketOrderInfoEntry};

use async_trait::async_trait;
use cachem::{EmptyMsg, Insert, Parse, Storage, request};

const METRIC_INSERT:         &'static str = "insert::market_order_info::complete";
const METRIC_INSERT_ENTRIES: &'static str = "insert::market_order_info::entries";

#[async_trait]
impl Insert<InsertMarketOrderInfoReq> for MarketOrderInfoCache {
    type Error    = EmptyMsg;
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertMarketOrderInfoReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        let mut map = HashMap::new();
        let mut data = input.0;

        while let Some(x) = data.pop() {
            map
                .entry(x.order_id)
                .and_modify(|entry| {
                    if *entry != x {
                        *entry = x.clone();
                    }
                })
                .or_insert(x);
        }

        self.metrix.send_len(METRIC_INSERT_ENTRIES, map.len()).await;
        *self.cache.write().await = map;
        self.save_to_file().await.unwrap();

        self.metrix.send_time(METRIC_INSERT, timer).await;
        Ok(EmptyMsg::default())
    }
}

#[request(Actions::InsertMarketOrdersInfo)]
#[derive(Debug, Parse)]
pub struct InsertMarketOrderInfoReq(pub Vec<MarketOrderInfoEntry>);
