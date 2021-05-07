use crate::error::CollectorError;
use crate::time::previous_30_minute;

use cachem::{ConnectionPool, EmptyMsg, Protocol};
use caph_eve_data_wrapper::{EveDataWrapper, MarketOrder};
use caph_db::*;
use chrono::{DateTime, Utc};
use futures::stream::{FuturesUnordered, StreamExt};

pub struct MarketV2 {
    eve:    EveDataWrapper,
    pool:   ConnectionPool,
}

impl MarketV2 {
    pub fn new(eve: EveDataWrapper, pool: ConnectionPool) -> Self {
        Self {
            eve,
            pool,
        }
    }

    pub async fn task(&mut self) -> Result<(), CollectorError> {
        log::info!("Loading eve services");
        let market_service = self.eve.market().await?;
        let system_service = self.eve.systems().await?;
        log::info!("Services loaded");

        let timestamp = previous_30_minute(Utc::now().timestamp() as u64) * 1_000;

        let mut requests = FuturesUnordered::new();
        let regions = system_service.region_ids();
        for region in regions {
            requests.push(market_service.orders(*region));
        }

        let mut results = Vec::with_capacity(1_000_000);
        while let Some(return_val) = requests.next().await {
            match return_val {
                Ok(entries) => {
                    results.extend(entries);
                }
                // if you dont handle errors, there are non
                _ => (),
            }
        }
        self.market_info(results, timestamp).await?;

        // Finally commit all changes
        let mut conn = self.pool.acquire().await.unwrap();
        Protocol::request::<_, EmptyMsg>(
            &mut conn,
            CommitMarketOrderV2Req {}
        )
        .await
        .unwrap();

        Ok(())
    }

    async fn market_info(
        &mut self,
        entries: Vec<MarketOrder>,
        timestamp: u64
    ) -> Result<(), CollectorError> {
        if entries.is_empty() {
            return Ok(());
        }

        let mut market_info = Vec::with_capacity(1_000_000);
        let mut market_data = Vec::with_capacity(1_000_000);

        for entry in entries.iter() {
            let issued = entry.issued.parse::<DateTime<Utc>>().unwrap();
            let expire = issued.checked_add_signed(chrono::Duration::days(entry.duration as i64)).unwrap();

            let market_order_info = MarketOrderInfoEntry {
                issued:       issued.timestamp() as u64 * 1000,
                expire:       expire.timestamp() as u64 * 1000,
                order_id:     entry.order_id,
                location_id:  entry.location_id,
                system_id:    entry.system_id,
                type_id:      entry.type_id,
                volume_total: entry.volume_total,
                price:        entry.price,
                is_buy_order: entry.is_buy_order,
            };

            let market_order = MarketOrderEntry {
                order_id:      entry.order_id,
                timestamp,
                volume_remain: entry.volume_remain,
                type_id:       entry.type_id,
            };

            market_data.push(market_order);
            market_info.push(market_order_info);
        }

        // This write takes very long, about 800ms
        let mut conn = self.pool.acquire().await.unwrap();
        let info = Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertMarketOrderInfoReq(market_info)
        );

        let mut conn = self.pool.acquire().await.unwrap();
        let data = Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertMarketOrderV2Req(market_data)
        );

        let _ = tokio::join! {
            info,
            data
        };

        Ok(())
    }
}
