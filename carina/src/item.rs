use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, Parse, Save, ProtocolRequest};
use std::collections::HashMap;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct ItemCache(Mutex<HashMap<u32, ItemEntry>>);

impl ItemCache {
    pub const CAPACITY: usize = 40_000;

    const FILE_NAME: &'static str = "items.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(Mutex::new(cache)))
    }

    pub async fn fetch_by_id(&self, item_id: u32) -> Option<ItemEntry> {
        if let Some(x) = self.0.lock().await.get(&item_id) {
            Some(x.clone())
        } else {
            None
        }
    }

    pub async fn insert(&self, data: Vec<ItemEntry>) -> Result<(), CachemError> {
        let mut old_data = { self.0.lock().await.clone() };
        let mut data = data;
        let mut changes = 0usize;

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
            *self.0.lock().await = old_data;
        }
        Ok(())
    }

    async fn load() -> Result<HashMap<u32, ItemEntry>, CachemError> {
        if let Some(mut buf) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut buf).await?;
            let mut data = HashMap::with_capacity(length as usize);
            for _ in 0..length {
                let entry = ItemEntry::read(&mut buf).await?;
                data.insert(entry.item_id, entry);
            }
            Ok(data)
        } else {
            Ok(HashMap::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for ItemCache {
    async fn store(&self) -> Result<(), CachemError> {
        let mut buf = Cursor::new(Vec::new());
        u32::from(self.0.lock().await.len() as u32).write(&mut buf).await?;
        for entries in self.0.lock().await.values() {
            entries.write(&mut buf).await?;
        }
        FileUtils::save(Self::FILE_NAME, buf).await?;
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

#[derive(Parse)]
pub struct FetchItemEntryById(pub u32);
parser_request!(Action::Fetch, Caches::Item, FetchItemEntryById);

#[derive(Parse)]
pub struct InsertItemEntries(pub Vec<ItemEntry>);
parser_request!(Action::Insert, Caches::Item, InsertItemEntries);
