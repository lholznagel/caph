use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, Parse, Save, ProtocolRequest};
use std::collections::HashMap;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct IdNameCache(Mutex<HashMap<u32, IdNameEntry>>);

impl IdNameCache {
    pub const CAPACITY: usize = 425_000;

    const FILE_NAME: &'static str = "id_names.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(Mutex::new(cache)))
    }

    pub async fn fetch_by_id(&self, item_id: u32) -> Option<IdNameEntry> {
        if let Some(x) = self.0.lock().await.get(&item_id) {
            Some(x.clone())
        } else {
            None
        }
    }

    pub async fn insert(&self, data: Vec<IdNameEntry>) -> Result<(), CachemError> {
        let mut old_data = { self.0.lock().await.clone() };
        let mut data = data;
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
            *self.0.lock().await = old_data;
        }

        Ok(())
    }

    async fn load() -> Result<HashMap<u32, IdNameEntry>, CachemError> {
        if let Some(mut buf) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut buf).await?;
            let mut data = HashMap::with_capacity(length as usize);
            for _ in 0..length {
                let entry = IdNameEntry::read(&mut buf).await?;
                data.insert(entry.item_id, entry);
            }
            Ok(data)
        } else {
            Ok(HashMap::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for IdNameCache {
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

#[derive(Parse)]
pub struct FetchNameEntryById(pub u32);
parser_request!(Action::Fetch, Caches::IdName, FetchNameEntryById);

#[derive(Parse)]
pub struct InsertIdNameEntries(pub Vec<IdNameEntry>);
parser_request!(Action::Insert, Caches::IdName, InsertIdNameEntries);
