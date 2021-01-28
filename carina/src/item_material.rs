use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, Parse, Save, ProtocolRequest};
use std::collections::HashMap;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct ItemMaterialCache(Mutex<HashMap<u32, ItemMaterialEntry>>);

impl ItemMaterialCache {
    pub const CAPACITY: usize = 45_000;

    const FILE_NAME: &'static str = "item_materials.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(Mutex::new(cache)))
    }

    pub async fn fetch_by_id(&self, item_id: u32) -> Option<ItemMaterialEntry> {
        if let Some(x) = self.0.lock().await.get(&item_id) {
            Some(*x)
        } else {
            None
        }
    }

    pub async fn insert(&self, data: Vec<ItemMaterialEntry>) -> Result<(), CachemError> {
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

    async fn load() -> Result<HashMap<u32, ItemMaterialEntry>, CachemError> {
        if let Some(mut buf) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut buf).await?;
            let mut data = HashMap::with_capacity(length as usize);
            for _ in 0..length {
                let entry = ItemMaterialEntry::read(&mut buf).await?;
                data.insert(entry.item_id, entry);
            }
            Ok(data)
        } else {
            Ok(HashMap::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for ItemMaterialCache {
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

#[derive(Parse)]
pub struct FetchItemMaterialEntryById(pub u32);
parser_request!(Action::Fetch, Caches::ItemMaterial, FetchItemMaterialEntryById);

#[derive(Parse)]
pub struct InsertItemMaterialEntries(pub Vec<ItemMaterialEntry>);
parser_request!(Action::Insert, Caches::ItemMaterial, InsertItemMaterialEntries);
