use super::MarketOrderCache;

use crate::{Actions, MarketOrderEntry};

use async_trait::async_trait;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchLatestMarketOrdersReq> for MarketOrderCache {
    type Error    = EmptyResponse;
    type Response = FetchLatestMarketOrderRes;

    async fn fetch(&self, input: FetchLatestMarketOrdersReq) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.current.read().await.get(&input.0) {
            Ok(FetchLatestMarketOrderRes(x.clone()))
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[request(Actions::FetchLatestMarketOrders)]
#[derive(Debug, Parse)]
pub struct FetchLatestMarketOrdersReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchLatestMarketOrderRes(pub Vec<MarketOrderEntry>);
