use std::collections::HashMap;
use std::time::Instant;

use crate::{Actions, ItemMaterialCache, ItemMaterialEntry};

use async_trait::*;
use cachem::{EmptyMsg, Insert, Parse, Storage, request};

const METRIC_INSERT:         &'static str = "insert::item_material::complete";
const METRIC_INSERT_ENTRIES: &'static str = "insert::item_material::entries";

#[async_trait]
impl Insert<InsertItemMaterialReq> for ItemMaterialCache {
    type Error    = EmptyMsg;
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertItemMaterialReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        let mut map = HashMap::new();

        for x in input.0.iter() {
            map
                .entry(x.item_id)
                .and_modify(|entry: &mut Vec<ItemMaterialEntry>| {
                    if !entry.contains(&x) {
                        entry.push(x.clone());
                    }
                })
                .or_insert(vec![*x]);
        }

        self.metrix.send_len(METRIC_INSERT_ENTRIES, map.len()).await;
        *self.cache.write().await = map;
        self.save_to_file().await.unwrap();

        self.metrix.send_time(METRIC_INSERT, timer).await;
        Ok(EmptyMsg::default())
    }
}


#[request(Actions::InsertItemMaterials)]
#[derive(Debug, Parse)]
pub struct InsertItemMaterialReq(pub Vec<ItemMaterialEntry>);

