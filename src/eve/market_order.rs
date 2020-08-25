use crate::error::*;
use crate::eve::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketOrder {
    pub duration: u32,
    pub is_buy_order: bool,
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

impl Eve {
    pub async fn fetch_market_orders(
        &self,
        region_id: RegionId,
        type_id: TypeId,
    ) -> Result<Vec<MarketOrder>> {
        let response = self
            .fetch(&format!(
                "markets/{}/orders/?datasource=tranquility&order_type=all&type_id={}&page=1",
                region_id.0,
                type_id.0
            ))
            .await?;

        let pages = self.page_count(&response);
        log::debug!("Downloaded market orders page  1 from {}", pages);

        let mut market_buy_orders: Vec<MarketOrder> = Vec::with_capacity((pages as u16 * 1_000) as usize);

        let first: Vec<MarketOrder> = response.json().await?;
        market_buy_orders.extend(first);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "universe/types/?datasource=tranquility&page={}",
                    page
                ))
                .await?
                .json::<Vec<MarketOrder>>()
                .await
                .map_err(EveError::ReqwestError)?;

            market_buy_orders.extend(next_page);
            log::debug!("Downloaded market orders page {:2} from {}", page, pages);
        }

        log::debug!("Downloaded {} market buys", market_buy_orders.len());
        Ok(market_buy_orders)
    }
}
