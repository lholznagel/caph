use crate::{Actions, ItemMaterialCache, ItemMaterialEntry};

use async_trait::*;
use cachem::{EmptyResponse, Insert, Parse, Storage, request};

#[async_trait]
impl Insert<InsertItemMaterialReq> for ItemMaterialCache {
    type Error    = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertItemMaterialReq) -> Result<Self::Response, Self::Error> {
        let mut old_data = { self.0.read().await.clone() };
        let mut changes: usize = 0;

        for x in input.0.iter() {
            old_data
                .entry(x.item_id)
                .and_modify(|entry| {
                    if !entry.contains(&x) {
                        changes += 1;
                        entry.push(x.clone());
                    }
                })
                .or_insert({
                    changes += 1;
                    vec![*x]
                });
        }

        // there where some changes, so we apply those to the main structure
        if changes > 0 {
            *self.0.write().await = old_data;
        }
        self.save_to_file().await.unwrap();
        Ok(EmptyResponse::default())
    }
}


#[request(Actions::InsertItemMaterials)]
#[derive(Debug, Parse)]
pub struct InsertItemMaterialReq(pub Vec<ItemMaterialEntry>);

