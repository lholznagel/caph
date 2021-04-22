use std::time::Instant;

use crate::{Actions, IdNameCache, IdNameEntry};

use async_trait::*;
use cachem::{Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::id_name_bulk::complete";

#[async_trait]
impl Fetch<FetchIdNameBulkReq> for IdNameCache {
    type Response = FetchIdNameBulkRes;

    async fn fetch(&self, input: FetchIdNameBulkReq) -> Self::Response {
        let timer = Instant::now();
        let cache_copy = self.cache.read().await;

        let mut res = Vec::with_capacity(input.0.len());
        for x in input.0 {
            if let Some(y) = cache_copy.get(&x) {
                res.push(y.clone())
            }
        }

        self.metrix.send_time(METRIC_FETCH, timer).await;
        FetchIdNameBulkRes(res)
    }
}

#[request(Actions::FetchIdNameBulk)]
#[derive(Debug, Parse)]
pub struct FetchIdNameBulkReq(pub Vec<u32>);

#[derive(Debug, Parse)]
pub struct FetchIdNameBulkRes(pub Vec<IdNameEntry>);
