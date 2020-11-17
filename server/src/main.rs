mod api;
mod cache;
mod error;
mod services;

use self::services::*;

use async_std::future;
use async_std::prelude::*;
use async_std::sync::Arc;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init().unwrap();

    log::info!("Preparing caches");
    let cache = CacheService::new();
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

    let item_service = ItemService::new(cache.clone());
    let region_service = RegionService::new(cache.clone());
    let market_service = MarketService::new(cache.clone(), item_service.clone());
    let blueprint_service = BlueprintService::new(cache.clone(), market_service.clone());
    let state = State {
        blueprint_service,
        item_service,
        market_service,
        region_service,
    };

    let mut app = tide::with_state(state.clone());
    log::info!("Starting server");

    app.at("/api").nest({
        let mut market = tide::with_state(state.clone());
        market.at("/items").get(api::item::fetch_items).nest({
            let mut server = tide::with_state(state.clone());
            server.at("/bulk").post(api::item::bulk_ids);
            server
                .at("/search")
                .get(api::item::search)
                .post(api::item::bulk_search);
            server.at("/:id").get(api::item::fetch_item);
            server
        });
        market
            .at("/market")
            .post(api::market::fetch)
            .at("/count")
            .get(api::market::count);
        market.at("/regions").get(api::region::fetch_regions);
        market.at("/blueprints").nest({
            let mut server = tide::with_state(state.clone());
            server.at("/good").get(api::blueprint::good);
            server.at("/:id").get(api::blueprint::blueprint_cost);
            server
        });

        market
    });

    app.listen("0.0.0.0:9000").await?;

    Ok(())
}

#[derive(Clone)]
pub struct State {
    pub blueprint_service: BlueprintService,
    pub item_service: ItemService,
    pub market_service: MarketService,
    pub region_service: RegionService,
}
