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
                    if let Some(y) = x.get_mut(&entry.order_id) {
                        if y.last().unwrap().volume != entry.volume_remain {
                            y.push(
                                MarketItemOrderId {
                                    timestamp: entry.timestamp,
                                    order_id: entry.order_id,
                                    volume: entry.volume_remain,
                                }
                            )
                        }
                    } else {
                        x.insert(
                            entry.order_id,
                            vec![
                                MarketItemOrderId {
                                    timestamp: entry.timestamp,
                                    order_id: entry.order_id,
                                    volume: entry.volume_remain,
                                }
                            ]
                        );
                    }
                })
                .or_insert({
                    let mut map = HashMap::new();
                    map.insert(
                        entry.order_id,
                        vec![
                            MarketItemOrderId {
                                timestamp: entry.timestamp,
                                order_id: entry.order_id,
                                volume: entry.volume_remain,
                            }
                        ]
                    );
                    map
                });

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

/*#[cfg(test)]
mod tests_insert_market_orders {
    use super::*;

    use metrix_exporter::MetrixSender;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn insert() {
        let metrix = MetrixSender::new_test();
        let cache = MarketOrderCache {
            current: RwLock::new(HashMap::new()),
            history: RwLock::new(HashMap::new()),
            metrix,
        };

        let data = vec![
            MarketOrderEntry {
                order_id: 0u64,
                timestamp: 0 * 1800 * 1000,
                volume_remain: 100,
                item_id: 1337
            },
            MarketOrderEntry {
                order_id: 1u64,
                timestamp: 0 * 1800 * 1000,
                volume_remain: 50,
                item_id: 1338
            }
        ];

        cache.insert(InsertMarketOrderReq(data)).await;
        
        let mut history_expected = HashMap::new();
        let mut orders_1_expected = HashMap::new();
        let mut orders_2_expected = HashMap::new();
        orders_1_expected.insert(0u64, vec![
            MarketItemOrderId {
                order_id: 0u64,
                timestamp: 0 * 1800 * 1000,
                volume: 100,
            }
        ]);
        orders_2_expected.insert(1u64, vec![
            MarketItemOrderId {
                order_id: 1u64,
                timestamp: 0 * 1800 * 1000,
                volume: 50,
            }
        ]);
        history_expected.insert(1337u32, orders_1_expected);
        history_expected.insert(1338u32, orders_2_expected);
        let is = &*cache.history.read().await;
        assert_eq!(&history_expected, is);
    }
}*/
