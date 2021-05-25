use crate::error::CollectorError;
use crate::time::previous_30_minute;

use cachem::{ConnectionPool, EmptyMsg, Protocol};
use caph_db::*;
use caph_eve_data_wrapper::{EveDataWrapper, MarketOrder};
use chrono::{DateTime, Utc};
use futures::stream::{FuturesUnordered, StreamExt};

pub struct Market {
    eve:    EveDataWrapper,
    pool: ConnectionPool,
}

impl Market {
    pub fn new(eve: EveDataWrapper, pool: ConnectionPool) -> Self {
        Self {
            eve,
            pool
        }
    }

    /// Runs a task in the background that periodically collects all market
    /// entries from all markets and writes them into the database.
    pub async fn task(&mut self) -> Result<(), CollectorError> {
        log::info!("Loading eve services");
        let market_service = self.eve.market().await?;
        let system_service = self.eve.systems().await?;
        log::info!("Services loaded");

        let timestamp = previous_30_minute(Utc::now().timestamp() as u64)? * 1_000;

        let mut requests = FuturesUnordered::new();
        let regions = system_service.region_ids();

        for region in regions {
            requests.push(market_service.orders(*region));
        }

        let mut results = Vec::new();
        while let Some(return_val) = requests.next().await {
            if let Ok(r) = return_val {
                results.extend(r);
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
        let mut conn = self.pool.acquire().await?;

        let mut market_order_infos = Vec::with_capacity(MarketOrderCache::CAPACITY);
        for entry in entries.iter() {
            let issued = entry.issued.parse::<DateTime<Utc>>()?;
            let expire = issued.checked_add_signed(chrono::Duration::days(entry.duration as i64)).ok_or(CollectorError::ChronoError)?;

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

        if !market_order_infos.is_empty() {
            Protocol::request::<_, EmptyMsg>(
                &mut conn,
                InsertMarketOrderInfoReq(market_order_infos)
            )
            .await?;

            Protocol::request::<_, EmptyMsg>(
                &mut conn,
                InsertMarketOrderReq(market_orders)
            )
            .await?;
        }

        Ok(())
    }
}
