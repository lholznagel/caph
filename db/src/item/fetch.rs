use std::time::Instant;

use crate::{Actions, ItemCache, ItemEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::item::complete";

#[async_trait]
impl Fetch<FetchItemReq> for ItemCache {
    type Error    = EmptyMsg;
    type Response = FetchItemRes;

    async fn fetch(&self, input: FetchItemReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = FetchItemRes(x.clone());
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchItem)]
#[derive(Debug, Parse)]
pub struct FetchItemReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchItemRes(pub ItemEntry);
