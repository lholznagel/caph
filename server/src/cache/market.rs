use crate::Cache;
use crate::cache::RegionCacheEntry;
use crate::metrics::Metrics;

use eve_online_api::{EveClient, MarketOrder, RegionId};
use futures::stream::{FuturesUnordered, StreamExt};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Instant;

pub struct MarketCache;

impl MarketCache {
    pub async fn refresh(regions: Vec<RegionCacheEntry>, metrics: Option<Metrics>) -> HashMap<u32, Vec<MarketCacheEntry>> {
        let start = Instant::now();

        log::debug!("Fetching market orders");
        let client = EveClient::default();
        let mut requests = FuturesUnordered::new();

        for region in regions {
            requests.push(client.fetch_market_orders(RegionId(region.region_id)));
        }

        let mut orders: HashMap<u32, Vec<MarketCacheEntry>> = HashMap::new();
        while let Some(return_val) = requests.next().await {
            match return_val {
                Ok(result) => {
                    for order in result.unwrap_or_default() {
                        orders
                            .entry(order.type_id.0)
                            .and_modify(|x| x.push(MarketCacheEntry::from(order.clone())))
                            .or_insert(vec![MarketCacheEntry::from(order)]);
                    }
                }
                // we ignore errors
                _ => (),
            }
        }
        log::debug!("Fetched market orders");

        let orders_len = orders.len();
        let request_time = start.elapsed().as_millis();
        if let Some(x) = metrics.as_ref() {
            x.put_market_metrics(orders_len, request_time).await;
        }

        orders
    }
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct MarketCacheEntry {
    pub is_buy_order: bool,
    pub location_id: u64,
    pub price: f32,
    pub system_id: u32,
    pub type_id: u32,
    pub volume_remain: u32,
}

impl From<MarketOrder> for MarketCacheEntry {
    fn from(x: MarketOrder) -> Self {
        Self {
            is_buy_order: x.is_buy_order,
            location_id: x.location_id,
            price: x.price,
            system_id: x.system_id.0,
            type_id: x.type_id.0,
            volume_remain: x.volume_remain,
        }
    }
}

// TODO: Filter by names and groupids, security
#[derive(Clone, Debug, Deserialize)]
pub struct MarketFilter {
    /// Returns only buy orders
    #[serde(rename = "onlyBuyOrders")]
    pub only_buy_orders: Option<bool>,
    /// Returns only sell orders
    #[serde(rename = "onlySellOrders")]
    pub only_sell_orders: Option<bool>,
    #[serde(rename = "locationIds")]
    /// Filters by location id
    pub location_ids: Option<Vec<u64>>,
    /// Filters by system id
    #[serde(rename = "systemIds")]
    pub system_ids: Option<Vec<u32>>,
    /// Filters by type ids
    pub ids: Option<Vec<u32>>,
}

/// All market implementations
impl Cache {
    pub async fn fetch_market(&self, filter: MarketFilter) -> Vec<MarketCacheEntry> {
        self.data
            .lock()
            .await
            .clone()
            .into_iter()
            .filter(|(k, _)| {
                if let Some(x) = filter.ids.clone() {
                    x.contains(k)
                } else {
                    true
                }
            })
            .map(|(_, v)| v.market)
            .fold(Vec::new(), |mut acc, v| {
                acc.extend(v);
                acc
            })
            .into_iter()
            // filters only buy orders
            .filter(|x| {
                match filter.only_buy_orders {
                    Some(_) => x.is_buy_order == true,
                    None => true
                }
            })
            // filters only sell orders
            .filter(|x| {
                match filter.only_sell_orders {
                    Some(_) => x.is_buy_order == false,
                    None => true
                }
            })
            // filters by location id
            .filter(|x| {
                match filter.location_ids.clone() {
                    Some(l) => l.contains(&x.location_id),
                    None => true
                }
            })
            // filters by system id
            .filter(|x| {
                match filter.system_ids.clone() {
                    Some(l) => l.contains(&x.system_id),
                    None => true
                }
            })
            .collect::<Vec<MarketCacheEntry>>()
    }

    pub async fn count_market_entries(&self) -> usize {
        self.data
            .lock()
            .await
            .clone()
            .into_iter()
            .map(|(_, k)| k.market)
            .fold(0usize, |mut acc, _| {
                acc += 1;
                acc
            })
    }
}