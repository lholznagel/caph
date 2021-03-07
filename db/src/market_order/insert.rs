use super::{MarketOrderCache, MarketOrderEntry, MarketItemOrderId};

use crate::Actions;

use async_trait::async_trait;
use cachem::{EmptyMsg, Insert, Parse, Storage, request};
use std::collections::HashMap;
use std::time::Instant;

const METRIC_INSERT:         &'static str = "insert::market_order::current::complete";
const METRIC_INSERT_ENTRIES: &'static str = "insert::market_order::current::entries";

#[async_trait]
impl Insert<InsertMarketOrderReq> for MarketOrderCache {
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertMarketOrderReq) -> Self::Response {
        let timer = Instant::now();
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

        self.metrix.send_len(METRIC_INSERT_ENTRIES, current.len()).await;
        *self.current.write().await = current;
        self.save_to_file().await.unwrap();

        self.metrix
            .send_time(METRIC_INSERT, timer)
            .await;
        EmptyMsg::default()
    }
}

#[request(Actions::InsertMarketOrders)]
#[derive(Debug, Parse)]
pub struct InsertMarketOrderReq(pub Vec<MarketOrderEntry>);
