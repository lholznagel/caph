use std::collections::HashMap;
use std::time::Instant;

use crate::{Actions, UserCache, UserEntry};

use async_trait::*;
use cachem::{EmptyMsg, Insert, Parse, Storage, request};

const METRIC_INSERT:         &'static str = "insert::user::complete";
const METRIC_INSERT_ENTRIES: &'static str = "insert::user::entries";

#[async_trait]
impl Insert<InsertUserReq> for UserCache {
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertUserReq) -> Self::Response {
        let timer = Instant::now();
        let mut map = HashMap::new();

        map
            .entry(input.0.user_id)
            .and_modify(|entry| *entry = input.0.clone())
            .or_insert(input.0);

        self.metrix.send_len(METRIC_INSERT_ENTRIES, map.len()).await;
        *self.cache.write().await = map;
        self.save_to_file().await.unwrap();

        self.metrix.send_time(METRIC_INSERT, timer).await;
        EmptyMsg::default()
    }
}

#[request(Actions::InsertUser)]
#[derive(Debug, Parse)]
pub struct InsertUserReq(pub UserEntry);

