use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{CachemError, Fetch, FileUtils, Insert, Parse, Save, request};
use std::collections::HashSet;
use tokio::sync::RwLock;

pub struct RegionCache(RwLock<HashSet<u32>>);

impl RegionCache {
    pub const CAPACITY: usize = 50;

    const FILE_NAME: &'static str = "regions.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(RwLock::new(cache)))
    }

    async fn load() -> Result<HashSet<u32>, CachemError> {
        let entries = FileUtils::open::<u32>(Self::FILE_NAME).await?;
        let mut data = HashSet::with_capacity(entries.len() as usize);
        for entry in entries {
            data.insert(entry);
        }
        Ok(data)
    }
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

#[async_trait]
impl Save for RegionCache {
    async fn store(&self) -> Result<(), CachemError> {
        let mut entries = Vec::with_capacity(self.0.read().await.len());
        for x in self.0.read().await.iter() {
            entries.push(*x);
        }
        FileUtils::save(Self::FILE_NAME, entries).await?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Parse)]
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

#[derive(Parse)]
pub struct RegionEntries(pub HashSet<u32>);

#[request(Actions::Fetch, Caches::Region)]
#[derive(Default, Parse)]
pub struct FetchRegionEntries;

#[request(Actions::Insert, Caches::Region)]
#[derive(Parse)]
pub struct InsertRegionEntries(pub HashSet<RegionEntry>);
