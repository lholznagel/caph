use std::time::Instant;

use crate::{Actions, UserCache, UserEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::user::complete";

#[async_trait]
impl Fetch<FetchUserReq> for UserCache {
    type Response = FetchUserRes;

    async fn fetch(&self, input: FetchUserReq) -> Self::Response {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = x.clone();
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchUserRes::Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchUserRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchUser)]
#[derive(Debug, Parse)]
pub struct FetchUserReq(pub u32);

#[derive(Debug, Parse)]
pub enum FetchUserRes {
    Ok(UserEntry),
    Err(EmptyMsg),
}
