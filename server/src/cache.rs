mod market;
mod sde;

pub use self::market::*;
pub use self::sde::*;

use crate::metrics::Metrics;

use async_std::sync::Mutex;
use std::collections::HashMap;

pub struct Cache {
    metrics: Option<Metrics>,

    data: Mutex<HashMap<u32, CacheEntry>>,
    regions: Mutex<Vec<RegionCacheEntry>>,
    names: Mutex<Vec<NameCacheEntry>>,
    sde_checksum: Mutex<Vec<u8>>,
}

impl Cache {
    pub fn new(metrics: Option<Metrics>) -> Self {
        Self {
            metrics,

            data: Mutex::new(HashMap::new()),
            names: Mutex::new(Vec::new()),
            regions: Mutex::new(Vec::new()),
            sde_checksum: Mutex::new(Vec::with_capacity(32)),
        }
    }

    pub async fn refresh(&self) {
        log::debug!("Refreshing sde cache");

        let checksum = { self.sde_checksum.lock().await.clone() };
        if let Some((results, checksum)) = SdeCache::refresh(checksum, self.metrics.clone()).await {
            *self.sde_checksum.lock().await = checksum;

            dbg!(results.len());
            for result in results {
                match result {
                    SdeCacheResult::ItemInfos(x) => {
                        for type_ in x {
                            self.data
                                .lock()
                                .await
                                .entry(type_.id)
                                .and_modify(|x| x.info = type_.clone())
                                .or_insert(CacheEntry::new(type_.clone()));
                        }
                    },
                    SdeCacheResult::Regions(x) => {
                        *self.regions.lock().await = x;
                    },
                    SdeCacheResult::UniqueNames(x) => {
                        *self.names.lock().await = x;
                    }
                }
            }
        }
        log::debug!("Done refreshing sde cache");

        log::debug!("Refreshing market cache");
        let regions = self.regions.lock().await.clone();
        for (k, v) in MarketCache::refresh(regions, self.metrics.clone()).await {
            self.data.lock().await.entry(k).and_modify(|x| x.market = v);
        }
        log::debug!("Done refreshing market cache");
    }
}

impl Cache {
    pub async fn fetch_item(&self, id: u32) -> Option<ItemCacheEntry> {
        self.data
            .lock()
            .await
            .clone()
            .into_iter()
            .find(|(k, _)| *k == id)
            .map(|(_, x)| x.info)
    }

    pub async fn fetch_items(&self) -> Vec<ItemCacheEntry> {
        self.data
            .lock()
            .await
            .clone()
            .into_iter()
            .map(|(_, x)| x.info)
            .collect()
    }
}

impl Cache {
    pub async fn fetch_regions(&self) -> Vec<RegionCacheEntry> {
        self.regions
            .lock()
            .await
            .clone()
    }
}

#[derive(Clone, Debug)]
pub struct CacheEntry {
    info: ItemCacheEntry,
    market: Vec<MarketCacheEntry>,
}

impl CacheEntry {
    pub fn new(sde_entry: ItemCacheEntry) -> Self {
        Self {
            info: sde_entry,
            market: Vec::new(),
        }
    }
}
