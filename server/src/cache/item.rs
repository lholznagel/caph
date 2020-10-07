use crate::metrics::Metrics;

use async_std::sync::Mutex;
use eve_online_api::Type;
use std::io::Cursor;
use std::time::Instant;

pub struct ItemCache {
    checksum: Mutex<Vec<u8>>,
    items: Mutex<Vec<Type>>,
    metrics: Option<Metrics>,
}

impl ItemCache {
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
        let result = eve_sde_parser::from_reader(&mut Cursor::new(result)).unwrap();
        let items = result.items();
        log::debug!("Parsed sde zip");

        let request_time = start.elapsed().as_millis();
        if let Some(x) = self.metrics.as_ref() {
            x.put_sde_metris(items.len(), request_time).await;
        }

        *self.checksum.lock().await = fetched_checksum;
        *self.items.lock().await = items;
    }
}
