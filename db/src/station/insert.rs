use crate::{Actions, StationCache, StationEntry};

use async_trait::*;
use cachem::{EmptyResponse, Insert, Parse, Storage, request};

#[async_trait]
impl Insert<InsertStationReq> for StationCache {
    type Error    = EmptyResponse;
    type Response = EmptyResponse;

    async fn insert(&self, input: InsertStationReq) -> Result<Self::Response, Self::Error> {
        let mut old_data = { self.0.read().await.clone() };
        let mut data = input.0;
        let mut changes: usize = 0;

        while let Some(x) = data.pop() {
            old_data
                .entry(x.system_id)
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

        self.save_to_file().await.unwrap();
        Ok(EmptyResponse::default())
    }
}

#[request(Actions::InsertStations)]
#[derive(Debug, Parse)]
pub struct InsertStationReq(pub Vec<StationEntry>);

