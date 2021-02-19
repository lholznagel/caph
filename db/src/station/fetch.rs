use crate::{Actions, StationCache, StationEntry};

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchStationReq> for StationCache {
    type Error    = EmptyResponse;
    type Response = FetchStationRes;

    async fn fetch(&self, input: FetchStationReq) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            return Ok(FetchStationRes(x.clone()))
        } else {
            return Err(EmptyResponse::default())
        }
    }
}

#[request(Actions::FetchStation)]
#[derive(Debug, Parse)]
pub struct FetchStationReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchStationRes(pub StationEntry);
