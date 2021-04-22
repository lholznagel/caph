use std::time::Instant;

use crate::{Actions, SystemRegionCache, SystemRegionEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::system_region::complete";

#[async_trait]
impl Fetch<FetchSystemRegionReq> for SystemRegionCache {
    type Response = FetchSystemRegionRes;

    async fn fetch(&self, input: FetchSystemRegionReq) -> Self::Response {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = x.clone();
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchSystemRegionRes::Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchSystemRegionRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchSystemRegion)]
#[derive(Debug, Parse)]
pub struct FetchSystemRegionReq(pub u32);

#[derive(Debug, Parse)]
pub enum FetchSystemRegionRes {
    Ok(SystemRegionEntry),
    Err(EmptyMsg),
}
