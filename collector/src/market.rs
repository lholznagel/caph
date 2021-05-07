use crate::error::CollectorError;
use crate::time::previous_30_minute;

use cachem::{ConnectionPool, EmptyMsg, Protocol};
use caph_eve_data_wrapper::{EveDataWrapper, MarketOrder};
use caph_db::*;
use chrono::{DateTime, Utc};
use futures::stream::{FuturesUnordered, StreamExt};
use metrix_exporter::MetrixSender;
use std::time::Instant;

pub struct Market {
    eve:    EveDataWrapper,
    metrix: MetrixSender,
    pool: ConnectionPool,
}

impl Market {
    const METRIC_MARKET:              &'static str = "market::time::complete";
    const METRIC_FETCHED_REGION:      &'static str = "market::time::region::fetch";
    const METRIC_FETCH_MARKET_DATA:   &'static str = "market::time::market_data::fetch";
    const METRIC_INSERT_MARKET_DATA:  &'static str = "market::time::market_data::insert";
    const METRIC_PREPARE_ORDER_ID:    &'static str = "market::time::prep::order_id";
    const METRIC_PREPARE_MARKET_INFO: &'static str = "market::time::prep::market_info";
    const METRIC_PREPARE_MARKET_DATA: &'static str = "market::time::prep::market_data";
    const METRIC_SEND_MARKET_INFO:    &'static str = "market::time::send::market_info";
    const METRIC_SEND_MARKET_DATA:    &'static str = "market::time::send::market_data";
    const METRIC_COUNT_MARKET_INFO:   &'static str = "market::count::market_info";
    const METRIC_COUNT_MARKET_DATA:   &'static str = "market::count::market_data";

    pub fn new(eve: EveDataWrapper, metrix: MetrixSender, pool: ConnectionPool) -> Self {
        Self {
            eve,
            metrix,
            pool
        }
    }

    pub async fn task(&mut self) -> Result<(), CollectorError> {
        let start = Instant::now();
        let timer = Instant::now();

        log::info!("Loading eve services");
        let market_service = self.eve.market().await?;
        let system_service = self.eve.systems().await?;
        log::info!("Services loaded");
        let bench = Instant::now();

        let timestamp = previous_30_minute(Utc::now().timestamp() as u64) * 1_000;

        let mut requests = FuturesUnordered::new();
        let regions = system_service.region_ids();

        self.metrix.send_time(Self::METRIC_FETCHED_REGION, timer).await;
        let timer = Instant::now();

        for region in regions {
            requests.push(market_service.orders(*region));
            //requests.push(market_service.orders(10000002));
        }

        let mut results = Vec::new();
        while let Some(return_val) = requests.next().await {
            match return_val {
                Ok(result) => {
                    results.extend(result);
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
        dbg!(bench.elapsed().as_millis());

        Ok(())
    }

    async fn market_info(
        &mut self,
        entries: Vec<MarketOrder>,
        timestamp: u64
    ) -> Result<(), CollectorError> {
        let timer = Instant::now();

        self.metrix.send_time(Self::METRIC_PREPARE_ORDER_ID, timer).await;
        let timer = Instant::now();

        let mut market_order_infos = Vec::with_capacity(MarketOrderCache::CAPACITY);
        for entry in entries.iter() {
            let issued = entry.issued.parse::<DateTime<Utc>>().unwrap();
            let expire = issued.checked_add_signed(chrono::Duration::days(entry.duration as i64)).unwrap();

            let market_order_info = MarketOrderInfoEntry {
                issued:       issued.timestamp() as u64 * 1000,
                expire:       expire.timestamp() as u64 * 100,
                order_id:     entry.order_id,
                location_id:  entry.location_id,
                system_id:    entry.system_id,
                type_id:      entry.type_id,
                volume_total: entry.volume_total,
                price:        entry.price,
                is_buy_order: entry.is_buy_order,
            };
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
            // log::warn!("Market orders was empty");
        }

        self.metrix.send_time(Self::METRIC_SEND_MARKET_INFO, timer).await;
        let timer = Instant::now();

        let mut market_orders = Vec::with_capacity(MarketOrderCache::CAPACITY);
        for entry in entries {
            let market_order = MarketOrderEntry {
                order_id:      entry.order_id,
                timestamp,
                volume_remain: entry.volume_remain,
                type_id:       entry.type_id,
            };
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
            // log::warn!("Market orders was empty");
        }

        self.metrix.send_time(Self::METRIC_SEND_MARKET_DATA, timer).await;
        Ok(())
    }
}
