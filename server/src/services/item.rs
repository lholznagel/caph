use crate::cache::ItemCacheEntry;
use crate::services::CacheService;

use async_std::sync::Arc;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ItemService {
    cache: Arc<CacheService>
}

impl ItemService {
    pub fn new(cache: Arc<CacheService>) -> Self {
        Self {
            cache
        }
    }

    pub async fn all(&self) -> Vec<ItemCacheEntry> {
        self.cache.fetch_items().await
    }

    pub async fn by_id(&self, id: u32) -> Option<ItemCacheEntry> {
        self.cache
            .fetch_items()
            .await
            .into_iter()
            .find(|x| x.id == id)
    }

    /// If a id does not exist, it will silently by ignored
    pub async fn bulk_item_by_id(&self, ids: Vec<u32>) -> Vec<ItemCacheEntry> {
        self.all()
            .await
            .into_iter()
            .filter(|x| ids.contains(&x.id))
            .collect::<Vec<ItemCacheEntry>>()
    }

    pub async fn search(&self, exact: bool, name: &str) -> Vec<ItemCacheEntry> {
        self.all()
            .await
            .into_iter()
            .filter(|x| {
                if exact {
                    x.name.to_lowercase() == name.to_lowercase()
                } else {
                    x.name.to_lowercase().contains(&name.to_lowercase())
                }
            })
            .collect::<Vec<ItemCacheEntry>>()
    }

    pub async fn bulk_search(&self, exact: bool, names: Vec<String>) -> HashMap<String, Vec<ItemCacheEntry>> {
        let mut results = HashMap::with_capacity(names.len());

        for name in names {
            let search_result = self.search(exact, &name).await;
            results.insert(name, search_result);
        }

        results
    }
}