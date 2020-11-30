mod api;
mod error;
mod reprocessing;
mod services;

use self::services::*;

use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://caph:caph@cygnus.local:5432/caph_eve")
        .await?;

    let blueprint_service = BlueprintService::new(pool.clone());
    let item_service = ItemService::new(pool.clone());
    let market_service = MarketService::new(pool.clone(), item_service.clone());
    let region_service = RegionService::new(pool.clone());
    let resolve_service = ResolveService::new(pool.clone());

    let state = State {
        blueprint_service,
        item_service,
        market_service,
        region_service,
        resolve_service,
    };

    let mut app = tide::with_state(state.clone());
    log::info!("Starting server");

    app.at("/api").nest({
        let mut market = tide::with_state(state.clone());
        market
            .at("/items")
            .get(api::item::fetch_items)
            .post(api::item::bulk_ids)
            .nest({
                let mut server = tide::with_state(state.clone());
                server
                    .at("/reprocessing")
                    .post(api::item::bulk_reprocessing);
                server
                    .at("/search")
                    .get(api::item::search)
                    .post(api::item::bulk_search);
                server
                    .at("/:id")
                    .get(api::item::fetch_item)
                    .at("/reprocessing")
                    .get(api::item::reprocessing);
                server
            });
        market
            .at("/resolve")
            .post(api::resolve::bulk_resolve)
            .nest({
                let mut server = tide::with_state(state.clone());
                server.at("/:id").get(api::resolve::resolve);
                server
            });
        market.at("/market").post(api::market::fetch);
        market.at("/regions").get(api::region::fetch_regions);
        market.at("/blueprints").nest({
            let mut server = tide::with_state(state.clone());
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
    pub resolve_service: ResolveService,
}
