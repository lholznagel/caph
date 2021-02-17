use crate::{Actions, BlueprintCache, BlueprintEntry};

use async_trait::async_trait;
use cachem::{EmptyResponse, Insert, Parse, request};

#[async_trait]
impl Insert<InsertBlueprintReq> for BlueprintCache {
    type Error    = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertBlueprintReq) -> Result<Self::Response, Self::Error> {
        let mut old_data = { self.0.read().await.clone() };
        let mut data = input.0;
        let mut changes: usize = 0;

        while let Some(x) = data.pop() {
            old_data
                .entry(x.item_id)
                .and_modify(|entry| {
                    if *entry != x {
                        changes += 1;
                        *entry = x.clone();
                    }
                })
                .or_insert({
                    changes += 1;
                    x
                });
        }

        // there where some changes, so we apply those to the main structure
        if changes > 0 {
            *self.0.write().await = old_data;
        }
        Ok(EmptyResponse::default())
    }
}

#[request(Actions::InsertBlueprints)]
#[derive(Debug, Parse)]
pub struct InsertBlueprintReq(pub Vec<BlueprintEntry>);
