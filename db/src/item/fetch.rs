use std::time::Instant;

use crate::{Actions, ItemCache, ItemEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::item::complete";

#[async_trait]
impl Fetch<FetchItemReq> for ItemCache {
    type Response = FetchItemRes;

    async fn fetch(&self, input: FetchItemReq) -> Self::Response {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = x.clone();
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchItemRes::Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchItemRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchItem)]
#[derive(Debug, Parse)]
pub struct FetchItemReq(pub u32);

#[derive(Debug, Parse)]
pub enum FetchItemRes {
    Ok(ItemEntry),
    Err(EmptyMsg),
}
