use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{CachemError, Fetch, FileUtils, Insert, Parse, Save, request};
use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;

type ItemId = u32;

#[derive(Clone, Debug)]
struct BucketEntry {
    timestamp: u64,
    entries: Vec<OrderEntry>,
}

#[derive(Clone, Debug)]
struct OrderEntry {
    volume_remain: u32,
    order_id: u64,
}

#[derive(Default)]
pub struct MarketOrderCache {
    current: RwLock<HashMap<u64, MarketOrderEntry>>,
    history: RwLock<HashMap<ItemId, VecDeque<BucketEntry>>>,
}

impl MarketOrderCache {
    pub const CAPACITY: usize = 1_000_000;

    const FILE_NAME: &'static str = "market_orders.carina";

    pub async fn new() -> Result<Self, CachemError> {
        /*let history = Self::load().await?;
        Ok(Self {
            current: RwLock::new(HashMap::with_capacity(Self::CAPACITY)),
            history: RwLock::new(history),
        })*/
        Ok(Self::default())
    }

    /*async fn load() -> Result<HashMap<u64, Vec<MarketOrderEntry>>, CachemError> {
        let entries = FileUtils::open::<MarketOrderSaveEntry>(Self::FILE_NAME).await?;
        let mut data = HashMap::with_capacity(entries.len() as usize);
        for entry in entries {
            data.insert(entry.order_id, entry.into());
        }
        Ok(data)
    }*/
}

#[async_trait]
impl Fetch<FetchMarketOrderEntryById> for MarketOrderCache {
    type Error = EmptyResponse;
    type Response = MarketOrderEntry;

    async fn fetch(&self, input: FetchMarketOrderEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.current.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Fetch<FetchMarketOrderHistoryEntryById> for MarketOrderCache {
    type Error = EmptyResponse;
    type Response = MarketOrderEntry;

    async fn fetch(&self, input: FetchMarketOrderHistoryEntryById) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.current.read().await.get(&input.0) {
            Ok(x.clone())
        } else {
            Err(EmptyResponse::default())
        }
    }
}

#[async_trait]
impl Insert<InsertMarketOrderEntries> for MarketOrderCache {
    type Error = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertMarketOrderEntries) -> Result<Self::Response, Self::Error> {
        let mut old_data = { self.history.read().await.clone() };
        let mut data = input.0;

        while let Some(x) = data.pop() {
            // Make sure that the new entry is really 30 mintes after the last
            // entry
            if let Some(item_entries) = old_data.get(&x.item_id) {
                if let Some(front) = item_entries.front() {
                    let mut new_ts = front.timestamp + 1800;
                    // if the new timestamp is more than 30 minutes in the
                    // future, insert empty vecs between those
                    while new_ts <= x.timestamp {
                        old_data
                            .entry(x.item_id)
                            .and_modify(|y| y.push_front(BucketEntry {
                                timestamp: new_ts,
                                entries: Vec::new()
                            }));
                        new_ts += 1800;
                    }
                }

                let order_entry = OrderEntry {
                    volume_remain: x.volume_remain,
                    order_id: x.order_id
                };
                old_data
                    .entry(x.item_id)
                    .and_modify(|be| {
                        be
                            .front_mut()
                            .unwrap()
                            .entries
                            .push(order_entry);
                    });
            }
        }
        dbg!(&old_data);
        *self.history.write().await = old_data;

        let mut map = HashMap::with_capacity(Self::CAPACITY);
        for x in data {
            map.insert(x.order_id, x);
        }
        *self.current.write().await = map;
        Ok(EmptyResponse::default())
    }
}

#[async_trait]
impl Save for MarketOrderCache {
    async fn store(&self) -> Result<(), CachemError> {
        /*let mut entries = Vec::with_capacity(self.history.read().await.len());
        for (id, x) in self.history.read().await.iter() {
            entries.push(MarketOrderSaveEntry {
                order_id: id.clone(),
                orders: x.clone(),
            });
        }
        FileUtils::save(Self::FILE_NAME, entries).await?;*/
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct MarketOrderEntry {
    pub order_id:      u64,
    pub timestamp:     u64,
    pub volume_remain: u32,
    pub item_id:       u32,
}

impl MarketOrderEntry {
    pub fn new(
        order_id: u64,
        timestamp: u64,
        volume_remain: u32,
        item_id: u32,
    ) -> Self {
        Self {
            order_id,
            timestamp,
            volume_remain,
            item_id,
        }
    }
}

#[derive(Parse)]
pub struct MarketOrderSaveEntry {
    pub order_id: u64,
    pub orders: Vec<MarketOrderEntry>,
}

impl Into<Vec<MarketOrderEntry>> for MarketOrderSaveEntry {
    fn into(self) -> Vec<MarketOrderEntry> {
        self.orders
    }
}

#[request(Actions::Fetch, Caches::MarketOrder)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderEntryById(pub u64);

#[request(Actions::Fetch, Caches::MarketOrder)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderHistoryEntryById(pub u64);

#[request(Actions::Insert, Caches::MarketOrder)]
#[derive(Debug, Parse)]
pub struct InsertMarketOrderEntries(pub Vec<MarketOrderEntry>);
