use crate::metrics::MarketMetrics;

use caph_eve_online_api::{EveClient, MarketOrder, RegionId};
use futures::stream::{FuturesUnordered, StreamExt};
use sqlx::{pool::PoolConnection, Executor, Pool, Postgres};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const BATCH_SIZE: usize = 10000;

pub struct Market {
    db: Pool<Postgres>,
    metrics: MarketMetrics,
}

impl Market {
    pub fn new(db: Pool<Postgres>, metrics: MarketMetrics) -> Self {
        Self { db, metrics }
    }

    pub async fn background(&mut self) -> Result<(), ()> {
        let start = Instant::now();
        let client = EveClient::default();

        let mut requests = FuturesUnordered::new();
        let mut conn = self.db.acquire().await.unwrap();
        let regions = sqlx::query_as::<_, Region>("SELECT DISTINCT region_id FROM stations")
            .fetch_all(&mut conn)
            .await
            .map(|x| {
                x.into_iter()
                    .map(|y| y.region_id as u32)
                    .collect::<Vec<u32>>()
            })
            .unwrap();

        for region in regions {
            requests.push(client.fetch_market_orders(RegionId(region)));
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut results = Vec::new();
        while let Some(return_val) = requests.next().await {
            match return_val {
                Ok(result) => {
                    results.extend(result.unwrap_or_default());
                }
                // if you dant handle errors, there are non
                _ => (),
            }
        }
        self.metrics
            .set_timing(MarketMetrics::EVE_DOWNLOAD_TIME, start);

        let start = Instant::now();
        conn.execute("BEGIN").await.unwrap();
        self.clear_market(&mut conn).await.unwrap();
        self.insert_market(&mut conn, results, timestamp)
            .await
            .unwrap();
        conn.execute("COMMIT").await.unwrap();
        self.insert_market_history(&mut conn).await.unwrap();
        self.metrics
            .set_timing(MarketMetrics::TOTAL_DB_INSERT_TIME, start);

        Ok(())
    }

    async fn insert_market(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        entries: Vec<MarketOrder>,
        timestamp: u64,
    ) -> Result<(), ()> {
        log::info!("Starting market import");
        let start = Instant::now();

        let mut skip = 0;
        let mut values_order = Vec::with_capacity(BATCH_SIZE);
        let mut values_market = Vec::with_capacity(BATCH_SIZE);
        let entries_len = entries.len();
        while skip <= entries_len {
            for x in entries.clone().into_iter().skip(skip).take(BATCH_SIZE) {
                values_order.push(format!(
                    "({}, {}, {}, {}, {}, {}, {}, '{}')",
                    x.volume_total,
                    x.system_id.0,
                    x.type_id.0,
                    x.order_id,
                    x.location_id,
                    x.price,
                    x.is_buy_order,
                    x.issued,
                ));

                values_market.push(format!(
                    "({}, {}, {})",
                    x.volume_remain, x.order_id, timestamp
                ));
            }

            if !values_order.is_empty() {
                sqlx::query(&format!(r#"
                    INSERT INTO market_order_info
                    (volume_total, system_id, type_id, order_id, location_id, price, is_buy_order, issued)
                    VALUES {}
                    ON CONFLICT DO NOTHING
                "#, values_order.join(", ")))
                    .execute(&mut *conn)
                    .await
                    .unwrap();
            }

            if !values_market.is_empty() {
                sqlx::query(&format!(
                    r#"
                    INSERT INTO market
                    (volume_remain, order_id, timestamp)
                    VALUES {}
                "#,
                    values_market.join(", ")
                ))
                .execute(&mut *conn)
                .await
                .unwrap();
            }

            skip += BATCH_SIZE;
            values_order.clear();
            values_market.clear();
            log::debug!(
                "[market_import] {} from {} {}",
                skip,
                entries_len,
                (skip as f32 / entries_len as f32) * 100f32
            );
        }

        log::info!("Importing market done. Took {}s", start.elapsed().as_secs());
        self.metrics
            .set_timing(MarketMetrics::MARKET_INFO_INSERT_TIME, start);
        Ok(())
    }

    async fn insert_market_history(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
    ) -> Result<(), ()> {
        log::info!("Starting market history import");
        let start = Instant::now();

        sqlx::query(
            "INSERT INTO market_history SELECT volume_remain, timestamp, order_id FROM market ON CONFLICT DO NOTHING",
        )
        .execute(&mut *conn)
        .await
        .unwrap();

        log::info!(
            "Importing market history done. Took {}s",
            start.elapsed().as_secs()
        );
        self.metrics
            .set_timing(MarketMetrics::MARKET_HISTORY_INSERT_TIME, start);
        Ok(())
    }

    async fn clear_market(&mut self, conn: &mut PoolConnection<Postgres>) -> Result<(), ()> {
        log::debug!("Removing all unlocked market entries");
        let start = Instant::now();

        sqlx::query("DELETE FROM market")
            .execute(&mut *conn)
            .await
            .unwrap();

        log::debug!("Removed all old market entries");
        self.metrics.set_timing(MarketMetrics::CLEANUP_TIME, start);
        Ok(())
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Region {
    pub region_id: i32,
}
