use crate::eve_client::*;
use crate::fetch;
use crate::EveApiError;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketHistory {
    pub average: f32,
    pub date: String,
    pub highest: f32,
    pub lowest: f32,
    pub order_count: u64,
    pub volume: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketOrder {
    pub duration: u32,
    pub is_buy_order: bool,
    pub issued: String,
    pub location_id: u64,
    pub min_volume: u32,
    pub order_id: u64,
    pub price: f32,
    pub range: String,
    pub system_id: SystemId,
    pub type_id: TypeId,
    pub volume_remain: u32,
    pub volume_total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketGroup {
    pub description: String,
    pub market_group_id: GroupId,
    pub name: String,
    pub types: Vec<u32>,

    pub parent_group_id: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketPrice {
    pub type_id: TypeId,

    pub adjusted_price: Option<f32>,
    pub average_price: Option<f32>,
}

impl EveClient {
    fetch!(
        fetch_market_history,
        "markets",
        "history",
        RegionId,
        Vec<MarketHistory>
    );
    fetch!(
        fetch_market_orders,
        "markets",
        "orders",
        RegionId,
        Vec,
        MarketOrder
    );
    fetch!(
        fetch_market_types,
        "markets",
        "types",
        RegionId,
        Vec<TypeId>
    );
    fetch!(fetch_market_prices, "markets/prices", Vec<MarketPrice>);

    fetch!(fetch_market_groups, "markets/groups", GroupId, Vec<GroupId>);

    /// The default for order type is all
    pub async fn fetch_market_orders_by_id(
        &self,
        id: &RegionId,
        type_id: &TypeId,
        order_type: &str,
    ) -> crate::error::Result<Option<Vec<MarketOrder>>> {
        let order_type = match order_type {
            "all" | "buy" | "sell" => order_type,
            _ => "all",
        };

        let mut response = self
            .fetch(&format!(
                "markets/{}/orders?type_id={}&order_type={}",
                id, type_id, order_type
            ))
            .await?;

        if response.status() == 404 {
            return Ok(None);
        }

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<MarketOrder> = Vec::new();
        fetched_data.extend(response.body_json::<Vec<MarketOrder>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "markets/{}/orders?type_id={}&order_type={}&page={}",
                    id, type_id, order_type, page
                ))
                .await?
                .body_json::<Vec<MarketOrder>>()
                .await
                .map_err(EveApiError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(Some(fetched_data))
    }
}
