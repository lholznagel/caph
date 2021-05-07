use super::{MarketOrderCacheV2, MarketOrderEntry, MarketItemOrder};

use crate::{Actions, Commit};

use async_trait::async_trait;
use cachem::{EmptyMsg, Insert, Parse, request};
use std::collections::HashMap;

#[async_trait]
impl Commit<CommitMarketOrderV2Req> for MarketOrderCacheV2 {
    type Response = EmptyMsg;

    async fn commit(&self, _: CommitMarketOrderV2Req) -> Self::Response {
        self.cache.commit();
        EmptyMsg::default()
    }
}

#[request(Actions::CommitMarketOrdersV2)]
#[derive(Debug, Parse)]
pub struct CommitMarketOrderV2Req;

impl MarketOrderCacheV2 {
    fn insert_entry(
        &self,
        entry: MarketOrderEntry,
        x: &mut HashMap<u32, HashMap<u64, Vec<MarketItemOrder>>>
    ) {
        x
            .entry(entry.type_id)
            .and_modify(|x| {
                if let Some(y) = x.get_mut(&entry.order_id) {
                    // Check if the last volume is the same volume than the
                    // new value, if not add the new entry
                    if let Some(x) = y.last() {
                        if x.volume != entry.volume_remain {
                            y.push(
                                MarketItemOrder {
                                    timestamp: entry.timestamp,
                                    order_id: entry.order_id,
                                    volume: entry.volume_remain,
                                }
                            )
                        }
                    }
                } else {
                    // The order does not exists, so add it
                    x.insert(
                        entry.order_id,
                        vec![
                            MarketItemOrder {
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
                        MarketItemOrder {
                            timestamp: entry.timestamp,
                            order_id: entry.order_id,
                            volume: entry.volume_remain,
                        }
                    ]
                );
                map
            });
    }
}

#[async_trait]
impl Insert<InsertMarketOrderV2Req> for MarketOrderCacheV2 {
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertMarketOrderV2Req) -> Self::Response {
        for entry in input.0 {
            self
                .cache
                .write(|x| self.insert_entry(entry, x));
        }

        EmptyMsg::default()
    }
}

#[request(Actions::InsertMarketOrdersV2)]
#[derive(Debug, Parse)]
pub struct InsertMarketOrderV2Req(pub Vec<MarketOrderEntry>);

