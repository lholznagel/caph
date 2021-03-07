use std::time::Instant;

use crate::{Actions, StationCache, StationEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::station::complete";

#[async_trait]
impl Fetch<FetchStationReq> for StationCache {
    type Response = FetchStationRes;

    async fn fetch(&self, input: FetchStationReq) -> Self::Response {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = x.clone();
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchStationRes::Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchStationRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchStation)]
#[derive(Debug, Parse)]
pub struct FetchStationReq(pub u32);

#[derive(Debug, Parse)]
pub enum FetchStationRes {
    Ok(StationEntry),
    Err(EmptyMsg),
}
