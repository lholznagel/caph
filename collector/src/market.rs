use crate::error::CollectorError;
use crate::metrics::MarketMetrics;

use caph_eve_online_api::{EveClient, MarketOrder};
use futures::stream::{FuturesUnordered, StreamExt};
use metrix::Metrics;
use sqlx::{pool::PoolConnection, Executor, Pool, Postgres};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const BATCH_SIZE: usize = 500_000;

pub struct Market {
    db: Pool<Postgres>,
    metrics: Metrics,

    values_order: Vec<String>,
    values_market: Vec<String>,
}

impl Market {
    pub fn new(db: Pool<Postgres>, metrics: Metrics) -> Self {
        Self {
            db,
            metrics,
            values_order: Vec::with_capacity(BATCH_SIZE),
            values_market: Vec::with_capacity(BATCH_SIZE),
        }
    }

    pub async fn background(&mut self) -> Result<(), CollectorError> {
        let start = Instant::now();
        let client = EveClient::default();

        let mut requests = FuturesUnordered::new();
        let mut conn = self.db.acquire().await?;
        let regions = sqlx::query_as::<_, Region>("SELECT DISTINCT region_id FROM stations")
            .fetch_all(&mut conn)
            .await
            .map(|x| {
                x.into_iter()
                    .map(|y| y.region_id as u32)
                    .collect::<Vec<u32>>()
            })?;

        for region in regions {
            requests.push(client.fetch_market_orders(region));
        }

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
            .duration(MarketMetrics::EVE_DOWNLOAD_TIME, start)
            .await;

        let start = Instant::now();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| CollectorError::ClockRunsBackwards)?
            .as_millis() as u64;
        conn.execute("BEGIN").await?;
        self.clear_market(&mut conn).await?;
        self.insert_market(&mut conn, results, timestamp).await?;
        conn.execute("COMMIT").await?;
        self.insert_market_history(&mut conn).await?;
        self.metrics
            .duration(MarketMetrics::TOTAL_DB_INSERT_TIME, start)
            .await;
        self.metrics
            .current_timestamp(MarketMetrics::LAST_COMPLETE_READOUT)
            .await;

        Ok(())
    }

    async fn insert_market(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
        entries: Vec<MarketOrder>,
        timestamp: u64,
    ) -> Result<(), CollectorError> {
        log::info!("Starting market import");
        let start = Instant::now();

        let mut skip = 0;
        let entries_len = entries.len();
        while skip <= entries_len {
            for x in entries.clone().into_iter().skip(skip).take(BATCH_SIZE) {
                self.values_order.push(format!(
                    "({}, {}, {}, {}, {}, {}, {}, '{}')",
                    x.volume_total,
                    x.system_id,
                    x.type_id,
                    x.order_id,
                    x.location_id,
                    x.price,
                    x.is_buy_order,
                    x.issued,
                ));

                self.values_market.push(format!(
                    "({}, {}, {})",
                    x.volume_remain, x.order_id, timestamp
                ));
            }

            if !self.values_order.is_empty() {
                sqlx::query(&format!(r#"
                    INSERT INTO market_orders
                    (volume_total, system_id, type_id, order_id, location_id, price, is_buy_order, issued)
                    VALUES {}
                    ON CONFLICT DO NOTHING
                "#, self.values_order.join(", ")))
                    .execute(&mut *conn)
                    .await?;
            }

            if !self.values_market.is_empty() {
                sqlx::query(&format!(
                    r#"
                    INSERT INTO market_current
                    (volume_remain, order_id, timestamp)
                    VALUES {}
                "#,
                    self.values_market.join(", ")
                ))
                .execute(&mut *conn)
                .await?;
            }

            skip += BATCH_SIZE;
            self.values_order.clear();
            self.values_market.clear();
            log::debug!(
                "[market_import] {} from {} {}",
                skip,
                entries_len,
                (skip as f32 / entries_len as f32) * 100f32
            );
        }

        log::info!("Importing market done. Took {}s", start.elapsed().as_secs());
        self.metrics
            .duration(MarketMetrics::MARKET_INFO_INSERT_TIME, start)
            .await;
        Ok(())
    }

    async fn insert_market_history(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
    ) -> Result<(), CollectorError> {
        log::info!("Starting market history import");
        let start = Instant::now();

        sqlx::query(
            "INSERT INTO market_history SELECT volume_remain, timestamp, order_id FROM market_current ON CONFLICT DO NOTHING",
        )
        .execute(&mut *conn)
        .await?;

        log::info!(
            "Importing market history done. Took {}s",
            start.elapsed().as_secs()
        );
        self.metrics
            .duration(MarketMetrics::MARKET_HISTORY_INSERT_TIME, start)
            .await;
        Ok(())
    }

    async fn clear_market(
        &mut self,
        conn: &mut PoolConnection<Postgres>,
    ) -> Result<(), CollectorError> {
        log::debug!("Removing all unlocked market entries");
        let start = Instant::now();

        sqlx::query("DELETE FROM market_current")
            .execute(&mut *conn)
            .await?;

        log::debug!("Removed all old market entries");
        self.metrics
            .duration(MarketMetrics::CLEANUP_TIME, start)
            .await;
        Ok(())
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Region {
    pub region_id: i32,
}
