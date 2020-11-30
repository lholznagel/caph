use crate::error::CollectorError;
use crate::metrics::PostgresMetrics;

use sqlx::{Pool, Postgres};

pub struct PostgresService {
    db: Pool<Postgres>,
    metrics: PostgresMetrics,
}

impl PostgresService {
    pub fn new(db: Pool<Postgres>, metrics: PostgresMetrics) -> Self {
        Self { db, metrics }
    }

    pub async fn background(&mut self) -> Result<(), CollectorError> {
        let count = self.count("items").await?;
        let size = self.size("items").await?;
        self.metrics.set(PostgresMetrics::TABLE_ITEMS, count, size);

        let count = self.count("item_materials").await?;
        let size = self.size("item_materials").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_ITEM_MATERIALS, count, size);

        let count = self.count("names").await?;
        let size = self.size("names").await?;
        self.metrics.set(PostgresMetrics::TABLE_NAMES, count, size);

        let count = self.count("stations").await?;
        let size = self.size("stations").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_STATIONS, count, size);

        let count = self.count("blueprints").await?;
        let size = self.size("blueprints").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_BLUEPRINTS, count, size);

        let count = self.count("blueprint_resources").await?;
        let size = self.size("blueprint_resources").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_BLUEPRINT_RESOURCES, count, size);

        let count = self.count("schematics").await?;
        let size = self.size("schematics").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_SCHEMATICS, count, size);

        let count = self.count("schematic_resources").await?;
        let size = self.size("schematic_resources").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_SCHEMATIC_RESOURCES, count, size);

        let count = self.count("market").await?;
        let size = self.size("market").await?;
        self.metrics.set(PostgresMetrics::TABLE_MARKET, count, size);

        let count = self.count("market_order_info").await?;
        let size = self.size("market_order_info").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_MARKET_ORDER_INFO, count, size);

        let count = self.count("market_history").await?;
        let size = self.size("market_history").await?;
        self.metrics
            .set(PostgresMetrics::TABLE_MARKET_HISTORY, count, size);

        Ok(())
    }

    async fn count(&mut self, table: &str) -> Result<i64, CollectorError> {
        #[derive(sqlx::FromRow)]
        struct Count {
            count: i64,
        }

        let mut conn = self.db.acquire().await?;
        let result = sqlx::query_as::<_, Count>(
            r#"
            SELECT reltuples::bigint AS count
            FROM pg_catalog.pg_class
            WHERE relname = $1;
        "#,
        )
        .bind(table)
        .fetch_one(&mut conn)
        .await?
        .count;
        Ok(result)
    }

    async fn size(&mut self, table: &str) -> Result<i64, CollectorError> {
        #[derive(sqlx::FromRow)]
        struct Size {
            size: i64,
        }

        let mut conn = self.db.acquire().await?;
        let result = sqlx::query_as::<_, Size>("SELECT pg_total_relation_size($1) AS size;")
            .bind(table)
            .fetch_one(&mut conn)
            .await?
            .size;
        Ok(result)
    }
}
