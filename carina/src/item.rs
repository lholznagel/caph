use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{CachemError, Fetch, FileUtils, Insert, Parse, Save, request};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct ItemCache(RwLock<HashMap<u32, ItemEntry>>);

impl ItemCache {
    pub const CAPACITY: usize = 40_000;

    const FILE_NAME: &'static str = "items.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(RwLock::new(cache)))
    }

    async fn load() -> Result<HashMap<u32, ItemEntry>, CachemError> {
        let entries = FileUtils::open::<ItemEntry>(Self::FILE_NAME).await?;
        let mut data = HashMap::with_capacity(entries.len() as usize);
        for entry in entries {
            data.insert(entry.item_id, entry);
        }
        Ok(data)
    }
}

#[async_trait]
impl Fetch<FetchItemEntryById> for ItemCache {
    type Error = EmptyResponse;
    type Response = ItemEntry;

    async fn fetch(&self, input: FetchItemEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Insert<InsertItemEntries> for ItemCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertItemEntries) -> Result<Self::Response, Self::Error> {
        let mut old_data = { self.0.read().await.clone() };
        let mut data = input.0;
        let mut changes: usize = 0;

        while let Some(x) = data.pop() {
            old_data
                .entry(x.item_id)
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
impl Save for ItemCache {
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
pub struct ItemEntry {
    pub item_id: u32,
    pub volume:  f32,
}

impl ItemEntry {
    pub fn new(
        item_id: u32,
        volume: f32,
    ) -> Self {
        Self {
            item_id,
            volume,
        }
    }
}

#[request(Actions::Fetch, Caches::Item)]
#[derive(Parse)]
pub struct FetchItemEntryById(pub u32);

#[request(Actions::Insert, Caches::Item)]
#[derive(Parse)]
pub struct InsertItemEntries(pub Vec<ItemEntry>);
