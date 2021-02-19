use crate::{Actions, RegionCache, RegionEntry};

use async_trait::*;
use cachem::{EmptyResponse, Insert, Parse, Storage, request};
use std::collections::HashSet;

#[async_trait]
impl Insert<InsertRegionReq> for RegionCache {
    type Error    = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertRegionReq) -> Result<Self::Response, Self::Error> {
        let mut new_data = HashSet::with_capacity(input.0.len());
        for x in input.0 {
            new_data.insert(x.region_id.into());
        }

        *self.0.write().await = new_data;
        self.save_to_file().await.unwrap();
        Ok(EmptyResponse::default())
    }
}


#[request(Actions::InsertRegions)]
#[derive(Debug, Parse)]
pub struct InsertRegionReq(pub HashSet<RegionEntry>);
