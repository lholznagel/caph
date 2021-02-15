use crate::{Actions, Caches, EmptyResponse, MarketOrderInfoCache, MarketOrderInfoEntry};

use async_trait::async_trait;
use cachem::{Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchMarketOrderInfoReq> for MarketOrderInfoCache {
    type Error    = EmptyResponse;
    type Response = FetchMarketOrderInfoRes;

    async fn fetch(&self, input: FetchMarketOrderInfoReq) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            Ok(FetchMarketOrderInfoRes(x.clone()))
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[request(Actions::Fetch, Caches::MarketOrderInfo)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoReq(pub u64);

#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoRes(pub MarketOrderInfoEntry);
