use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{Fetch, Insert, Parse, request};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct IdNameCache(RwLock<HashMap<u32, IdNameEntry>>);

impl IdNameCache {
    pub const CAPACITY: usize = 425_000;
}

#[async_trait]
impl Fetch<FetchNameEntryById> for IdNameCache {
    type Error = EmptyResponse;
    type Response = IdNameEntry;

    async fn fetch(&self, input: FetchNameEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Insert<InsertIdNameEntries> for IdNameCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertIdNameEntries) -> Result<Self::Response, Self::Error> {
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

#[derive(Clone, Debug, PartialEq, Parse)]
pub struct IdNameEntry {
    pub item_id: u32,
    pub name:    String,
}

impl IdNameEntry {
    pub fn new(
        item_id: u32,
        name: String
    ) -> Self {
        Self {
            item_id,
            name,
        }
    }
}

#[request(Actions::Fetch, Caches::IdName)]
#[derive(Debug, Parse)]
pub struct FetchNameEntryById(pub u32);

#[request(Actions::Insert, Caches::IdName)]
#[derive(Debug, Parse)]
pub struct InsertIdNameEntries(pub Vec<IdNameEntry>);
