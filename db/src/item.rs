use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{Fetch, Insert, Parse, request};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct ItemCache(RwLock<HashMap<u32, ItemEntry>>);

impl ItemCache {
    pub const CAPACITY: usize = 40_000;
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
#[derive(Debug, Parse)]
pub struct FetchItemEntryById(pub u32);

#[request(Actions::Insert, Caches::Item)]
#[derive(Debug, Parse)]
pub struct InsertItemEntries(pub Vec<ItemEntry>);
