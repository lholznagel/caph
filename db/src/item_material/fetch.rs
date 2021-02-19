use crate::{Actions, ItemMaterialCache, ItemMaterialEntry};

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchItemMaterialReq> for ItemMaterialCache {
    type Error    = EmptyResponse;
    type Response = FetchItemMaterialRes;

    async fn fetch(&self, input: FetchItemMaterialReq) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            return Ok(FetchItemMaterialRes(x.clone()))
        } else {
            return Err(EmptyResponse::default())
        }
    }
}

#[request(Actions::FetchItemMaterial)]
#[derive(Debug, Parse)]
pub struct FetchItemMaterialReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchItemMaterialRes(pub Vec<ItemMaterialEntry>);
