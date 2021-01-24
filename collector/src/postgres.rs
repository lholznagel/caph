use crate::error::CollectorError;
use crate::metrics::PostgresMetrics;

use metrix::Metrics;
use sqlx::{Pool, Postgres};

macro_rules! table_stats {
    ($self:ident, $table:expr, $name_count:ident, $name_size:ident) => {
        if let Ok(count) = $self.count($table).await {
            $self
                .metrics
                .set(PostgresMetrics::$name_count, count as u64)
                .await;
        } else {
            log::error!("Error getting count of table {}", $table);
        }
        if let Ok(size) = $self.size($table).await {
            $self
                .metrics
                .set(PostgresMetrics::$name_size, size as u64)
                .await;
        } else {
            log::error!("Error getting size of table {}", $table);
        }
    };
}

pub struct PostgresService {
    db: Pool<Postgres>,
    metrics: Metrics,
}

impl PostgresService {
    pub fn new(db: Pool<Postgres>, metrics: Metrics) -> Self {
        Self { db, metrics }
    }

    pub async fn background(&mut self) {
        table_stats!(self, "items", TABLE_ITEMS_COUNT, TABLE_ITEMS_SIZE);

        table_stats!(self, "names", TABLE_NAMES_COUNT, TABLE_NAMES_SIZE);

        table_stats!(self, "stations", TABLE_STATIONS_COUNT, TABLE_STATIONS_SIZE);

        table_stats!(
            self,
            "item_materials",
            TABLE_ITEM_MATERIALS_COUNT,
            TABLE_ITEM_MATERIALS_SIZE
        );

        table_stats!(
            self,
            "blueprints",
            TABLE_BLUEPRINTS_COUNT,
            TABLE_BLUEPRINTS_SIZE
        );

        table_stats!(
            self,
            "blueprint_resources",
            TABLE_BLUEPRINT_RESOURCES_COUNT,
            TABLE_BLUEPRINT_RESOURCES_SIZE
        );

        table_stats!(
            self,
            "schematics",
            TABLE_SCHEMATICS_COUNT,
            TABLE_SCHEMATICS_SIZE
        );

        table_stats!(
            self,
            "schematic_resources",
            TABLE_SCHEMATIC_RESOURCES_COUNT,
            TABLE_SCHEMATIC_RESOURCES_SIZE
        );

        table_stats!(
            self,
            "market_current",
            TABLE_MARKET_CURRENT_COUNT,
            TABLE_MARKET_CURRENT_SIZE
        );

        table_stats!(
            self,
            "market_orders",
            TABLE_MARKET_ORDERS_COUNT,
            TABLE_MARKET_ORDERS_SIZE
        );

        table_stats!(
            self,
            "market_history",
            TABLE_MARKET_HISTORY_COUNT,
            TABLE_MARKET_HISTORY_SIZE
        );
    }

    async fn count(&mut self, table: &str) -> Result<i64, CollectorError> {
        #[derive(sqlx::FromRow)]
        struct Count {
            count: i64,
        }

        let mut conn = self.db.acquire().await?;
        let result =
            sqlx::query_as::<_, Count>(&format!("SELECT COUNT(*) as count FROM {};", table))
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
