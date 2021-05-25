use std::collections::HashMap;

use crate::{Actions, BlueprintCache, BlueprintEntry, PlanetSchematicEntry};

use async_trait::async_trait;
use cachem::{EmptyMsg, Insert, Parse, Storage, request};

#[async_trait]
impl Insert<InsertBlueprintReq> for BlueprintCache {
    type Response = EmptyMsg;

    async fn insert(&self, input: InsertBlueprintReq) -> Self::Response {
        let mut blueprint_data = input.blueprints;
        let mut schematic_data = input.schematics;

        let mut blueprints = HashMap::new();
        let mut schematics = HashMap::new();

        while let Some(x) = blueprint_data.pop() {
            blueprints
                .entry(x.bid)
                .and_modify(|entry| {
                    if *entry != x {
                        *entry = x.clone();
                    }
                })
                .or_insert(x);
        }

        while let Some(x) = schematic_data.pop() {
            schematics
                .entry(x.psid)
                .and_modify(|entry| {
                    if *entry != x {
                        *entry = x.clone();
                    }
                })
                .or_insert(x);
        }

        *self.blueprints.write().await = blueprints;
        *self.schematics.write().await = schematics;
        self.save_to_file().await.unwrap();

        EmptyMsg::default()
    }
}

#[request(Actions::InsertBlueprints)]
#[derive(Debug, Parse)]
pub struct InsertBlueprintReq {
    pub blueprints: Vec<BlueprintEntry>,
    pub schematics: Vec<PlanetSchematicEntry>,
}
