use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{CachemError, Fetch, FileUtils, Insert, Parse, Save, request};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct StationCache(RwLock<HashMap<u32, StationEntry>>);

impl StationCache {
    pub const CAPACITY: usize = 6_000;

    const FILE_NAME: &'static str = "stations.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(RwLock::new(cache)))
    }

    async fn load() -> Result<HashMap<u32, StationEntry>, CachemError> {
        let entries = FileUtils::open::<StationEntry>(Self::FILE_NAME).await?;
        let mut data = HashMap::with_capacity(entries.len() as usize);
        for entry in entries {
            data.insert(entry.system_id, entry);
        }
        Ok(data)
    }
}

#[async_trait]
impl Fetch<FetchStationEntryById> for StationCache {
    type Error = EmptyResponse;
    type Response = StationEntry;

    async fn fetch(&self, input: FetchStationEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Insert<InsertStationEntries> for StationCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertStationEntries) -> Result<Self::Response, Self::Error> {
        let mut old_data = { self.0.read().await.clone() };
        let mut data = input.0;
        let mut changes: usize = 0;

        while let Some(x) = data.pop() {
            old_data
                .entry(x.station_id)
                .and_modify(|entry| {
                    if *entry != x {
                        changes += 1;
                        *entry = x.clone();
                    }
                })
                .or_insert({
                    changes += 1;
                    x
                });
        }

        // there where some changes, so we apply those to the main structure
        if changes > 0 {
            *self.0.write().await = old_data;
        }
        Ok(EmptyResponse::default())
    }
}

#[async_trait]
impl Save for StationCache {
    async fn store(&self) -> Result<(), CachemError> {
        let mut entries = Vec::with_capacity(self.0.read().await.len());
        for (_, x) in self.0.read().await.iter() {
            entries.push(x.clone());
        }
        FileUtils::save(Self::FILE_NAME, entries).await?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct StationEntry {
    pub station_id: u32,
    pub region_id:  u32,
    pub system_id:  u32,
    pub security:   f32,
}

impl StationEntry {
    pub fn new(
        station_id: u32,
        region_id:  u32,
        system_id:  u32,
        security:   f32,
    ) -> Self {

        Self {
            station_id,
            region_id,
            system_id,
            security,
        }
    }
}

#[request(Actions::Fetch, Caches::Station)]
#[derive(Parse)]
pub struct FetchStationEntryById(pub u32);

#[request(Actions::Insert, Caches::Station)]
#[derive(Parse)]
pub struct InsertStationEntries(pub Vec<StationEntry>);
