use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{CachemError, Fetch, FileUtils, Insert, Lookup, Parse, Save, request};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct MarketOrderInfoCache(RwLock<HashMap<u64, MarketOrderInfoEntry>>);

impl MarketOrderInfoCache {
    pub const CAPACITY: usize = 1_000_000;

    const FILE_NAME: &'static str = "./db/storage/market_order_infos.carina";

    pub async fn new() -> Result<Self, CachemError> {
        Ok(Self(RwLock::new(Self::load().await?)))
    }

    async fn load() -> Result<HashMap<u64, MarketOrderInfoEntry>, CachemError> {
        let entries = FileUtils::open::<MarketOrderInfoEntry>(Self::FILE_NAME).await?;
        let mut data = HashMap::with_capacity(entries.len() as usize);
        for entry in entries {
            data.insert(entry.order_id, entry);
        }
        Ok(data)
    }
}

#[async_trait]
impl Fetch<FetchMarketOrderInfoEntryById> for MarketOrderInfoCache {
    type Error = EmptyResponse;
    type Response = MarketOrderInfoEntry;

    async fn fetch(&self, input: FetchMarketOrderInfoEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Lookup<LookupMarketOrderInfoEntries> for MarketOrderInfoCache {
    type Error = EmptyResponse;
    type Response = LookupMarketOrderInfoEntriesResult;

    async fn lookup(&self, input: LookupMarketOrderInfoEntries) -> Result<Self::Response, Self::Error> {
        let start = std::time::Instant::now();
        let data = { self.0.read().await.clone() };
        let mut new_entries = Vec::with_capacity(1_000);

        for order_id in input.0 {
            if !data.contains_key(&order_id) {
                new_entries.push(order_id);
            }
        }
        log::info!("Lookup took {}ms", start.elapsed().as_millis());

        Ok(LookupMarketOrderInfoEntriesResult(new_entries))
    }
}

#[async_trait]
impl Insert<InsertMarketOrderInfoEntries> for MarketOrderInfoCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertMarketOrderInfoEntries) -> Result<Self::Response, Self::Error> {
        let mut old_data = { self.0.read().await.clone() };
        let mut data = input.0;
        let mut changes: usize = 0;

        while let Some(x) = data.pop() {
            old_data
                .entry(x.order_id)
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
        self.store().await.unwrap();
        Ok(EmptyResponse::default())
    }
}

#[async_trait]
impl Save for MarketOrderInfoCache {
    async fn store(&self) -> Result<(), CachemError> {
        let mut entries = Vec::with_capacity(self.0.read().await.len());
        for (_, x) in self.0.read().await.iter() {
            entries.push(x.clone());
        }
        FileUtils::save(Self::FILE_NAME, entries).await?;
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

#[request(Actions::Fetch, Caches::MarketOrderInfo)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderInfoEntryById(pub u64);

#[request(Actions::Insert, Caches::MarketOrderInfo)]
#[derive(Debug, Parse)]
pub struct InsertMarketOrderInfoEntries(pub Vec<MarketOrderInfoEntry>);

#[request(Actions::Lookup, Caches::MarketOrderInfo)]
#[derive(Debug, Parse)]
pub struct LookupMarketOrderInfoEntries(pub Vec<u64>);

#[derive(Debug, Parse)]
pub struct LookupMarketOrderInfoEntriesResult(pub Vec<u64>);
