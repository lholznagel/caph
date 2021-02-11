use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{Fetch, Insert, Parse, request};
use std::collections::HashSet;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct RegionCache(RwLock<HashSet<u32>>);

impl RegionCache {
    pub const CAPACITY: usize = 50;
}

#[async_trait]
impl Fetch<FetchRegionEntries> for RegionCache {
    type Error = EmptyResponse;
    type Response = RegionEntries;

    async fn fetch(&self, _input: FetchRegionEntries) -> Result<Self::Response, Self::Error> {
        let entries = self.0.read().await;
        Ok(RegionEntries(entries.clone()))
    }
}

#[async_trait]
impl Insert<InsertRegionEntries> for RegionCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertRegionEntries) -> Result<Self::Response, Self::Error> {
        let mut new_data = HashSet::with_capacity(input.0.len());
        for x in input.0 {
            new_data.insert(x.region_id.into());
        }

        *self.0.write().await = new_data;
        Ok(EmptyResponse::default())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Parse)]
pub struct RegionEntry {
    pub region_id: u32,
}

impl RegionEntry {
    pub fn new(
        region_id: u32,
    ) -> Self {

        Self {
            region_id
        }
    }
}

#[derive(Debug, Parse)]
pub struct RegionEntries(pub HashSet<u32>);

#[request(Actions::Fetch, Caches::Region)]
#[derive(Debug, Default, Parse)]
pub struct FetchRegionEntries;

#[request(Actions::Insert, Caches::Region)]
#[derive(Debug, Parse)]
pub struct InsertRegionEntries(pub HashSet<RegionEntry>);
