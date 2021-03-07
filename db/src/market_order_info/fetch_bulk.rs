use std::time::Instant;

use crate::{Actions, MarketOrderInfoCache, MarketOrderInfoEntry};

use async_trait::async_trait;
use cachem::{Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::market_order_info::bulk::complete";

#[async_trait]
impl Fetch<FetchMarketOrderInfoBulkReq> for MarketOrderInfoCache {
    type Response = FetchMarketOrderInfoResBulk;

    async fn fetch(&self, input: FetchMarketOrderInfoBulkReq) -> Self::Response {
        let timer = Instant::now();
        let mut ret = Vec::with_capacity(input.0.len());

        for x in input.0 {
            if let Some(x) = self.cache.read().await.get(&x) {
                ret.push(x.clone());
            }
        }
        let res = FetchMarketOrderInfoResBulk(ret);
        self.metrix.send_time(METRIC_FETCH, timer).await;
        res
    }
}

#[request(Actions::FetchMarketOrderInfoBulk)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoBulkReq(pub Vec<u64>);

#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoResBulk(pub Vec<MarketOrderInfoEntry>);
