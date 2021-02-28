use std::time::Instant;

use crate::{Actions, IdNameCache, IdNameEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::id_name::complete";

#[async_trait]
impl Fetch<FetchIdNameReq> for IdNameCache {
    type Error    = EmptyMsg;
    type Response = FetchIdNameRes;

    async fn fetch(&self, input: FetchIdNameReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = FetchIdNameRes(x.clone());
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchIdName)]
#[derive(Debug, Parse)]
pub struct FetchIdNameReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchIdNameRes(pub IdNameEntry);
