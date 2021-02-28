use std::time::Instant;

use super::MarketOrderCache;

use crate::{Actions, MarketOrderEntry};

use async_trait::async_trait;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::market_order::latest::complete";

#[async_trait]
impl Fetch<FetchLatestMarketOrdersReq> for MarketOrderCache {
    type Error    = EmptyMsg;
    type Response = FetchLatestMarketOrderRes;

    async fn fetch(&self, input: FetchLatestMarketOrdersReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        if let Some(x) = self.current.read().await.get(&input.0) {
            let res = FetchLatestMarketOrderRes(x.clone());
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchLatestMarketOrders)]
#[derive(Debug, Parse)]
pub struct FetchLatestMarketOrdersReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchLatestMarketOrderRes(pub Vec<MarketOrderEntry>);
