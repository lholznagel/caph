use crate::cache::*;

use async_std::sync::Mutex;

#[deprecated = "Use DB"]
pub struct CacheService {
    pub blueprints: Mutex<Vec<BlueprintCacheEntry>>,
    pub schematics: Mutex<Vec<SchematicCacheEntry>>,
    pub sde_checksum: Mutex<Vec<u8>>,
}

impl CacheService {
    #[deprecated = "Use DB"]
    pub fn new() -> Self {
        Self {
            blueprints: Mutex::new(Vec::new()),
            schematics: Mutex::new(Vec::new()),
            sde_checksum: Mutex::new(Vec::with_capacity(32)),
        }
    }

    #[deprecated = "Use DB"]
    pub async fn refresh(&self) {
        log::debug!("Refreshing sde cache");

        let checksum = { self.sde_checksum.lock().await.clone() };
        if let Some((results, checksum)) = SdeCache::refresh(checksum).await {
            *self.sde_checksum.lock().await = checksum;

            for result in results {
                match result {
                    SdeCacheResult::Blueprints(x) => {
                        *self.blueprints.lock().await = x;
                    }
                    SdeCacheResult::Schematics(x) => {
                        *self.schematics.lock().await = x;
                    }
                }
            }
        }
        log::debug!("Done refreshing sde cache");
    }
}

impl CacheService {
    #[deprecated = "Use DB"]
    pub async fn fetch_blueprints(&self) -> Vec<BlueprintCacheEntry> {
        self.blueprints.lock().await.clone()
    }

    #[deprecated = "Use DB"]
    pub async fn fetch_schematics(&self) -> Vec<SchematicCacheEntry> {
        self.schematics.lock().await.clone()
    }
}