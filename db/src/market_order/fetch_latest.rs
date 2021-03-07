use std::time::Instant;

use super::MarketOrderCache;

use crate::{Actions, MarketOrderEntry};

use async_trait::async_trait;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::market_order::latest::complete";

#[async_trait]
impl Fetch<FetchLatestMarketOrdersReq> for MarketOrderCache {
    type Response = FetchLatestMarketOrderRes;

    async fn fetch(&self, input: FetchLatestMarketOrdersReq) -> Self::Response {
        let timer = Instant::now();
        if let Some(x) = self.current.read().await.get(&input.0) {
            let res = x.clone();
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchLatestMarketOrderRes::Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchLatestMarketOrderRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchLatestMarketOrders)]
#[derive(Debug, Parse)]
pub struct FetchLatestMarketOrdersReq(pub u32);

#[derive(Debug, Parse)]
pub enum FetchLatestMarketOrderRes {
    Ok(Vec<MarketOrderEntry>),
    Err(EmptyMsg),
}
