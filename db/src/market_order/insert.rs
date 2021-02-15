use super::{MarketOrderCache, MarketOrderEntry, MarketItemOrderId};

use crate::{Actions, Caches, EmptyResponse};

use async_trait::async_trait;
use cachem::{Insert, Parse, Storage, request};
use std::collections::HashMap;

#[async_trait]
impl Insert<InsertMarketOrderEntries> for MarketOrderCache {
    type Error    = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertMarketOrderEntries) -> Result<Self::Response, Self::Error> {
        let mut current = HashMap::new();

        for entry in input.0 {
            self
                .history
                .write()
                .await
                .entry(entry.item_id)
                .and_modify(|x| {
                    // Look if there is already an entry with the order id and
                    // volume, if not insert it
                    if let None = x
                        .iter()
                        .find(|y| 
                            y.order_id == entry.order_id &&
                            y.volume == entry.volume_remain
                        ) {

                        x.push(
                            MarketItemOrderId {
                                timestamp: entry.timestamp,
                                order_id: entry.order_id,
                                volume: entry.volume_remain,
                            }
                        )
                    }
                })
                .or_insert(vec![
                    MarketItemOrderId {
                        timestamp: entry.timestamp,
                        order_id: entry.order_id,
                        volume: entry.volume_remain,
                    }
                ]);

            // Always insert into the current
            current.insert(entry.order_id, entry);
        }

        *self.current.write().await = current;
        self.save_to_file().await.unwrap();
        Ok(EmptyResponse::default())
    }
}

#[request(Actions::Insert, Caches::MarketOrder)]
#[derive(Debug, Parse)]
pub struct InsertMarketOrderEntries(pub Vec<MarketOrderEntry>);
