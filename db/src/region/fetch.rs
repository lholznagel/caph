use std::time::Instant;

use crate::{Actions, RegionCache, RegionEntries};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::regions::complete";

#[async_trait]
impl Fetch<FetchRegionReq> for RegionCache {
    type Error    = EmptyMsg;
    type Response = RegionEntries;

    async fn fetch(&self, _input: FetchRegionReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        let entries = self.cache.read().await;
        let res = RegionEntries(entries.clone());
        self.metrix.send_time(METRIC_FETCH, timer).await;
        Ok(res)
    }
}

#[request(Actions::FetchRegions)]
#[derive(Debug, Default, Parse)]
pub struct FetchRegionReq;
