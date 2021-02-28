use std::time::Instant;

use crate::{Actions, MarketOrderInfoCache, MarketOrderInfoEntry};

use async_trait::async_trait;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::market_order_info::complete";

#[async_trait]
impl Fetch<FetchMarketOrderInfoReq> for MarketOrderInfoCache {
    type Error    = EmptyMsg;
    type Response = FetchMarketOrderInfoRes;

    async fn fetch(&self, input: FetchMarketOrderInfoReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = FetchMarketOrderInfoRes(x.clone());
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchMarketOrderInfo)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoReq(pub u64);

#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoRes(pub MarketOrderInfoEntry);
