use crate::metrics::Metrics;

use eve_sde_parser::{ParseRequest, ParseResult, UniqueName};
use serde::Serialize;
use std::io::Cursor;

const SDE_CHECKSUM_URL: &'static str =
    "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/checksum";
const SDE_ZIP_URL: &'static str =
    "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip";

pub enum SdeCacheResult {
    ItemInfos(Vec<ItemCacheEntry>),
    Regions(Vec<RegionCacheEntry>),
    UniqueNames(Vec<NameCacheEntry>)
}

pub struct SdeCache;

impl SdeCache {
    pub async fn refresh(
        checksum: Vec<u8>,
        _metrics: Option<Metrics>,
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
            ParseRequest::TypeIds,
            ParseRequest::UniqueNames,
            ParseRequest::Region,
        ];

        let mut unique_names: Vec<NameCacheEntry> = Vec::new();
        let mut regions = Vec::new();

        let mut results = Vec::new();
        let parse_results =
            eve_sde_parser::from_reader(&mut Cursor::new(zip), parse_requests).unwrap();
        for parse_result in parse_results {
            match parse_result {
                ParseResult::TypeIds(x) => {
                    let mut transformed_items = Vec::with_capacity(x.len());
                    for (k, v) in x {
                        transformed_items.push(ItemCacheEntry {
                            description: v
                                .description
                                .map(|x| x.get("en".into()).unwrap_or(&String::new()).clone())
                                .unwrap_or_default()
                                .clone(),
                            group_id: v.group_id,
                            name: v.name.get("en".into()).unwrap_or(&String::new()).clone(),
                            id: k,
                            volume: v.volume,
                        })
                    }
                    results.push(SdeCacheResult::ItemInfos(transformed_items));
                }
                ParseResult::Region(x) => {
                    regions.push(RegionCacheEntry {
                        name: unique_names
                            .clone()
                            .into_iter()
                            .find(|y| y.item_id == x.region_id)
                            .map(|y| y.item_name)
                            .unwrap_or_default(),
                        region_id: x.region_id,
                    });
                }
                ParseResult::UniqueNames(x) => {
                    for name in x {
                        unique_names.push(NameCacheEntry::from(name));
                    }
                }
                _ => (),
            }
        }
        results.push(SdeCacheResult::Regions(regions));
        results.push(SdeCacheResult::UniqueNames(unique_names));

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

    pub volume: Option<f32>,
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