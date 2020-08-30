use crate::error::*;
use crate::BestPriceResult;

use eve_online_api::*;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct BlueprintItemResult {
    pub item: BestPriceResult,
    pub blueprint: Option<MarketOrder>,
}

pub struct BlueprintItem {
    region: RegionId,
}

impl BlueprintItem {
    pub fn new(region: RegionId) -> Self {
        Self { region }
    }

    pub async fn collect(&self, type_data: Vec<Type>) -> Result<Vec<BlueprintItemResult>> {
        let mut result = Vec::new();

        for (counter, data) in type_data.clone().into_iter().enumerate() {
            let blueprint_cost = self.get_blueprint_cost(TypeId(data.type_id.0 + 1)).await?;
            let item_best_price = BestPrice::new(self.region).collect(vec![data]).await?;

            if item_best_price.len() == 1 {
                result.push(BlueprintItemResult {
                    item: item_best_price.get(0).unwrap().clone(),
                    blueprint: blueprint_cost,
                });
            }

            log::info!("{} from {} done", counter, type_data.len() - 1);
        }

        result.sort_by(|x, y| {
            y.item
                .order
                .price
                .partial_cmp(&x.item.order.price)
                .unwrap_or(Ordering::Equal)
        });

        Ok(result)
    }

    async fn get_blueprint_cost(&self, type_id: TypeId) -> Result<Option<MarketOrder>> {
        Ok(Eve::default()
            .fetch_market_orders(self.region, type_id)
            .await?
            .into_iter()
            .filter(|x| !x.is_buy_order)
            .enumerate()
            .max_by(|(_, a), (_, b)| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
            .map(|(_, x)| x))
    }
}
