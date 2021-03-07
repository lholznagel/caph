use crate::{Actions, RegionCache, RegionEntry};

use async_trait::*;
use cachem::{EmptyMsg, Insert, Parse, Storage, request};
use std::collections::HashSet;
use std::time::Instant;

const METRIC_INSERT:         &'static str = "insert::region::complete";
const METRIC_INSERT_ENTRIES: &'static str = "insert::region::entries";

#[async_trait]
impl Insert<InsertRegionReq> for RegionCache {
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertRegionReq) -> Self::Response {
        let timer = Instant::now();
        let mut map = HashSet::with_capacity(input.0.len());
        for x in input.0 {
            map.insert(x.region_id.into());
        }

        self.metrix.send_len(METRIC_INSERT_ENTRIES, map.len()).await;
        *self.cache.write().await = map;
        self.save_to_file().await.unwrap();

        self.metrix.send_time(METRIC_INSERT, timer).await;
        EmptyMsg::default()
    }
}


#[request(Actions::InsertRegions)]
#[derive(Debug, Parse)]
pub struct InsertRegionReq(pub HashSet<RegionEntry>);
