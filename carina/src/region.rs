use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, Parse, Save, ProtocolRequest};
use std::collections::HashSet;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct RegionCache(Mutex<HashSet<u32>>);

impl RegionCache {
    pub const CAPACITY: usize = 50;

    const FILE_NAME: &'static str = "regions.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(Mutex::new(cache)))
    }

    pub async fn fetch_all(&self) -> Option<RegionEntries> {
        let entries = self.0.lock().await;
        Some(RegionEntries(entries.clone()))
    }

    pub async fn insert(&self, data: HashSet<RegionEntry>) -> Result<(), CachemError> {
        let mut new_data = HashSet::with_capacity(data.len());
        for x in data {
            new_data.insert(x.region_id.into());
        }

        *self.0.lock().await = new_data;
        Ok(())
    }

    async fn load() -> Result<HashSet<u32>, CachemError> {
        if let Some(mut db_file) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut db_file).await?;
            let mut data = HashSet::with_capacity(length as usize);
            for _ in 0..length {
                let region_id = u32::read(&mut db_file).await?;
                data.insert(region_id);
            }
            Ok(data)
        } else {
            Ok(HashSet::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for RegionCache {
    async fn store(&self) -> Result<(), CachemError> {
        let mut db_data = Cursor::new(Vec::new());
        u32::from(self.0.lock().await.len() as u32).write(&mut db_data).await?;
        for (region_id) in self.0.lock().await.iter() {
            region_id.write(&mut db_data).await?;
        }
        FileUtils::save(Self::FILE_NAME, db_data).await?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Parse)]
pub struct RegionEntry {
    pub region_id: u32,
}

impl RegionEntry {
    pub fn new(
        region_id: u32,
    ) -> Self {

        Self {
            region_id
        }
    }
}

#[derive(Parse)]
pub struct RegionEntries(pub HashSet<u32>);

#[derive(Default, Parse)]
pub struct FetchRegionEntries;
parser_request!(Action::Fetch, Caches::Region, FetchRegionEntries);

#[derive(Parse)]
pub struct InsertRegionEntries(pub HashSet<RegionEntry>);
parser_request!(Action::Insert, Caches::Region, InsertRegionEntries);
