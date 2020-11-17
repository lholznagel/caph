use crate::cache::*;

use async_std::sync::Mutex;

pub struct CacheService {
    pub blueprints: Mutex<Vec<BlueprintCacheEntry>>,
    pub items: Mutex<Vec<ItemCacheEntry>>,
    pub markets: Mutex<Vec<MarketCacheEntry>>,
    pub names: Mutex<Vec<NameCacheEntry>>,
    pub regions: Mutex<Vec<RegionCacheEntry>>,
    pub schematics: Mutex<Vec<SchematicCacheEntry>>,
    pub sde_checksum: Mutex<Vec<u8>>,
}

impl CacheService {
    pub fn new() -> Self {
        Self {
            blueprints: Mutex::new(Vec::new()),
            items: Mutex::new(Vec::new()),
            markets: Mutex::new(Vec::new()),
            names: Mutex::new(Vec::new()),
            regions: Mutex::new(Vec::new()),
            schematics: Mutex::new(Vec::new()),
            sde_checksum: Mutex::new(Vec::with_capacity(32)),
        }
    }

    pub async fn refresh(&self) {
        log::debug!("Refreshing sde cache");

        let checksum = { self.sde_checksum.lock().await.clone() };
        if let Some((results, checksum)) = SdeCache::refresh(checksum).await {
            *self.sde_checksum.lock().await = checksum;

            for result in results {
                match result {
                    SdeCacheResult::ItemInfos(x) => {
                        *self.items.lock().await = x;
                    }
                    SdeCacheResult::Blueprints(x) => {
                        *self.blueprints.lock().await = x;
                    }
                    SdeCacheResult::Schematics(x) => {
                        *self.schematics.lock().await = x;
                    }
                    SdeCacheResult::Regions(x) => {
                        *self.regions.lock().await = x;
                    }
                    SdeCacheResult::UniqueNames(x) => {
                        *self.names.lock().await = x;
                    }
                }
            }
        }
        log::debug!("Done refreshing sde cache");

        log::debug!("Refreshing market cache");
        let regions = self.regions.lock().await.clone();
        for (_, v) in MarketCache::refresh(regions).await {
            self.markets.lock().await.extend(v);
        }
        log::debug!("Done refreshing market cache");
    }
}

impl CacheService {
    pub async fn fetch_markets(&self) -> Vec<MarketCacheEntry> {
        self.markets.lock().await.clone()
    }

    pub async fn fetch_items(&self) -> Vec<ItemCacheEntry> {
        self.items.lock().await.clone()
    }

    pub async fn fetch_regions(&self) -> Vec<RegionCacheEntry> {
        self.regions.lock().await.clone()
    }

    pub async fn fetch_blueprints(&self) -> Vec<BlueprintCacheEntry> {
        self.blueprints.lock().await.clone()
    }

    pub async fn fetch_schematics(&self) -> Vec<SchematicCacheEntry> {
        self.schematics.lock().await.clone()
    }
}