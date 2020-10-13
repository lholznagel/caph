use crate::metrics::Metrics;

use async_std::sync::Mutex;
use eve_online_api::{EveClient, MarketOrder};
use futures::stream::{FuturesUnordered, StreamExt};
use serde::Serialize;
use std::collections::HashMap;
use std::time::Instant;

pub struct MarketCache {
    data: Mutex<HashMap<u32, Vec<MarketCacheEntry>>>,
    metrics: Option<Metrics>,
}

impl MarketCache {
    pub fn new(metrics: Option<Metrics>) -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
            metrics,
        }
    }

    pub async fn fetch(&self) -> HashMap<u32, Vec<MarketCacheEntry>> {
        self.data.lock().await.clone()
    }

    pub async fn refresh(&self) {
        let start = Instant::now();

        let orders = MarketCacheCollector::default().collect().await;

        let orders_len = orders.len();
        let request_time = start.elapsed().as_millis();
        if let Some(x) = self.metrics.as_ref() {
            x.put_market_metrics(orders_len, request_time).await;
        }

        let mut data = self.data.lock().await;
        *data = orders;
    }
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct MarketCacheEntry {
    pub is_buy_order: bool,
    pub location_id: u64,
    pub price: f32,
    pub system_id: u32,
    pub volume_remain: u32,
}

impl From<MarketOrder> for MarketCacheEntry {
    fn from(x: MarketOrder) -> Self {
        Self {
            is_buy_order: x.is_buy_order,
            location_id: x.location_id,
            price: x.price,
            system_id: x.system_id.0,
            volume_remain: x.volume_remain,
        }
    }
}

//pub struct MarketCacheCollector(Option<Metrics>);
#[derive(Default)]
struct MarketCacheCollector;

impl MarketCacheCollector {
    pub async fn collect(&self) -> HashMap<u32, Vec<MarketCacheEntry>> {
        log::debug!("Fetching regions");
        let client = EveClient::default();
        let mut requests = FuturesUnordered::new();

        let regions = client.fetch_region_ids().await.unwrap_or_default();
        for region in regions {
            requests.push(client.fetch_market_orders(region));
        }
        log::debug!("Fetched regions");

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

        orders
    }
}
