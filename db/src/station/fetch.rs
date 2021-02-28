use std::time::Instant;

use crate::{Actions, StationCache, StationEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::station::complete";

#[async_trait]
impl Fetch<FetchStationReq> for StationCache {
    type Error    = EmptyMsg;
    type Response = FetchStationRes;

    async fn fetch(&self, input: FetchStationReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = FetchStationRes(x.clone());
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchStation)]
#[derive(Debug, Parse)]
pub struct FetchStationReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchStationRes(pub StationEntry);
