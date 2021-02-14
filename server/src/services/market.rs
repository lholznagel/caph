use crate::error::EveServerError;

use cachem::{ConnectionPool, Protocol};
use caph_db::{MarketOrderSaveEntry};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MarketFilter {
    /// Returns only buy orders
    #[serde(rename = "onlyBuyOrders")]
    pub only_buy_orders: Option<bool>,
    /// Returns only sell orders
    #[serde(rename = "onlySellOrders")]
    pub only_sell_orders: Option<bool>,
    /// Filters by location id
    #[serde(rename = "locationIds")]
    pub location_ids: Option<Vec<u64>>,
    /// Filters by system id
    #[serde(rename = "systemIds")]
    pub system_ids: Option<Vec<u32>>,
    /// Filters by type ids
    pub ids: Option<Vec<u32>>,
    /// Filters by security group. If Max = 0.5 then all market orders above 0.5 will be returned
    #[serde(rename = "maxSecurity")]
    pub max_security: Option<f32>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<u32>,
    #[serde(rename = "sortPrice")]
    pub sort_price: Option<Sort>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Sort {
    #[serde(alias = "asc")]
    #[serde(alias = "ASC")]
    Asc,
    #[serde(alias = "desc")]
    #[serde(alias = "DESC")]
    Desc,
}

#[derive(Clone)]
pub struct MarketService {
    pool: ConnectionPool,
}

impl MarketService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }

    /*pub async fn raw(&self) -> Result<Vec<Orders>, EveServerError> {
        let mut conn = self.pool.acquire().await?;

        let a = Protocol::request::<_, Market>(
            &mut conn,
            RawMarketOrderEntries::default()
        )
        .await
        .unwrap()
        .0
        .into_iter()
        .filter(|x| x.item_id == 1230)
        .collect::<Vec<_>>();

        let mut transformed = Vec::with_capacity(a.len());
        for x in a {
            transformed.push(Orders::from(x));
        }

        Ok(transformed)
    }*/
}
/*
    pub async fn all(&self, filter: MarketFilter) -> Result<Vec<Market>, EveServerError> {
        let mut query = Vec::new();
        query.push(r#"
        SELECT DISTINCT market_current.order_id, market_orders.location_id, market_orders.type_id, market_orders.system_id, market_orders.is_buy_order, market_orders.price, market_current.volume_remain, stations.security, stations.region_id
        FROM market_orders
        JOIN stations
        ON market_orders.system_id = stations.system_id
        JOIN market_current
        ON market_current.order_id = market_orders.order_id
        "#);

        let mut filters = Vec::new();
        if let Some(x) = filter.max_security {
            filters.push(format!("security >= {}", x));
        }

        if let Some(true) = filter.only_buy_orders {
            filters.push("is_buy_order = true".into());
        } else if let Some(true) = filter.only_sell_orders {
            filters.push("is_buy_order = false".into());
        }

        if let Some(x) = filter.ids {
            filters.push(format!("type_id = ANY(ARRAY{:?})", x));
        }

        if let Some(x) = filter.location_ids {
            filters.push(format!("location_id = ANY(ARRAY{:?})", x));
        }

        if let Some(x) = filter.system_ids {
            filters.push(format!("location_id = ANY(ARRAY{:?})", x));
        }

        let combined_filter = filters.join(" AND ");
        if !filters.is_empty() {
            query.push("WHERE".into());
            query.push(&combined_filter);
        }

        let mut limits = Vec::new();
        if let Some(x) = filter.sort_price {
            limits.push(format!("ORDER BY price {:?}", x));
        }

        if let Some(x) = filter.max_items {
            limits.push(format!("LIMIT {}", x));
        }

        let limiter = limits.join(" ");
        if !limits.is_empty() {
            query.push(&limiter);
        }

        let query = query.join(" ");
        let mut conn = self.db.acquire().await.unwrap();
        sqlx::query_as::<_, Market>(&query)
            .fetch_all(&mut conn)
            .await
            .map_err(|x| x.into())
    }

    pub async fn fetch_by_item_id(&self, item_id: u32) -> Result<Vec<Market>, EveServerError> {
        let mut conn = self.db.acquire().await.unwrap();
        sqlx::query_as::<_, Market>(r#"SELECT market_history.volume_remain, market_history.timestamp, market_history.order_id,
                market_orders.price, market_orders.is_buy_order, stations.region_id, stations.system_id, stations.security
            FROM market_history
            JOIN market_orders
                ON market_history.order_id = market_orders.order_id
            JOIN stations
                ON market_orders.system_id = stations.system_id
            WHERE market_orders.type_id = $1
            ORDER BY price DESC"#)
            .bind(item_id as i32)
            .fetch_all(&mut conn)
            .await
            .map_err(|x| x.into())
    }

    pub async fn stats(
        &self,
        item_id: u32,
        is_buy_order: bool,
    ) -> Result<MarketStats, EveServerError> {
        #[derive(sqlx::FromRow)]
        struct CountResult {
            count: i64,
        }

        #[derive(sqlx::FromRow)]
        struct PriceResult {
            price: f32,
        }

        let mut conn = self.db.acquire().await.unwrap();
        let highest_price = sqlx::query_as::<_, PriceResult>(
            r#"SELECT MAX(market_orders.price) as price
            FROM market_orders
            JOIN market_history
            ON market_orders.order_id = market_history.order_id
            WHERE market_history.volume_remain > 0
            AND market_orders.type_id = $1
            AND market_orders.is_buy_order = $2
            "#,
        )
        .bind(item_id as i32)
        .bind(is_buy_order)
        .fetch_one(&mut conn)
        .await
        .unwrap()
        .price;
        let lowest_price = sqlx::query_as::<_, PriceResult>(
            r#"SELECT MIN(market_orders.price) as price
            FROM market_orders
            JOIN market_history
            ON market_orders.order_id = market_history.order_id
            WHERE market_history.volume_remain > 0
            AND market_orders.type_id = $1
            AND market_orders.is_buy_order = $2
            "#,
        )
        .bind(item_id as i32)
        .bind(is_buy_order)
        .fetch_one(&mut conn)
        .await
        .unwrap()
        .price;
        let average_price = sqlx::query_as::<_, PriceResult>(
            r#"SELECT CAST(AVG(market_orders.price) as real) as price
            FROM market_orders
            JOIN market_history
            ON market_orders.order_id = market_history.order_id
            WHERE market_history.volume_remain > 0
            AND market_orders.type_id = $1
            AND market_orders.is_buy_order = $2
            "#,
        )
        .bind(item_id as i32)
        .bind(is_buy_order)
        .fetch_one(&mut conn)
        .await
        .unwrap()
        .price;

        let order_count = sqlx::query_as::<_, CountResult>(
            r#"SELECT COUNT(*) as count
            FROM market_orders
            JOIN market_history
            ON market_orders.order_id = market_history.order_id
            WHERE market_history.volume_remain > 0
            AND market_orders.type_id = $1
            AND market_orders.is_buy_order = $2
            "#,
        )
        .bind(item_id as i32)
        .bind(is_buy_order)
        .fetch_one(&mut conn)
        .await
        .unwrap()
        .count as u32;
        let total_volume = sqlx::query_as::<_, CountResult>(
            r#"SELECT CAST(volume_remain as BIGINT) as count
            FROM market_orders
            JOIN market_history
            ON market_orders.order_id = market_history.order_id
            WHERE market_history.volume_remain > 0
            AND market_orders.type_id = $1
            AND market_orders.is_buy_order = $2
            "#,
        )
        .bind(item_id as i32)
        .bind(is_buy_order)
        .fetch_all(&mut conn)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.count as u64)
        .sum();

        Ok(MarketStats {
            average_price,
            highest_price,
            lowest_price,
            order_count,
            total_volume,
        })
    }
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Market {
    pub is_buy_order: bool,
    pub location_id: i64,
    pub order_id: i64,
    pub price: f32,
    pub region_id: i32,
    pub security: f32,
    pub system_id: i32,
    pub type_id: i32,
    pub volume_remain: i32,
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct MarketStats {
    pub average_price: f32,
    pub highest_price: f32,
    pub lowest_price: f32,
    pub order_count: u32,
    pub total_volume: u64,
}
*/
