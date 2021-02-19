use std::cmp::Ordering;
use std::time::SystemTime;

use crate::error::EveServerError;

use cachem::{ConnectionPool, Protocol};
use caph_db::{FetchLatestMarketOrderRes, FetchLatestMarketOrdersReq, FetchMarketOrderFilter, FetchMarketOrderInfoBulkReq, FetchMarketOrderInfoResBulk, FetchMarketOrderReq, FetchMarketOrderRes, FetchStationReq, FetchStationRes};
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
pub struct MarketService(ConnectionPool);

impl MarketService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self(pool)
    }

    pub async fn top_orders(
        &self,
        item_id:u32,
        req: TopOrderReq,
    ) -> Result<Vec<TopOrder>, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let market_data = Protocol::request::<_, FetchLatestMarketOrderRes>(
            &mut conn,
            FetchLatestMarketOrdersReq(item_id)
        )
        .await
        .map(|x| x.0)?;

        let order_ids = market_data.iter().map(|x| x.order_id).collect::<Vec<_>>();
        let market_infos = Protocol::request::<_, FetchMarketOrderInfoResBulk>(
            &mut conn,
            FetchMarketOrderInfoBulkReq(order_ids)
        )
        .await
        .map(|x| x.0)?;

        let mut market_infos = market_infos
            .iter()
            .filter(|x| x.is_buy_order == req.is_buy_order)
            .collect::<Vec<_>>();

        match req.sort {
            Sort::Asc => {
                market_infos
                    .sort_by(|a, b|
                        a
                            .price
                            .partial_cmp(&b.price)
                            .unwrap_or(Ordering::Equal)
                    );
            },
            Sort::Desc => {
                market_infos
                    .sort_by(|a, b|
                        b
                            .price
                            .partial_cmp(&a.price)
                            .unwrap_or(Ordering::Equal)
                    );
            }
        }

        let subset = if market_infos.len() >= req.count {
            market_infos[0..req.count as usize].to_vec()
        } else {
            market_infos
        };

        let mut ret = Vec::with_capacity(subset.len());
        for x in subset {
            let station = Protocol::request::<_, FetchStationRes>(
                &mut conn,
                FetchStationReq(x.system_id)
            )
            .await
            .map(|x| x.0)?;

            let data = market_data.iter().find(|x| x.order_id == x.order_id).unwrap();
            let order = TopOrder {
                price: x.price,
                region_id: station.region_id,
                order_id: x.order_id,
                security: station.security,
                system_id: x.system_id,
                timestamp: data.timestamp,
                volume_remain: data.volume_remain,
            };
            ret.push(order);
        }

        Ok(ret)
    }

    pub async fn historic(&self, item_id: u32, is_buy_order: bool) -> Result<Vec<Historic>, EveServerError> {
        /*pub struct Historic {
            pub ts: u64,
            pub average: u64,
            pub min: u64,
            pub max: u64,
        }*/

        let mut conn = self.0.acquire().await?;
        let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let start_ts = ts - (48 * 1800);

        let market_data = Protocol::request::<_, FetchMarketOrderRes>(
            &mut conn,
            FetchMarketOrderReq(FetchMarketOrderFilter {
                item_id,
                ts_start: start_ts,
                ts_stop: ts
            })
        )
        .await
        .map(|x| x.0)?;

        // Get the order ids from the first layer
        let order_ids = market_data
            .get(0)
            .unwrap()
            .entries
            .iter()
            .map(|x| x.order_id)
            .collect::<Vec<_>>();
        let market_infos = Protocol::request::<_, FetchMarketOrderInfoResBulk>(
            &mut conn,
            FetchMarketOrderInfoBulkReq(order_ids)
        )
        .await
        .map(|x| x.0)?
        .iter()
        .filter(|x| x.is_buy_order == is_buy_order)
        .collect::<Vec<_>>();

        let mut result = Vec::new();
        for x in market_data {
            let filtered = x
                .entries
                .iter()
                .filter(|x| 
                    market_infos
                        .iter()
                        .find(|y| y.order_id == x.order_id)
                        .is_some()
                )
                .collect::<Vec<_>>();

            let highest_price = filtered
                .iter()
                .max_by(|a, b| a
                        .price
                        .partial_cmp(&b.price)
                        .unwrap_or(Ordering::Equal)
                )
                .map(|x| x.price)
                .unwrap_or_default();
            let lowest_price = filtered
                .iter()
                .min_by(|a, b| a
                        .price
                        .partial_cmp(&b.price)
                        .unwrap_or(Ordering::Equal)
                )
                .map(|x| x.price)
                .unwrap_or_default();
            let average_price = filtered
                .iter()
                .map(|x| x.price)
                .sum::<f32>() / filtered.len() as f32;
        }

        Ok(Vec::new())
    }

    /*pub async fn all(&self, filter: MarketFilter) -> Result<Vec<Market>, EveServerError> {
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
    }*/

    /*pub async fn fetch_by_item_id(&self, item_id: u32) -> Result<Vec<Market>, EveServerError> {
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
    }*/

    pub async fn stats(
        &self,
        item_id: u32,
        is_buy_order: bool,
    ) -> Result<MarketStats, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let market_data = Protocol::request::<_, FetchLatestMarketOrderRes>(
            &mut conn,
            FetchLatestMarketOrdersReq(item_id)
        )
        .await
        .map(|x| x.0)?;

        let order_ids = market_data.iter().map(|x| x.order_id).collect::<Vec<_>>();
        let market_infos = Protocol::request::<_, FetchMarketOrderInfoResBulk>(
            &mut conn,
            FetchMarketOrderInfoBulkReq(order_ids)
        )
        .await
        .map(|x| x.0)?;

        let market_infos = market_infos
            .iter()
            .filter(|x| x.is_buy_order == is_buy_order)
            .collect::<Vec<_>>();

        let highest_price = market_infos
            .iter()
            .max_by(|a, b| a
                    .price
                    .partial_cmp(&b.price)
                    .unwrap_or(Ordering::Equal)
            )
            .map(|x| x.price)
            .unwrap_or_default();
        let lowest_price = market_infos
            .iter()
            .min_by(|a, b| a
                    .price
                    .partial_cmp(&b.price)
                    .unwrap_or(Ordering::Equal)
            )
            .map(|x| x.price)
            .unwrap_or_default();
        let average_price = market_infos
            .iter()
            .map(|x| x.price)
            .sum::<f32>() / market_infos.len() as f32;

        let order_count = market_infos
            .iter()
            .count() as u32;
        
        let order_ids_filtered = market_infos
            .iter()
            .map(|x| x.order_id)
            .collect::<Vec<_>>();
        let total_volume = market_data
            .iter()
            .filter(|x| order_ids_filtered.contains(&x.order_id))
            .map(|x| x.volume_remain as u64)
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

#[derive(Clone, Debug, Serialize)]
pub struct MarketStats {
    pub average_price: f32,
    pub highest_price: f32,
    pub lowest_price: f32,
    pub order_count: u32,
    pub total_volume: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct TopOrder {
    pub price: f32,
    pub region_id: u32,
    pub order_id: u64,
    pub security: f32,
    pub system_id: u32,
    pub timestamp: u64,
    pub volume_remain: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TopOrderReq {
    pub is_buy_order: bool,
    pub count: usize,
    pub sort: Sort,
}

#[derive(Serialize)]
pub struct Historic {
    pub ts: u64,
    pub average: u64,
    pub min: u64,
    pub max: u64,
}
