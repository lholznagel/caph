use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{CachemError, Fetch, FileUtils, Insert, Parse, Save, request};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct BlueprintCache(RwLock<HashMap<u32, BlueprintEntry>>);

impl BlueprintCache {
    pub const CAPACITY: usize = 100_000;

    const FILE_NAME: &'static str = "blueprints.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(RwLock::new(cache)))
    }

    async fn load() -> Result<HashMap<u32, BlueprintEntry>, CachemError> {
        let entries = FileUtils::open::<BlueprintEntry>(Self::FILE_NAME).await?;
        let mut data = HashMap::with_capacity(entries.len() as usize);
        for entry in entries {
            data.insert(entry.item_id, entry);
        }
        Ok(data)
    }
}

#[async_trait]
impl Fetch<FetchBlueprintEntryById> for BlueprintCache {
    type Error = EmptyResponse;
    type Response = BlueprintEntry;

    async fn fetch(&self, input: FetchBlueprintEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Insert<InsertBlueprintEntries> for BlueprintCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertBlueprintEntries) -> Result<Self::Response, Self::Error> {
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
impl Save for BlueprintCache {
    async fn store(&self) -> Result<(), CachemError> {
        let mut entries = Vec::with_capacity(self.0.read().await.len());
        for (_, x) in self.0.read().await.iter() {
            entries.push(x.clone());
        }
        FileUtils::save(Self::FILE_NAME, entries).await?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Parse)]
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

#[derive(Clone, Debug, PartialEq, Parse)]
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

#[request(Actions::Fetch, Caches::Blueprint)]
#[derive(Parse)]
pub struct FetchBlueprintEntryById(pub u32);

#[request(Actions::Insert, Caches::Blueprint)]
#[derive(Parse)]
pub struct InsertBlueprintEntries(pub Vec<BlueprintEntry>);
