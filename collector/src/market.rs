use crate::error::CollectorError;
use crate::time::previous_30_minute;

use cachem::{ConnectionPool, EmptyMsg, Protocol};
use caph_eve_online_api::{EveClient, MarketOrder};
use caph_db::*;
use chrono::Utc;
use futures::stream::{FuturesUnordered, StreamExt};
use metrix_exporter::MetrixSender;
use std::time::Instant;

pub struct Market {
    metrix: MetrixSender,
    pool: ConnectionPool,
}

impl Market {
    const METRIC_MARKET:              &'static str = "market::time::complete";
    const METRIC_FETCHED_REGION:      &'static str = "market::time::fetch_region";
    const METRIC_FETCH_MARKET_DATA:   &'static str = "market::time::market_data::fetch";
    const METRIC_INSERT_MARKET_DATA:  &'static str = "market::time::market_data::insert";
    const METRIC_PREPARE_ORDER_ID:    &'static str = "market::time::prep::order_id";
    const METRIC_PREPARE_MARKET_INFO: &'static str = "market::time::prep::market_info";
    const METRIC_PREPARE_MARKET_DATA: &'static str = "market::time::prep::market_data";
    const METRIC_SEND_MARKET_INFO:    &'static str = "market::time::send::market_info";
    const METRIC_SEND_MARKET_DATA:    &'static str = "market::time::send::market_data";
    const METRIC_COUNT_ORDER_ID:      &'static str = "market::count::order_id";
    const METRIC_COUNT_MARKET_INFO:   &'static str = "market::count::market_info";
    const METRIC_COUNT_MARKET_DATA:   &'static str = "market::count::market_data";

    pub fn new(metrix: MetrixSender, pool: ConnectionPool) -> Self {
        Self {
            metrix,
            pool
        }
    }

    pub async fn task(&mut self) -> Result<(), CollectorError> {
        let start = Instant::now();
        let timer = Instant::now();

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

        self.metrix.send_time(Self::METRIC_FETCHED_REGION, timer).await;
        let timer = Instant::now();

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

        self.metrix.send_time(Self::METRIC_FETCH_MARKET_DATA, timer).await;
        let timer = Instant::now();

        self.market_info(results, timestamp).await?;
        self.metrix.send_time(Self::METRIC_INSERT_MARKET_DATA, timer).await;
        self.metrix.send_time(Self::METRIC_MARKET, start).await;

        Ok(())
    }

    async fn market_info(
        &mut self,
        entries: Vec<MarketOrder>,
        timestamp: u64
    ) -> Result<(), CollectorError> {
        let timer = Instant::now();

        let mut ids = Vec::with_capacity(MarketOrderInfoCache::CAPACITY);
        for entry in entries.iter() {
            ids.push(entry.order_id);
        }
        self.metrix.send_time(Self::METRIC_PREPARE_ORDER_ID, timer).await;
        self.metrix.send_len(Self::METRIC_COUNT_ORDER_ID, ids.len()).await;
        let timer = Instant::now();

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

        self.metrix.send_time(Self::METRIC_PREPARE_MARKET_INFO, timer).await;
        self.metrix.send_len(Self::METRIC_COUNT_MARKET_INFO, market_order_infos.len()).await;
        let timer = Instant::now();

        if market_order_infos.len() > 0 {
            let mut conn = self.pool.acquire().await.unwrap();
            Protocol::request::<_, EmptyMsg>(
                &mut conn,
                InsertMarketOrderInfoReq(market_order_infos)
            )
            .await
            .unwrap();
        } else {
            log::warn!("Market orders was empty");
        }

        self.metrix.send_time(Self::METRIC_SEND_MARKET_INFO, timer).await;
        let timer = Instant::now();

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

        self.metrix.send_time(Self::METRIC_PREPARE_MARKET_DATA, timer).await;
        self.metrix.send_len(Self::METRIC_COUNT_MARKET_DATA, market_orders.len()).await;
        let timer = Instant::now();

        if market_orders.len() > 0 {
            let mut conn = self.pool.acquire().await.unwrap();
            Protocol::request::<_, EmptyMsg>(
                &mut conn,
                InsertMarketOrderReq(market_orders)
            )
            .await
            .unwrap();
        } else {
            log::warn!("Market orders was empty");
        }

        self.metrix.send_time(Self::METRIC_SEND_MARKET_DATA, timer).await;
        Ok(())
    }
}
