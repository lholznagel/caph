use crate::{Actions, RegionCache, RegionEntries};

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchRegionReq> for RegionCache {
    type Error    = EmptyResponse;
    type Response = RegionEntries;

    async fn fetch(&self, _input: FetchRegionReq) -> Result<Self::Response, Self::Error> {
        let entries = self.0.read().await;
        Ok(RegionEntries(entries.clone()))
    }
}

#[request(Actions::FetchRegions)]
#[derive(Debug, Default, Parse)]
pub struct FetchRegionReq;
