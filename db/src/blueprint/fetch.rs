use std::time::Instant;

use crate::{Actions, BlueprintCache, BlueprintEntry};

use async_trait::*;
use cachem::{Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::blueprint::complete";

#[async_trait]
impl Fetch<FetchBlueprintReq> for BlueprintCache {
    type Response = FetchBlueprintRes;

    async fn fetch(&self, _: FetchBlueprintReq) -> Self::Response {
        let timer = Instant::now();
        let res = self
            .cache
            .read()
            .await
            .clone()
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();

        self.metrix.send_time(METRIC_FETCH, timer).await;
        FetchBlueprintRes(res)
    }
}

#[request(Actions::FetchBlueprint)]
#[derive(Debug, Default, Parse)]
pub struct FetchBlueprintReq;

#[derive(Debug, Parse)]
pub struct FetchBlueprintRes(pub Vec<BlueprintEntry>);
