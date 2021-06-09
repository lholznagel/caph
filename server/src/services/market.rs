use crate::error::EveServerError;
use crate::ItemService;

use cachem::v2::ConnectionPool;
use caph_db_v2::CacheName;
use caph_db_v2::MarketInfoEntry;
use caph_db_v2::MarketOrderRequest;
use caph_db_v2::MarketOrderResponse;
use caph_db_v2::SystemRegionEntry;
use caph_eve_data_wrapper::OrderId;
use caph_eve_data_wrapper::RegionId;
use caph_eve_data_wrapper::SolarSystemId;
use caph_eve_data_wrapper::TypeId;
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
pub struct MarketService {
    pool:         ConnectionPool,
    item_service: ItemService,
}

impl MarketService {
    pub fn new(
        pool:         ConnectionPool,
        item_service: ItemService,
    ) -> Self {
        Self {
            pool,
            item_service,
        }
    }

    pub async fn items(
        &self,
    ) -> Result<Vec<TypeId>, EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .keys(CacheName::MarketOrder)
            .await
            .map_err(Into::into)
    }

    pub async fn top_orders(
        &self,
        item_id: u32,
        req: TopOrderReq,
    ) -> Result<Vec<TopOrder>, EveServerError> {
        let ts = chrono::Utc::now().timestamp() as u64 * 1_000;
        let filter = MarketOrderRequest {
            start: ts,
            end:   ts,
        };
        let market_data = self
            .pool
            .acquire()
            .await?
            .fget::<_, _, _, Vec<MarketOrderResponse>>(CacheName::MarketOrder, item_id, Some(filter))
            .await?
            .unwrap_or_default();

        let order_ids = market_data
            .get(0)
            .unwrap()
            .entries
            .iter()
            .map(|x| x.order_id)
            .collect::<Vec<_>>();
        let market_infos = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, MarketInfoEntry>(CacheName::MarketInfo, order_ids)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

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
            let system_region = self
                .pool
                .acquire()
                .await?
                .get::<_, _, SystemRegionEntry>(CacheName::SystemRegion, x.system_id)
                .await?
                .unwrap();

            let data = market_data
                .get(0)
                .unwrap()
                .entries
                .iter()
                .find(|x| x.order_id == x.order_id)
                .unwrap();
            let order = TopOrder {
                price: x.price,
                region_id: system_region.region_id,
                order_id: x.order_id,
                security: system_region.security,
                system_id: x.system_id,
                timestamp: market_data.get(0).unwrap().timestamp,
                volume_remain: data.volume,
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
        let ts = chrono::Utc::now().timestamp() as u64;
        let ts = Self::previous_30_minute(ts);
        let start_ts = ts - (7 * 48 * 1800 * 1000);

        let filter = MarketOrderRequest {
            start: start_ts,
            end:   ts,
        };
        let market_data = self
            .pool
            .acquire()
            .await?
            .fget::<_, _, _, Vec<MarketOrderResponse>>(CacheName::MarketOrder, item_id, Some(filter))
            .await?
            .unwrap_or_default();

        // Get the order ids from the first layer
        let order_ids = market_data
            .get(0)
            .unwrap()
            .entries
            .iter()
            .map(|x| x.order_id)
            .collect::<Vec<_>>();
        let market_infos = self
            .pool
            .acquire()
            .await?
            .get::<_, _, Vec<MarketInfoEntry>>(CacheName::MarketInfo, order_ids)
            .await?
            .unwrap_or_default();
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

            let volume = market
                .iter()
                .filter(|x| x.volume > 0)
                .map(|x| x.volume as u64)
                .sum();

            let average_price = market_infos_filter
                .iter()
                .map(|x| x.price)
                .sum::<f32>() / volume as f32;

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

    pub async fn stats(
        &self,
        item_id: u32,
        is_buy_order: bool,
    ) -> Result<MarketStats, EveServerError> {
        let ts = chrono::Utc::now().timestamp() as u64 * 1_000;
        let filter = MarketOrderRequest {
            start: ts,
            end:   ts,
        };
        let market_data = self
            .pool
            .acquire()
            .await?
            .fget::<_, _, _, Vec<MarketOrderResponse>>(CacheName::MarketOrder, item_id, Some(filter))
            .await?
            .unwrap_or_default();

        let order_ids = market_data
            .get(0)
            .unwrap()
            .entries
            .iter()
            .map(|x| x.order_id)
            .collect::<Vec<_>>();
        let market_infos = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, MarketInfoEntry>(CacheName::MarketInfo, order_ids)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

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
            .get(0)
            .unwrap()
            .entries
            .iter()
            .filter(|x| order_ids_filtered.contains(&x.order_id))
            .map(|x| x.volume as u64)
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
    pub region_id: RegionId,
    pub order_id: OrderId,
    pub security: f32,
    pub system_id: SolarSystemId,
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
    pub volume: u64,
}
