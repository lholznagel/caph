use std::time::Instant;

use crate::{Actions, BlueprintCache, BlueprintEntry, PlanetSchematicEntry};

use async_trait::*;
use cachem::{Fetch, Parse, request};

const METRIC_FETCH: &'static str = "fetch::blueprint::complete";

#[async_trait]
impl Fetch<FetchBlueprintReq> for BlueprintCache {
    type Response = FetchBlueprintRes;

    async fn fetch(&self, _: FetchBlueprintReq) -> Self::Response {
        let timer = Instant::now();
        let blueprints = self
            .blueprints
            .read()
            .await
            .iter()
            .map(|(_, x)| x.clone())
            .collect::<Vec<_>>();
        let schematics = self
            .schematics
            .read()
            .await
            .iter()
            .map(|(_, x)| x.clone())
            .collect::<Vec<_>>();

        self.metrix.send_time(METRIC_FETCH, timer).await;
        FetchBlueprintRes {
            blueprints,
            schematics,
        }
    }
}

#[request(Actions::FetchBlueprint)]
#[derive(Debug, Default, Parse)]
pub struct FetchBlueprintReq;

#[derive(Debug, Parse)]
pub struct FetchBlueprintRes {
    pub blueprints: Vec<BlueprintEntry>,
    pub schematics: Vec<PlanetSchematicEntry>,
}
