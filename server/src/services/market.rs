use crate::cache::MarketCacheEntry;
use crate::services::{CacheService, ItemService};

use async_std::sync::Arc;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MarketFilter {
    /// Returns only buy orders
    #[serde(rename = "onlyBuyOrders")]
    pub only_buy_orders: Option<bool>,
    /// Returns only sell orders
    #[serde(rename = "onlySellOrders")]
    pub only_sell_orders: Option<bool>,
    /// Filters by location id
    #[serde(rename = "locationIds")]
    pub location_ids: Option<Vec<u64>>,
    /// Filters by system id
    #[serde(rename = "systemIds")]
    pub system_ids: Option<Vec<u32>>,
    /// Filters by type ids
    pub ids: Option<Vec<u32>>,
    /// Resoves the item names to id
    pub names: Option<Vec<String>>
}

#[derive(Clone)]
pub struct MarketService {
    cache: Arc<CacheService>,
    item_service: ItemService
}

impl MarketService {
    pub fn new(cache: Arc<CacheService>, item_service: ItemService) -> Self {
        Self { cache, item_service }
    }

    pub async fn all(&self, filter: MarketFilter) -> Vec<MarketCacheEntry> {
        let resolved_ids = if let Some(x) = filter.names.clone() {
            Some(
                self
                    .item_service
                    .bulk_search(true, x)
                    .await
                    .into_iter()
                    .map(|(_, x)| x[0].clone())
                    .map(|x| x.id)
                    .collect::<Vec<u32>>()
                )
        } else {
            None
        };

        self.cache
            .fetch_markets()
            .await
            .into_iter()
            .filter(|x| {
                if let Some(ids) = filter.ids.clone() {
                    ids.contains(&x.type_id)
                } else if let Some(ids) = resolved_ids.clone() {
                    ids.contains(&x.type_id)
                } else {
                    true
                }
            })
            // filters only buy orders
            .filter(|x| match filter.only_buy_orders {
                Some(_) => x.is_buy_order == true,
                None => true,
            })
            // filters only sell orders
            .filter(|x| match filter.only_sell_orders {
                Some(_) => x.is_buy_order == false,
                None => true,
            })
            // filters by location id
            .filter(|x| match filter.location_ids.clone() {
                Some(l) => l.contains(&x.location_id),
                None => true,
            })
            // filters by system id
            .filter(|x| match filter.system_ids.clone() {
                Some(l) => l.contains(&x.system_id),
                None => true,
            })
            .collect::<Vec<MarketCacheEntry>>()
    }

    pub async fn count(&self) -> usize {
        self.cache
            .fetch_markets()
            .await
            .len()
    }
}
