use crate::{Action, Caches, FileUtils, parser_request};

use async_trait::async_trait;
use cachem_utils::{CachemError, Parse, Save, ProtocolRequest};
use std::collections::HashMap;
use std::io::Cursor;
use tokio::sync::Mutex;

pub struct MarketOrderInfoCache(Mutex<HashMap<u64, MarketOrderInfoEntry>>);

impl MarketOrderInfoCache {
    pub const CAPACITY: usize = 1_000_000;

    const FILE_NAME: &'static str = "market_order.carina";

    pub async fn new() -> Result<Self, CachemError> {
        let cache = Self::load().await?;
        Ok(Self(Mutex::new(cache)))
    }

    pub async fn lookup(&self, lookup_ids: Vec<u64>) -> Result<LookupMarketOrderInfoEntriesResult, CachemError> {
        let start = std::time::Instant::now();
        let data = { self.0.lock().await.clone() };
        let mut new_entries = Vec::with_capacity(1_000);

        for order_id in lookup_ids {
            if !data.contains_key(&order_id) {
                new_entries.push(order_id);
            }
        }
        log::info!("Lookup took {}ms", start.elapsed().as_millis());

        Ok(LookupMarketOrderInfoEntriesResult(new_entries))
    }

    pub async fn fetch_by_id(&self, order_id: u64) -> Option<MarketOrderInfoEntry> {
        if let Some(x) = self.0.lock().await.get(&order_id) {
            Some(*x)
        } else {
            None
        }
    }

    pub async fn insert(&self, data: Vec<MarketOrderInfoEntry>) -> Result<(), CachemError> {
        let mut old_data = { self.0.lock().await.clone() };
        let mut data = data;
        let mut changes = 0usize;

        loop {
            if let Some(x) = data.pop() {
                old_data
                    .entry(x.order_id)
                    .or_insert({
                        changes += 1;
                        x
                    });
            } else {
                break;
            }
        }

        // there where some changes, so we apply those to the main structure
        if changes > 0 {
            *self.0.lock().await = old_data;
        }
        Ok(())
    }

    async fn load() -> Result<HashMap<u64, MarketOrderInfoEntry>, CachemError> {
        if let Some(mut buf) = FileUtils::open(Self::FILE_NAME).await? {
            let length = u32::read(&mut buf).await?;
            let mut data = HashMap::with_capacity(length as usize);

            for _ in 0..length {
                let entry = MarketOrderInfoEntry::read(&mut buf).await?;
                data.insert(entry.order_id, entry);
            }
            Ok(data)
        } else {
            Ok(HashMap::with_capacity(Self::CAPACITY))
        }
    }
}

#[async_trait]
impl Save for MarketOrderInfoCache {
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
pub struct MarketOrderInfoEntry {
    pub order_id:     u64,
    pub issued:       u64,
    pub volume_total: u32,
    pub system_id:    u32,
    pub item_id:      u32,
    pub location_id:  u64,
    pub price:        f32,
    pub is_buy_order: bool,
}

impl MarketOrderInfoEntry {
    pub fn new(
        order_id: u64,
        issued: u64,
        volume_total: u32,
        system_id: u32,
        item_id: u32,
        location_id: u64,
        price: f32,
        is_buy_order: bool,
    ) -> Self {
        Self {
            order_id,
            issued,
            volume_total,
            system_id,
            item_id,
            location_id,
            price,
            is_buy_order,
        }
    }
}

#[derive(Parse)]
pub struct FetchMarketOrderInfoEntryById(pub u64);
parser_request!(Action::Fetch, Caches::MarketOrderInfo, FetchMarketOrderInfoEntryById);

#[derive(Parse)]
pub struct InsertMarketOrderInfoEntries(pub Vec<MarketOrderInfoEntry>);
parser_request!(Action::Insert, Caches::MarketOrderInfo, InsertMarketOrderInfoEntries);

#[derive(Parse)]
pub struct LookupMarketOrderInfoEntries(pub Vec<u64>);
parser_request!(Action::Lookup, Caches::MarketOrderInfo, LookupMarketOrderInfoEntries);

#[derive(Parse)]
pub struct LookupMarketOrderInfoEntriesResult(pub Vec<u64>);
