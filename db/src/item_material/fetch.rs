use std::time::Instant;

use crate::{Actions, ItemMaterialCache, ItemMaterialEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::item_material::complete";

#[async_trait]
impl Fetch<FetchItemMaterialReq> for ItemMaterialCache {
    type Response = FetchItemMaterialRes;

    async fn fetch(&self, input: FetchItemMaterialReq) -> Self::Response {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = x.clone();
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchItemMaterialRes::Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            FetchItemMaterialRes::Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchItemMaterial)]
#[derive(Debug, Parse)]
pub struct FetchItemMaterialReq(pub u32);

#[derive(Debug, Parse)]
pub enum FetchItemMaterialRes {
    Ok(Vec<ItemMaterialEntry>),
    Err(EmptyMsg),
}
