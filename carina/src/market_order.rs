use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, ProtocolParse, Save, ProtocolRequest};
use std::collections::HashMap;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct MarketOrderCache {
    current: Mutex<HashMap<u64, MarketOrderEntry>>,
    history: Mutex<HashMap<u64, Vec<MarketOrderEntry>>>,
}

impl MarketOrderCache {
    pub const CAPACITY: usize = 1_000_000;

    const FILE_NAME: &'static str = "market_orders.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let history = Self::load().await?;
        Ok(Self {
            current: Mutex::new(HashMap::with_capacity(Self::CAPACITY)),
            history: Mutex::new(history),
        })
    }

    pub async fn fetch_by_id(&self, order_id: u64) -> Option<MarketOrderEntry> {
        if let Some(x) = self.current.lock().await.get(&order_id) {
            Some(x.clone())
        } else {
            None
        }
    }

    pub async fn fetch_histry_by_id(&self, order_id: u64) -> Option<Vec<MarketOrderEntry>> {
        if let Some(x) = self.history.lock().await.get(&order_id) {
            Some(x.clone())
        } else {
            None
        }
    }

    pub async fn insert(&self, data: Vec<MarketOrderEntry>) -> Result<(), CachemError> {
        let mut old_data = { self.history.lock().await.clone() };
        let mut data = data;
        let mut changes = 0usize;

        while let Some(x) = data.pop() {
            old_data
                .entry(x.order_id)
                .and_modify(|y| {
                    let last = y.last().unwrap();
                    if last.volume_remain != x.volume_remain {
                        y.push(x);
                        changes += 1;
                    }
                })
                .or_insert({
                    changes += 1;
                    vec![x]
                });
        }

        if changes > 0 {
            *self.history.lock().await = old_data;
        }

        let mut map = HashMap::with_capacity(Self::CAPACITY);
        for x in data {
            map.insert(x.order_id, x);
        }
        *self.current.lock().await = map;
        Ok(())
    }

    async fn load() -> Result<HashMap<u64, Vec<MarketOrderEntry>>, CachemError> {
        if let Some(mut buf) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut buf).await?;
            let mut data = HashMap::with_capacity(length as usize);

            for _ in 0..length {
                let entry_count = u32::read(&mut buf).await?;
                let mut entries = Vec::with_capacity(entry_count as usize);

                for _ in 0..entry_count {
                    let entry = MarketOrderEntry::read(&mut buf).await?;
                    entries.push(entry);
                }
                data.insert(entries[0].order_id, entries);
            }
            Ok(data)
        } else {
            Ok(HashMap::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for MarketOrderCache {
    async fn store(&self) -> Result<(), CachemError> {
        let mut buf = Cursor::new(Vec::new());
        u32::from(self.history.lock().await.len() as u32).write(&mut buf).await?;
        for entries in self.history.lock().await.values() {
            u32::from(entries.len() as u32).write(&mut buf).await?;
            for entry in entries {
                entry.write(&mut buf).await?;
            }
        }
        FileUtils::save(Self::FILE_NAME, buf).await?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, ProtocolParse)]
pub struct MarketOrderEntry {
    pub order_id:      u64,
    pub volume_remain: u32,
    pub timestamp:     u64,
}

impl MarketOrderEntry {
    pub fn new(
        order_id: u64,
        volume_remain: u32,
        timestamp: u64,
    ) -> Self {
        Self {
            order_id,
            volume_remain,
            timestamp,
        }
    }
}

#[derive(ProtocolParse)]
pub struct FetchMarketOrderEntryById(pub u64);
parser_request!(Action::Fetch, Caches::MarketOrder, FetchMarketOrderEntryById);

#[derive(ProtocolParse)]
pub struct InsertMarketOrderEntries(pub Vec<MarketOrderEntry>);
parser_request!(Action::Insert, Caches::MarketOrder, InsertMarketOrderEntries);
