use crate::cache::RegionCacheEntry;
use crate::services::CacheService;

use async_std::sync::Arc;

#[derive(Clone)]
pub struct RegionService {
    cache: Arc<CacheService>,
}

impl RegionService {
    pub fn new(cache: Arc<CacheService>) -> Self {
        Self { cache }
    }

    pub async fn all(&self) -> Vec<RegionCacheEntry> {
        self.cache
            .fetch_regions()
            .await
    }
}
