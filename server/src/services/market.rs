use crate::services::ItemService;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

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
    /// Resoves the item names to id
    pub names: Option<Vec<String>>,
    /// Filters by security group. If Max = 0.5 then all market orders above 0.5 will be returned
    pub max_security: Option<f32>,
}

#[derive(Clone)]
pub struct MarketService {
    db: Pool<Postgres>,
    item_service: ItemService,
}

impl MarketService {
    pub fn new(db: Pool<Postgres>, item_service: ItemService) -> Self {
        Self { db, item_service }
    }

    pub async fn all(&self, filter: MarketFilter) -> Vec<Market> {
        let mut query = Vec::new();
        query.push(r#"
            SELECT location_id, type_id, system_id, is_buy_order, volume_remain, price, stations.security, stations.region_id
            FROM market
            JOIN stations
            ON stations.solar_system_id = market.system_id
        "#);

        let mut filters = Vec::new();
        if let Some(x) = filter.names.clone() {
            let ids = self.item_service
                .bulk_search(true, x)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|(_, x)| x[0].clone())
                .map(|x| x.id as u32)
                .collect::<Vec<u32>>();
            filters.push(format!("type_id = ANY(ARRAY{:?})", ids));
        }

        if let Some(x) = filter.max_security {
            filters.push(format!("security >= {}", x));
        }

        if let Some(_) = filter.only_buy_orders {
            filters.push("is_buy_order = true".into());
        } else if let Some(_) = filter.only_sell_orders {
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

        let filter = filters.join(" AND ");
        if !filters.is_empty() {
            query.push("WHERE".into());
            query.push(&filter);
        }

        let query = query.join(" ");
        let mut conn = self.db.acquire().await.unwrap();
        sqlx::query_as::<_, Market>(&query)
            .fetch_all(&mut conn)
            .await
            .unwrap()
    }
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Market {
    pub is_buy_order: bool,
    pub location_id: i64,
    pub price: f32,
    pub region_id: i32,
    pub system_id: i32,
    pub security: f32,
    pub type_id: i32,
    pub volume_remain: i32,
}
