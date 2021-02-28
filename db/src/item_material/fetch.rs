use std::time::Instant;

use crate::{Actions, ItemMaterialCache, ItemMaterialEntry};

use async_trait::*;
use cachem::{EmptyMsg, Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::item_material::complete";

#[async_trait]
impl Fetch<FetchItemMaterialReq> for ItemMaterialCache {
    type Error    = EmptyMsg;
    type Response = FetchItemMaterialRes;

    async fn fetch(&self, input: FetchItemMaterialReq) -> Result<Self::Response, Self::Error> {
        let timer = Instant::now();
        if let Some(x) = self.cache.read().await.get(&input.0) {
            let res = FetchItemMaterialRes(x.clone());
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Ok(res)
        } else {
            self.metrix.send_time(METRIC_FETCH, timer).await;
            Err(EmptyMsg::default())
        }
    }
}

#[request(Actions::FetchItemMaterial)]
#[derive(Debug, Parse)]
pub struct FetchItemMaterialReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchItemMaterialRes(pub Vec<ItemMaterialEntry>);
