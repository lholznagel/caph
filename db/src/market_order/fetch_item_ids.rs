use std::time::Instant;

use super::MarketOrderCache;

use crate::Actions;

use async_trait::async_trait;
use cachem::{Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::market_order::item_ids::complete";

#[async_trait]
impl Fetch<FetchMarketOrderItemIdsReq> for MarketOrderCache {
    type Response = FetchMarketOrderItemIdsRes;

    async fn fetch(&self, _: FetchMarketOrderItemIdsReq) -> Self::Response {
        let timer = Instant::now();

        let ids = self
            .current
            .read()
            .await
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<u32>>();
        self.metrix.send_time(METRIC_FETCH, timer).await;
        FetchMarketOrderItemIdsRes(ids)
    }
}

#[request(Actions::FetchMarketOrderItemIds)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderItemIdsReq;

#[derive(Debug, Parse)]
pub struct FetchMarketOrderItemIdsRes(pub Vec<u32>);
