use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, ProtocolParse, Save, ProtocolRequest};
use std::collections::HashMap;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct StationCache(Mutex<HashMap<u32, StationEntry>>);

impl StationCache {
    pub const CAPACITY: usize = 6_000;

    const FILE_NAME: &'static str = "stations.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(Mutex::new(cache)))
    }

    pub async fn fetch_by_id(&self, station_id: u32) -> Option<StationEntry> {
        if let Some(x) = self.0.lock().await.get(&station_id) {
            Some(*x)
        } else {
            None
        }
    }

    pub async fn insert(&self, data: Vec<StationEntry>) -> Result<(), CachemError> {
        let mut old_data = { self.0.lock().await.clone() };
        let mut data = data;
        let mut changes = 0usize;

        while let Some(x) = data.pop() {
            old_data
                .entry(x.station_id)
                .and_modify(|entry| {
                    if *entry != x {
                        changes += 1;
                        *entry = x;
                    }
                })
                .or_insert({
                    changes += 1;
                    x
                });
        }

        if changes > 0 {
            *self.0.lock().await = old_data;
        }
        Ok(())
    }

    async fn load() -> Result<HashMap<u32, StationEntry>, CachemError> {
        if let Some(mut buf) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut buf).await?;
            let mut data = HashMap::with_capacity(length as usize);
            for _ in 0..length {
                let entry = StationEntry::read(&mut buf).await?;
                data.insert(entry.station_id, entry);
            }
            Ok(data)
        } else {
            Ok(HashMap::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for StationCache {
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

#[derive(Copy, Clone, Debug, PartialEq, ProtocolParse)]
pub struct StationEntry {
    pub station_id: u32,
    pub region_id:  u32,
    pub system_id:  u32,
    pub security:   f32,
}

impl StationEntry {
    pub fn new(
        station_id: u32,
        region_id:  u32,
        system_id:  u32,
        security:   f32,
    ) -> Self {

        Self {
            station_id,
            region_id,
            system_id,
            security,
        }
    }
}

#[derive(ProtocolParse)]
pub struct FetchStationEntryById(pub u32);
parser_request!(Action::Fetch, Caches::Station, FetchStationEntryById);

#[derive(ProtocolParse)]
pub struct InsertStationEntries(pub Vec<StationEntry>);
parser_request!(Action::Insert, Caches::Station, InsertStationEntries);
