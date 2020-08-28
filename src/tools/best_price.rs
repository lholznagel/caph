use crate::error::*;
use crate::*;

use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct BestPriceResult {
    pub buyers: u32,
    pub sellers: u32,
    pub item_name: String,
    pub order: MarketOrder,
    pub potential_market: f32,
    pub max_cargo: Option<f32>,
}

pub struct BestPrice {
    cargo_size: Option<u32>,
    region: RegionId,
}

impl BestPrice {
    pub fn new(region: RegionId) -> Self {
        Self {
            cargo_size: None,
            region,
        }
    }

    pub fn cargo_size(mut self, cargo_size: u32) -> Self {
        self.cargo_size = Some(cargo_size);
        self
    }

    pub async fn collect(self, type_data: Vec<TypeData>) -> Result<Vec<BestPriceResult>> {
        let mut result = Vec::new();
        for data in type_data {
            if let (Some(x), buyers, sellers, potential_market) =
                self.find_best_order(&data).await?
            {
                result.push(BestPriceResult {
                    buyers,
                    sellers,
                    max_cargo: self.calc_max_items_for_cargo(&data),
                    order: x,
                    potential_market,
                    item_name: data.name,
                });
            } else {
                log::debug!("No market order for item {}. Skipping", data.name);
            }
        }

        result.sort_by(|x, y| {
            y.order
                .price
                .partial_cmp(&x.order.price)
                .unwrap_or(Ordering::Equal)
        });

        Ok(result)
    }

    fn calc_max_items_for_cargo(&self, type_data: &TypeData) -> Option<f32> {
        let cargo_size = if let Some(x) = self.cargo_size {
            x
        } else {
            return None;
        };

        // Attribute 161 -> density
        if let Some(x) = type_data.find_dogma(AttributeId(161)) {
            Some(cargo_size as f32 / x)
        } else {
            log::warn!("Could not find density dogma.");
            None
        }
    }

    async fn find_best_order(
        &self,
        type_data: &TypeData,
    ) -> Result<(Option<MarketOrder>, u32, u32, f32)> {
        let orders = Eve::default()
            .fetch_market_orders(self.region, type_data.type_id)
            .await?;

        let buy_order = orders
            .clone()
            .into_iter()
            .filter(|x| x.is_buy_order)
            .enumerate()
            .max_by(|(_, a), (_, b)| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
            .map(|(_, x)| x);

        let buyers = orders
            .clone()
            .into_iter()
            .filter(|x| x.is_buy_order)
            .count() as u32;

        let sellers = orders
            .clone()
            .into_iter()
            .filter(|x| !x.is_buy_order)
            .count() as u32;

        let mut potential_market = 0f32;
        for order in orders {
            if order.is_buy_order {
                potential_market += order.volume_remain as f32 * order.price;
            }
        }

        Ok((buy_order, buyers, sellers, potential_market))
    }
}
