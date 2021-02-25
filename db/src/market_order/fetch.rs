use super::MarketOrderCache;

use crate::Actions;

use async_trait::async_trait;
use cachem::{EmptyResponse, Fetch, Parse, request};
use std::collections::HashMap;
use std::time::Instant;

#[async_trait]
impl Fetch<FetchMarketOrderReq> for MarketOrderCache {
    type Error    = EmptyResponse;
    type Response = FetchMarketOrderRes;

    async fn fetch(&self, input: FetchMarketOrderReq) -> Result<Self::Response, Self::Error> {
        let insert_start = Instant::now();

        let item_id = input.0.item_id;
        let ts_start = input.0.ts_start;
        let ts_stop = input.0.ts_stop;

        // Get all item entries that are newer than the given start timestamp
        let items = self
            .history
            .read()
            .await;
        let items = items.get(&item_id)
            .unwrap()
            .into_iter()
            .filter(|x| x.timestamp >= ts_start)
            .collect::<Vec<_>>();

        // Get a list of all order ids and make sure that every id only exist once
        // This is needed to detect missing entries
        let mut unqiue_order_ids = items.iter().map(|x| x.order_id).collect::<Vec<_>>();
        unqiue_order_ids.sort();
        unqiue_order_ids.dedup();

        // Stores all already seen values.
        // Used to fill up missing values
        let mut last_values: HashMap<u64, u32> = HashMap::new();
        let mut ret = HashMap::new();

        let mut ts_current = ts_start;
        while ts_current <= ts_stop {
            // Gets all items that have the current timestamp
            let mut items_filter = items
                .iter()
                .filter(|x| x.timestamp == ts_current)
                .map(|x| (x.order_id, x.volume))
                .collect::<Vec<(u64, u32)>>();
            // Loop over all found items and insert them as there last value
            for (order, volume) in items_filter.iter() {
                last_values.insert(*order, *volume);
            }

            // If the list of items is smaller than the list of unique order ids
            // we need to search which are missing and fill them in
            if items_filter.len() < unqiue_order_ids.len() {
                // Filter all order ids that we already have
                let item_orders = items_filter
                    .iter()
                    .map(|(order, _)| *order)
                    .collect::<Vec<_>>();
                // Find out what order ids are missing
                let missing = unqiue_order_ids
                    .iter()
                    .filter(|x| !item_orders.contains(x))
                    .collect::<Vec<_>>();
                for order_id in missing {
                    // Check if we have an old value in the map and insert it
                    let last_value = last_values.get(&order_id);

                    let volume = if let Some(x) = last_value {
                        *x
                    } else {
                        // We don´t have an old value, so we look in the history
                        // if there is an older entry
                        let items = self.history
                            .read()
                            .await;
                        let mut items = items
                            .get(&item_id)
                            .unwrap()
                            .iter()
                            .filter(|x| x.order_id == *order_id)
                            .filter(|x| x.timestamp < ts_current)
                            .collect::<Vec<_>>();
                        if items.len() > 0 {
                            // Sort the items by timestamp
                            items.sort_by(|a, b| b.timestamp.partial_cmp(&a.timestamp).unwrap_or(std::cmp::Ordering::Equal));
                            // Take the first item and return its volume
                            // We check if the result has more than 0 elements
                            // so the unwrap is save
                            items.first().unwrap().volume
                        } else {
                            // If we don´t have an old value and there is no
                            // historic value, return 0
                            0
                        }
                    };

                    // Make sure that we insert the new last value in the map
                    // for later lookups
                    last_values.insert(*order_id, volume);
                    // Push the value into the result
                    items_filter.push((*order_id, volume));
                }
            }
            // Sort the items by there order id to make sure that those
            // are always in order
            items_filter.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            // Add the items with the current ts into the return map
            ret.insert(ts_current, items_filter);

            // Increase the timestamp to the next 30 minute mark in milliseconds
            ts_current += 1_800 * 1_000;
        }

        let mut response = Vec::with_capacity(ret.len());
        for (ts, data) in ret {
            let mut entries = Vec::with_capacity(data.len());
            for (order, volume) in data {
                entries.push(FetchMarketOrderResponseEntries {
                    order_id: order,
                    volume
                });
            }
            response.push(FetchMarketOrderResponseTs {
                timestamp: ts,
                entries
            });
        }
        response.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap_or(std::cmp::Ordering::Equal));

        self.metrix
            .send_time(Self::METRIC_FETCH_DURATION, insert_start)
            .await;
        Ok(FetchMarketOrderRes(response))
    }
}

#[request(Actions::FetchMarketOrder)]
#[derive(Debug, Parse)]
pub struct FetchMarketOrderReq(pub FetchMarketOrderFilter);

#[derive(Debug, Parse)]
pub struct FetchMarketOrderFilter {
    pub item_id: u32,
    pub ts_start: u64,
    pub ts_stop: u64,
}

#[derive(Debug, Parse)]
pub struct FetchMarketOrderRes(pub Vec<FetchMarketOrderResponseTs>);

#[derive(Debug, Parse)]
pub struct FetchMarketOrderResponseTs {
    pub timestamp: u64,
    pub entries: Vec<FetchMarketOrderResponseEntries>,
}

#[derive(Debug, Parse)]
pub struct FetchMarketOrderResponseEntries {
    pub order_id: u64,
    pub volume: u32,
}
