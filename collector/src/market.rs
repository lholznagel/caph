use crate::error::CollectorError;
use crate::time::previous_30_minute;

use cachem::{ConnectionPool, EmptyResponse, Protocol};
use caph_eve_online_api::{EveClient, MarketOrder};
use caph_db::*;
use chrono::Utc;
use futures::stream::{FuturesUnordered, StreamExt};
use std::time::Instant;

pub struct Market {
    pool: ConnectionPool,
}

impl Market {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn task(&mut self) -> Result<(), CollectorError> {
        let timestamp = previous_30_minute(Utc::now().timestamp() as u64) * 1_000;
        let client = EveClient::default();

        let mut requests = FuturesUnordered::new();
        let mut conn = self.pool.acquire().await?;
        let regions = Protocol::request::<_, RegionEntries>(
            &mut conn,
            FetchRegionReq::default()
        )
        .await
        .unwrap();

        for region in regions.0 {
            requests.push(client.fetch_market_orders(region.into()));
        }

        let mut results = Vec::new();
        while let Some(return_val) = requests.next().await {
            match return_val {
                Ok(result) => {
                    results.extend(result.unwrap_or_default());
                }
                // if you dont handle errors, there are non
                _ => (),
            }
        }

        self.market_info(results, timestamp).await?;

        Ok(())
    }

    async fn market_info(
        &mut self,
        entries: Vec<MarketOrder>,
        timestamp: u64
    ) -> Result<(), CollectorError> {
        log::info!("Starting market import");
        let start = Instant::now();

        //let mut lookup_ids = Vec::with_capacity(MarketOrderInfoCache::CAPACITY);
        let mut ids = Vec::with_capacity(MarketOrderInfoCache::CAPACITY);
        for entry in entries.iter() {
            // lookup_ids.push(entry.order_id);
            ids.push(entry.order_id);
        }
        //log::info!("Looking up {} ids", lookup_ids.len());

        //let mut conn = self.pool.acquire().await.unwrap();
        /*let ids = Protocol::request::<_, LookupMarketOrderInfoEntries>(
            &mut conn,
            LookupMarketOrderInfoEntries(lookup_ids)
        )
        .await
        .unwrap().0;
        log::info!("Lookup took {}ms", start.elapsed().as_millis());*/

        log::info!("Starting import of {}", ids.len());
        // TODO: Add metrics
        let mut market_order_infos = Vec::with_capacity(MarketOrderCache::CAPACITY);
        for entry in entries.iter() {
            if !ids.contains(&entry.order_id.into()) {
                continue;
            }

            let market_order_info = MarketOrderInfoEntry::new(
                timestamp,
                entry.order_id,
                entry.location_id,
                entry.system_id,
                entry.type_id,
                entry.volume_total,
                entry.price,
                entry.is_buy_order,
            );
            market_order_infos.push(market_order_info);
        }
        log::info!("After prep vec {}ms", start.elapsed().as_millis());

        if market_order_infos.len() > 0 {
            let mut conn = self.pool.acquire().await.unwrap();
            Protocol::request::<_, EmptyResponse>(
                &mut conn,
                InsertMarketOrderInfoReq(market_order_infos)
            )
            .await
            .unwrap();
            log::info!("Send all market infos to db {}ms", start.elapsed().as_millis());
        } else {
            log::warn!("Market orders was empty");
        }

        let start = Instant::now();
        let mut market_orders = Vec::with_capacity(MarketOrderCache::CAPACITY);
        for entry in entries {
            let market_order = MarketOrderEntry::new(
                entry.order_id,
                timestamp,
                entry.volume_remain,
                entry.type_id,
            );
            market_orders.push(market_order);
        }
        log::info!("Preparing vec took {}ms", start.elapsed().as_millis());

        if market_orders.len() > 0 {
            let mut conn = self.pool.acquire().await.unwrap();
            Protocol::request::<_, EmptyResponse>(
                &mut conn,
                InsertMarketOrderReq(market_orders)
            )
            .await
            .unwrap();
        } else {
            log::warn!("Market orders was empty");
        }

        log::info!("Importing market done. Took {}ms", start.elapsed().as_millis());
        Ok(())
    }
}
