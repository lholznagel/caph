use crate::error::*;
use crate::eve::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketPrice {
    pub type_id: TypeId,
    pub adjusted_price: Option<f32>,
    pub average_price: Option<f32>,
}

impl Eve {
    pub async fn fetch_market_prices(&self) -> Result<Vec<MarketPrice>> {
        let response = self
            .fetch(&format!("markets/prices/?datasource=tranquility&page=1",))
            .await?;

        let market_prices: Vec<MarketPrice> = response.json().await?;
        log::debug!("Downloaded {} Market TypeIds", market_prices.len());

        if market_prices.len() == 20_000 {
            log::warn!("Downloaded more than 20.000 market prices, possible data loss.");
        }
        Ok(market_prices)
    }
}
