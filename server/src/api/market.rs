use crate::cache::MarketCacheEntry;
use crate::State;

use serde::Deserialize;
use std::collections::HashMap;
use tide::{Body, Request, Result};

pub async fn fetch_raw(mut req: Request<State>) -> Result<Body> {
    let filter: MarketFilter = req.body_json().await?;
    let data = req
        .state()
        .market_cache
        .fetch()
        .await
        .into_iter()
        .filter(|(id, _)| {
            if let Some(type_ids) = filter.clone().type_ids {
                if !type_ids.contains(&id) {
                    return false;
                }
            }
            true
        })
        .map(|(id, entries)| {
            let entries = entries
                .into_iter()
                .filter(|entry| {
                    let filter = filter.clone();

                    // when set, the order must be a buy order
                    if let Some(_) = filter.is_buy_order {
                        if !entry.is_buy_order {
                            return false
                        }
                    // when set, the order cannot be a buy order
                    } else if let Some(_) = filter.is_sell_order {
                        if entry.is_buy_order {
                            return false
                        }
                    }

                    if let Some(system_ids) = filter.system_ids {
                        if !system_ids.contains(&entry.system_id) {
                            return false;
                        }
                    }

                    if let Some(location_ids) = filter.location_ids {
                        if !location_ids.contains(&entry.location_id) {
                            return false;
                        }
                    }

                    true
                })
                .collect::<Vec<MarketCacheEntry>>();

            (id, entries)
        })
        .collect::<HashMap<u32, Vec<MarketCacheEntry>>>();

    Ok(Body::from_json(&data).unwrap())
}

// TODO: filter by location name, system name, item names
#[derive(Clone, Debug, Deserialize)]
struct MarketFilter {
    /// Returns only buy orders
    is_buy_order: Option<bool>,
    /// Returns only sell orders
    is_sell_order: Option<bool>,
    /// Filters by location id
    location_ids: Option<Vec<u64>>,
    /// Filters by system id
    system_ids: Option<Vec<u32>>,
    /// Filters by type ids
    type_ids: Option<Vec<u32>>,
}