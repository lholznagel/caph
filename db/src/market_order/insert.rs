use super::{MarketOrderCache, MarketOrderEntry, MarketItemOrderId};

use crate::Actions;

use async_trait::async_trait;
use cachem::{EmptyResponse, Insert, Parse, Storage, request};
use std::collections::HashMap;
use std::time::Instant;

#[async_trait]
impl Insert<InsertMarketOrderReq> for MarketOrderCache {
    type Error    = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertMarketOrderReq) -> Result<Self::Response, Self::Error> {
        let insert_start = Instant::now();
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

            current
                .entry(entry.item_id)
                .and_modify(|x: &mut Vec<MarketOrderEntry>| x.push(entry))
                .or_insert(vec![entry]);
        }

        *self.current.write().await = current;
        self.save_to_file().await.unwrap();

        self.metrix
            .as_ref()
            .unwrap()
            .send(Self::METRIC_INSERT_DURATION, insert_start.elapsed().as_nanos())
            .await;
        Ok(EmptyResponse::default())
    }
}

#[request(Actions::InsertMarketOrders)]
#[derive(Debug, Parse)]
pub struct InsertMarketOrderReq(pub Vec<MarketOrderEntry>);
