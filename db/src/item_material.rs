use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{Fetch, Insert, Parse, request};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct ItemMaterialCache(RwLock<HashMap<u32, ItemMaterialEntry>>);

impl ItemMaterialCache {
    pub const CAPACITY: usize = 45_000;
}

#[async_trait]
impl Fetch<FetchItemMaterialEntryById> for ItemMaterialCache {
    type Error = EmptyResponse;
    type Response = ItemMaterialEntry;

    async fn fetch(&self, input: FetchItemMaterialEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Insert<InsertItemMaterialEntries> for ItemMaterialCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertItemMaterialEntries) -> Result<Self::Response, Self::Error> {
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

#[derive(Clone, Copy, Debug, PartialEq, Parse)]
pub struct ItemMaterialEntry {
    pub item_id:     u32,
    pub material_id: u32,
    pub quantity:    u32,
}

impl ItemMaterialEntry {
    pub fn new(
        item_id: u32,
        material_id: u32,
        quantity: u32,
    ) -> Self {
        Self {
            item_id,
            material_id,
            quantity,
        }
    }
}

#[request(Actions::Fetch, Caches::ItemMaterial)]
#[derive(Debug, Parse)]
pub struct FetchItemMaterialEntryById(pub u32);

#[request(Actions::Insert, Caches::ItemMaterial)]
#[derive(Debug, Parse)]
pub struct InsertItemMaterialEntries(pub Vec<ItemMaterialEntry>);
