use crate::error::EveServerError;

use cachem::{ConnectionPool, Protocol};
use caph_db::{FetchLatestMarketOrderRes, FetchLatestMarketOrdersReq, FetchMarketOrderInfoBulkReq, FetchMarketOrderInfoResBulk, FetchMarketOrderReq, FetchMarketOrderRes, FetchStationReq, FetchStationRes};
use chrono::{NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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
        .map(|x| {
            match x {
                FetchLatestMarketOrderRes::Ok(x) => x,
                _ => Vec::new()
            }
        })?;

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
            .map(|x| {
                match x {
                    FetchStationRes::Ok(x) => x,
                    _ => panic!("No station") // FIXME
                }
            })?;

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

    pub fn previous_30_minute(timestamp: u64) -> u64 {
        let date_time = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        let time = if date_time.minute() >= 50 {
            NaiveTime::from_hms(date_time.hour(), 50, 0)
        } else if date_time.minute() < 20 {
            let duration = chrono::Duration::hours(1);
            let date_time = date_time.checked_sub_signed(duration).unwrap();
            NaiveTime::from_hms(date_time.hour(), 50, 0)
        } else {
            NaiveTime::from_hms(date_time.hour(), 20, 0)
        };
        NaiveDateTime::new(date_time.date(), time).timestamp() as u64 * 1000
    }

    pub async fn historic(&self, item_id: u32, is_buy_order: bool) -> Result<Vec<Historic>, EveServerError> {
        let mut conn = self.0.acquire().await?;
        let ts = chrono::Utc::now().timestamp() as u64;
        let ts = Self::previous_30_minute(ts);
        let start_ts = ts - (1 * 24 * 1800 * 1000);

        let market_data = Protocol::request::<_, FetchMarketOrderRes>(
            &mut conn,
            FetchMarketOrderReq {
                item_id,
                ts_start: start_ts,
                ts_stop: ts
            }
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
        .map(|x| x.0)?;
        let market_infos = market_infos
            .iter()
            .filter(|x| x.is_buy_order == is_buy_order)
            .collect::<Vec<_>>();

        let order_ids = market_infos.iter().map(|x| x.order_id).collect::<Vec<_>>();
        let mut ret = Vec::new();
        for x in market_data {
            let market = x
                .entries
                .iter()
                .filter(|x| order_ids.contains(&x.order_id))
                .collect::<Vec<_>>();

            let market_infos_filter = market_infos
                .iter()
                .filter(|x| market.iter().filter(|y| y.volume > 0).find(|y| x.order_id == y.order_id).is_some())
                .collect::<Vec<_>>();

            let highest_price = market_infos_filter
                .iter()
                .max_by(|a, b| a
                        .price
                        .partial_cmp(&b.price)
                        .unwrap_or(Ordering::Equal)
                )
                .map(|x| x.price)
                .unwrap_or_default();
            let lowest_price = market_infos_filter
                .iter()
                .min_by(|a, b| a
                        .price
                        .partial_cmp(&b.price)
                        .unwrap_or(Ordering::Equal)
                )
                .map(|x| x.price)
                .unwrap_or_default();
            let average_price = market_infos_filter
                .iter()
                .map(|x| x.price)
                .sum::<f32>() / market.len() as f32;

            let volume = market
                .iter()
                .filter(|x| x.volume > 0)
                .map(|x| x.volume)
                .sum();

            ret.push(Historic {
                ts: x.timestamp,
                average_price,
                highest_price,
                lowest_price,
                volume,
            });
        }

        Ok(ret)
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
        .map(|x| {
            match x {
                FetchLatestMarketOrderRes::Ok(x) => x,
                _ => panic!("NO latest market value") // FIXME
            }
        })?;

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
    pub average_price: f32,
    pub highest_price: f32,
    pub lowest_price: f32,
    pub volume: u32,
}
