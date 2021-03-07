use std::collections::HashMap;
use std::time::Instant;

use crate::{Actions, ItemCache, ItemEntry};

use async_trait::*;
use cachem::{EmptyMsg, Insert, Parse, Storage, request};

const METRIC_INSERT:         &'static str = "insert::item::complete";
const METRIC_INSERT_ENTRIES: &'static str = "insert::item::entries";

#[async_trait]
impl Insert<InsertItemReq> for ItemCache {
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertItemReq) -> Self::Response {
        let timer = Instant::now();
        let mut map = HashMap::new();
        let mut data = input.0;

        while let Some(x) = data.pop() {
            map
                .entry(x.item_id)
                .and_modify(|entry| {
                    if *entry != x {
                        *entry = x.clone();
                    }
                })
                .or_insert(x);
        }

        self.metrix.send_len(METRIC_INSERT_ENTRIES, map.len()).await;
        *self.cache.write().await = map;
        self.save_to_file().await.unwrap();

        self.metrix.send_time(METRIC_INSERT, timer).await;
        EmptyMsg::default()
    }
}

#[request(Actions::InsertItems)]
#[derive(Debug, Parse)]
pub struct InsertItemReq(pub Vec<ItemEntry>);

