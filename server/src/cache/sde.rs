use caph_eve_sde_parser::{Blueprint, BlueprintAdditional, ParseRequest, ParseResult, UniqueName};
use serde::Serialize;
use std::collections::HashMap;
use std::io::Cursor;

const SDE_CHECKSUM_URL: &'static str =
    "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/checksum";
const SDE_ZIP_URL: &'static str =
    "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip";

pub enum SdeCacheResult {
    Blueprints(Vec<BlueprintCacheEntry>),
    Schematics(Vec<SchematicCacheEntry>),
}

pub struct SdeCache;

impl SdeCache {
    pub async fn refresh(
        checksum: Vec<u8>,
    ) -> Option<(Vec<SdeCacheResult>, Vec<u8>)> {
        log::debug!("Fetching checksum");
        let fetched_checksum = Self::fetch_checksum().await;
        log::debug!("Fetched checksum");

        // checks if the fetched checksum equals the stored checksum
        if fetched_checksum == checksum {
            // early return if both checksums are the same
            return None;
        }

        log::debug!("Fetching sde zip");
        let zip = Self::fetch_zip().await;
        log::debug!("Fetched sde zip");

        log::debug!("Parsing sde zip");
        let parse_requests = vec![
            ParseRequest::Blueprints,
            ParseRequest::Schematics,
        ];

        let mut results = Vec::new();
        let parse_results =
            caph_eve_sde_parser::from_reader(&mut Cursor::new(zip), parse_requests).unwrap();
        for parse_result in parse_results {
            match parse_result {
                ParseResult::Schematic(x) => {
                    let mut schematics = Vec::with_capacity(x.len());
                    for (id, x) in x {
                        let mut inputs = HashMap::new();
                        let mut outputs = HashMap::new();

                        for (type_id, y) in x.types {
                            if y.is_input {
                                inputs.insert(type_id, y.quantity);
                            } else {
                                outputs.insert(type_id, y.quantity);
                            }
                        }

                        schematics.push(SchematicCacheEntry {
                            id,
                            inputs,
                            outputs,
                            time: x.cycle_time,
                        });
                    }
                    results.push(SdeCacheResult::Schematics(schematics));
                }
                ParseResult::Blueprints(x) => {
                    let mut blueprints = Vec::new();
                    for (_, blueprint) in x {
                        blueprints.push(BlueprintCacheEntry::from(blueprint));
                    }
                    results.push(SdeCacheResult::Blueprints(blueprints));
                }
                _ => (),
            }
        }

        log::debug!("Parsed sde zip");

        Some((results, fetched_checksum))
    }

    async fn fetch_checksum() -> Vec<u8> {
        surf::get(SDE_CHECKSUM_URL)
            .await
            .unwrap()
            .body_bytes()
            .await
            .unwrap()
            .to_vec()
    }

    async fn fetch_zip() -> Vec<u8> {
        surf::get(SDE_ZIP_URL)
            .await
            .unwrap()
            .body_bytes()
            .await
            .unwrap()
            .to_vec()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemCacheEntry {
    pub description: String,
    pub group_id: u32,
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegionCacheEntry {
    pub name: String,
    pub region_id: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct NameCacheEntry {
    pub group_id: u32,
    pub item_id: u32,
    pub item_name: String
}

impl From<UniqueName> for NameCacheEntry {
    fn from(x: UniqueName) -> Self {
        Self {
            group_id: x.group_id,
            item_id: x.item_id,
            item_name: x.item_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BlueprintCacheEntry {
    pub id: u32,
    pub copying: Option<BlueprintAdditional>,
    pub invention: Option<BlueprintAdditional>,
    pub manufacturing: Option<BlueprintAdditional>,
    pub reaction: Option<BlueprintAdditional>,
    pub research_material: Option<BlueprintAdditional>,
    pub research_time: Option<BlueprintAdditional>,
}

impl From<Blueprint> for BlueprintCacheEntry {
    fn from(x: Blueprint) -> Self {
        Self {
            id: x.blueprint_type_id,
            copying: x.activities.copying,
            invention: x.activities.invention,
            manufacturing: x.activities.manufacturing,
            reaction: x.activities.reaction,
            research_material: x.activities.research_material,
            research_time: x.activities.research_time,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SchematicCacheEntry {
    pub id: u32,
    pub inputs: HashMap<u32, u32>,
    pub outputs: HashMap<u32, u32>,
    pub time: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct SolarsystemCacheEntry {
    pub name: String,
    pub id: u32,
    pub security: f64,
    pub security_class: Option<String>,
}