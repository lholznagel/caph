use crate::metrics::Metrics;

use async_std::sync::Mutex;
use eve_online_api::{GroupId, Type, TypeId};
use eve_sde_parser::{ParseRequest, ParseResult};
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;

pub struct SdeCache {
    checksum: Mutex<Vec<u8>>,
    items: Mutex<Vec<Type>>,
    metrics: Option<Metrics>,
}

impl SdeCache {
    pub fn new(metrics: Option<Metrics>) -> Self {
        Self {
            checksum: Mutex::new(Vec::with_capacity(32)),
            items: Default::default(),
            metrics,
        }
    }

    pub async fn fetch(&self) -> Vec<Type> {
        self.items.lock().await.clone()
    }

    pub async fn refresh(&self) {
        let start = Instant::now();

        log::debug!("Fetching checksum");
        let fetched_checksum = surf::get(
            "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/checksum",
        )
        .await
        .unwrap()
        .body_bytes()
        .await
        .unwrap()
        .to_vec();
        log::debug!("Fetched checksum");

        // checks if the fetched checksum equals the stored checksum
        if fetched_checksum == self.checksum.lock().await.clone() {
            // early return if both checksums are the same
            return;
        }

        log::debug!("Fetching sde zip");
        let result = surf::get(
            "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip",
        )
        .await
        .unwrap()
        .body_bytes()
        .await
        .unwrap()
        .to_vec();
        log::debug!("Fetched sde zip");

        log::debug!("Parsing sde zip");
        let parse_requests = vec![ParseRequest::TypeIds];

        let result = eve_sde_parser::from_reader(&mut Cursor::new(result), parse_requests).unwrap();
        let items = match result.get(0).unwrap() {
            ParseResult::TypeIds(x) => x.clone(),
            _ => HashMap::new()
        };
        log::debug!("Parsed sde zip");

        let request_time = start.elapsed().as_millis();
        if let Some(x) = self.metrics.as_ref() {
            x.put_sde_metris(items.len(), request_time).await;
        }

        *self.checksum.lock().await = fetched_checksum;

        let mut transformed_items = Vec::with_capacity(items.len());
        for (k, v) in items {
            transformed_items.push(Type {
                description: v
                    .description
                    .map(|x| x.get("en".into()).unwrap_or(&String::new()).clone())
                    .unwrap_or_default()
                    .clone(),
                group_id: GroupId(v.group_id),
                name: v.name.get("en".into()).unwrap().clone(),
                published: v.published,
                type_id: TypeId(k),
                mass: v.mass,
                volume: v.volume,
                ..Default::default()
            })
        }
        *self.items.lock().await = transformed_items;
    }
}
