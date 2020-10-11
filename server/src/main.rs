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
    let market_cache = MarketCache::new(metrics.clone());
    market_cache.refresh().await;
    let market_cache = Arc::new(market_cache);

    let sde_cache = SdeCache::new(metrics.clone());
    sde_cache.refresh().await;
    let sde_cache = Arc::new(sde_cache);
    log::info!("Done preparing caches");

    let market_cache_copy = market_cache.clone();
    let sde_cache_copy = sde_cache.clone();
    async_std::task::spawn(async {
        let market_cache = market_cache_copy;
        let sde_cache = sde_cache_copy;

        loop {
            future::ready(1u32)
                .delay(Duration::from_secs(60 * 15))
                .await;

            log::info!("Updating caches");
            sde_cache.refresh().await;
            market_cache.refresh().await;
            log::info!("Updated caches");
        }
    });

    let state = State {
        sde_cache,
        market_cache,
    };

    let mut app = tide::with_state(state);
    log::info!("Starting server");
    app.at("/market/raw").get(api::market::fetch_raw);
    app.at("/sde/raw").get(api::sde::fetch_raw);
    app.listen("0.0.0.0:9000").await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub market_cache: Arc<MarketCache>,
    pub sde_cache: Arc<SdeCache>,
}
