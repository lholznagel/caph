use crate::{Actions, ItemCache, ItemEntry};

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchItemReq> for ItemCache {
    type Error    = EmptyResponse;
    type Response = FetchItemRes;

    async fn fetch(&self, input: FetchItemReq) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            return Ok(FetchItemRes(x.clone()))
        } else {
            return Err(EmptyResponse::default())
        }
    }
}

#[request(Actions::FetchItem)]
#[derive(Debug, Parse)]
pub struct FetchItemReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchItemRes(pub ItemEntry);
