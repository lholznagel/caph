use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, ProtocolParse, Save, ProtocolRequest};
use std::collections::HashMap;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct BlueprintCache(Mutex<HashMap<u32, BlueprintEntry>>);

impl BlueprintCache {
    pub const CAPACITY: usize = 100_000;

    const FILE_NAME: &'static str = "blueprints.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(Mutex::new(cache)))
    }

    pub async fn fetch_by_id(&self, item_id: u32) -> Option<BlueprintEntry> {
        if let Some(x) = self.0.lock().await.get(&item_id) {
            Some(x.clone())
        } else {
            None
        }
    }

    pub async fn insert(&self, data: Vec<BlueprintEntry>) -> Result<(), CachemError> {
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

    async fn load() -> Result<HashMap<u32, BlueprintEntry>, CachemError> {
        if let Some(mut buf) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut buf).await?;
            let mut data = HashMap::with_capacity(length as usize);
            for _ in 0..length {
                let blueprint_entry = BlueprintEntry::read(&mut buf).await?;
                data.insert(blueprint_entry.item_id, blueprint_entry);
            }
            Ok(data)
        } else {
            Ok(HashMap::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for BlueprintCache {
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

#[derive(Clone, Debug, PartialEq, ProtocolParse)]
pub struct BlueprintEntry {
    pub item_id:   u32,
    pub time:      u32,
    pub materials: Vec<Material>,
}

impl BlueprintEntry {
    pub fn new(
        item_id: u32,
        time: u32,
        materials: Vec<Material>,
    ) -> Self {
        Self {
            item_id,
            time,
            materials,
        }
    }
}

#[derive(Clone, Debug, PartialEq, ProtocolParse)]
pub struct Material {
    pub material_id: u32,
    pub quantity:    u32,
    pub is_product:  bool,
}

impl Material {
    pub fn new(
        material_id: u32,
        quantity: u32,
        is_product: bool,
    ) -> Self {
        Self {
            material_id,
            quantity,
            is_product,
        }
    }
}

#[derive(ProtocolParse)]
pub struct FetchBlueprintEntryById(pub u32);
parser_request!(Action::Fetch, Caches::Blueprint, FetchBlueprintEntryById);

#[derive(ProtocolParse)]
pub struct InsertBlueprintEntries(pub Vec<BlueprintEntry>);
parser_request!(Action::Insert, Caches::Blueprint, InsertBlueprintEntries);
