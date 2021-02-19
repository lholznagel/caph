use crate::{Actions, IdNameCache, IdNameEntry};

use async_trait::*;
use cachem::{EmptyResponse, Fetch, Parse, request};

#[async_trait]
impl Fetch<FetchIdNameReq> for IdNameCache {
    type Error    = EmptyResponse;
    type Response = FetchIdNameRes;

    async fn fetch(&self, input: FetchIdNameReq) -> Result<Self::Response, Self::Error> {
        if let Some(x) = self.0.read().await.get(&input.0) {
            return Ok(FetchIdNameRes(x.clone()))
        } else {
            return Err(EmptyResponse::default())
        }
    }
}

#[request(Actions::FetchIdName)]
#[derive(Debug, Parse)]
pub struct FetchIdNameReq(pub u32);

#[derive(Debug, Parse)]
pub struct FetchIdNameRes(pub IdNameEntry);
