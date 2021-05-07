use super::MarketOrderCache;

use crate::{Actions, MarketItemOrder};

use async_trait::async_trait;
use cachem::{Fetch, Parse, request};

const _METRIC_FETCH: &'static str  = "efetch::market_order::raw::complete";

#[async_trait]
impl Fetch<FetchRawMarketOrderReq> for MarketOrderCache {
    type Response = FetchRawMarketOrderRes;

    async fn fetch(&self, input: FetchRawMarketOrderReq) -> Self::Response{
        let type_id = input.0;

        // Get all item entries that are newer than the given start timestamp
        let historic = self
            .cache
            .read()
            .await;

        let response = if let Some(x) = historic.get(&type_id) {
            x
                .into_iter()
                .map(|(_, x)| x.clone())
                .flatten()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        FetchRawMarketOrderRes(response)
    }
}

#[request(Actions::FetchRawMarketOrders)]
#[derive(Debug, Parse)]
pub struct FetchRawMarketOrderReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchRawMarketOrderRes(pub Vec<MarketItemOrder>);
