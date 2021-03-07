use std::time::Instant;

use crate::{Actions, MarketOrderInfoCache, MarketOrderInfoEntry};

use async_trait::async_trait;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::market_order_info::complete";

#[async_trait]
impl Fetch<FetchMarketOrderInfoReq> for MarketOrderInfoCache {
    type Response = FetchMarketOrderInfoRes;

    async fn fetch(&self, input: FetchMarketOrderInfoReq) -> Self::Response {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = x.clone();
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchMarketOrderInfoRes::Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchMarketOrderInfoRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchMarketOrderInfo)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoReq(pub u64);

#[derive(Debug, Parse)]
pub enum FetchMarketOrderInfoRes {
    Ok(MarketOrderInfoEntry),
    Err(EmptyMsg),
}
