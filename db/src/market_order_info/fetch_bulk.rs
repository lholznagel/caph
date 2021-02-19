use crate::{Actions, MarketOrderInfoCache, MarketOrderInfoEntry};

use async_trait::async_trait;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchMarketOrderInfoBulkReq> for MarketOrderInfoCache {
    type Error    = EmptyResponse;
    type Response = FetchMarketOrderInfoResBulk;

    async fn fetch(&self, input: FetchMarketOrderInfoBulkReq) -> Result<Self::Response, Self::Error> {
        let mut ret = Vec::with_capacity(input.0.len());

        for x in input.0 {
            if let Some(x) = self.0.read().await.get(&x) {
                ret.push(x.clone());
            }
        }
        Ok(FetchMarketOrderInfoResBulk(ret))
    }
}

#[request(Actions::FetchMarketOrderInfoBulk)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoBulkReq(pub Vec<u64>);

#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoResBulk(pub Vec<MarketOrderInfoEntry>);
