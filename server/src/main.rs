mod api;
mod cache;
mod error;
mod metrics;

use self::cache::*;

use async_std::future;
use async_std::prelude::*;
use async_std::sync::Arc;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init().unwrap();

    let metrics = std::env::var("ENABLE_METRICS")
        .map(|_| Some(metrics::Metrics::default()))
        .unwrap_or(None);

    log::info!("Preparing caches");
    let cache = Cache::new(metrics.clone());
    cache.refresh().await;
    let cache = Arc::new(cache);
    log::info!("Done preparing caches");

    let cache_copy = cache.clone();
    async_std::task::spawn(async {
        let cache = cache_copy;

        loop {
            future::ready(1u32)
                .delay(Duration::from_secs(60 * 15))
                .await;

            log::info!("Updating caches");
            cache.refresh().await;
            log::info!("Updated caches");
        }
    });

    let state = State { cache };

    let mut app = tide::with_state(state.clone());
    log::info!("Starting server");

    app
        .at("/api")
        .nest({
            let mut market = tide::with_state(state.clone());
            market
                .at("/items")
                .get(api::item::fetch_items)
                .at("/:id")
                .get(api::item::fetch_item);
            market
                .at("/market")
                .post(api::market::fetch)
                .at("/count")
                .get(api::market::count);
            market
                .at("/regions")
                .get(api::region::fetch_regions);
            market
        });

    app.listen("0.0.0.0:9000").await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub cache: Arc<Cache>,
}
