use crate::error::CollectorError;

use cachem_utils::{ConnectionPool, Protocol};
use caph_eve_online_api::{EveClient, MarketOrder};
use carina::*;
use chrono::NaiveDateTime;
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

    pub async fn background(&mut self) -> Result<(), CollectorError> {
        let client = EveClient::default();

        log::info!("Requesting all regions.");
        let mut requests = FuturesUnordered::new();
        let mut conn = self.pool.acquire().await?;
        let regions = Protocol::request::<_, RegionEntries>(
            &mut conn,
            FetchRegionEntries::default()
        )
        .await
        .unwrap();
        log::info!("After fetch");
        self.pool.release(conn).await;
        log::info!("There are {} regions", regions.0.len());

        log::info!("Requesting market_infos");
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
        log::info!("Done requesting market infos");

        self.market_info(results).await?;

        Ok(())
    }

    async fn market_info(
        &mut self,
        entries: Vec<MarketOrder>,
    ) -> Result<(), CollectorError> {
        log::info!("Starting market import");
        let start = Instant::now();

        let mut lookup_ids = Vec::with_capacity(MarketOrderInfoCache::CAPACITY);
        for entry in entries.iter() {
            lookup_ids.push(entry.order_id);
        }
        log::info!("Looking up {} ids", lookup_ids.len());

        let mut conn = self.pool.acquire().await.unwrap();
        let ids = Protocol::request::<_, LookupMarketOrderInfoEntries>(
            &mut conn,
            LookupMarketOrderInfoEntries(lookup_ids)
        )
        .await
        .unwrap().0;
        log::info!("Lookup took {}ms", start.elapsed().as_millis());

        log::info!("Starting import of {}", ids.len());
        // FIXME: this process takes ages, about 190_000ms for about 1_000_000 entries
        // TODO: Add metrics
        let mut market_order_infos = Vec::with_capacity(MarketOrderCache::CAPACITY);
        for entry in entries.iter() {
            if !ids.contains(&entry.order_id.into()) {
                continue;
            }

            let date = NaiveDateTime::parse_from_str(
                &entry.issued,
                "%Y-%m-%dT%H:%M:%SZ"
            ).unwrap();

            let market_order_info = MarketOrderInfoEntry::new(
                entry.order_id,
                date.timestamp_millis() as u64,
                entry.volume_total,
                entry.system_id,
                entry.type_id,
                entry.location_id,
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
                InsertMarketOrderInfoEntries(market_order_infos)
            )
            .await
            .unwrap();
            self.pool.release(conn).await;
            log::info!("Send all market infos to db {}ms", start.elapsed().as_millis());
        } else {
            log::warn!("Market orders was empty");
        }

        let start = Instant::now();
        let mut market_orders = Vec::with_capacity(MarketOrderCache::CAPACITY);
        for entry in entries {
            let date = NaiveDateTime::parse_from_str(
                &entry.issued,
                "%Y-%m-%dT%H:%M:%SZ"
            ).unwrap();

            let market_order = MarketOrderEntry::new(
                entry.order_id,
                entry.volume_remain,
                date.timestamp_millis() as u64
            );
            market_orders.push(market_order);
        }
        log::info!("Preparing vec took {}ms", start.elapsed().as_millis());

        if market_orders.len() > 0 {
            let mut conn = self.pool.acquire().await.unwrap();
            Protocol::request::<_, EmptyResponse>(
                &mut conn,
                InsertMarketOrderEntries(market_orders)
            )
            .await
            .unwrap();
            self.pool.release(conn).await;
        } else {
            log::warn!("Market orders was empty");
        }

        log::info!("Importing market done. Took {}ms", start.elapsed().as_millis());
        Ok(())
    }
}