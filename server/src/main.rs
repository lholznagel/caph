mod api;
mod error;
mod reprocessing;
mod services;

use self::services::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::net::SocketAddr;
use warp::Filter;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec!["sqlx".into()]);

    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect("postgres://caph:caph@cygnus.local:5432/caph_eve")
        .await?;

    // Make sure the database has the newest scripts applied
    let mut conn = pool.acquire().await?;
    conn.execute(include_str!("./tables.sql")).await?;

    let blueprint_service = BlueprintService::new(pool.clone());
    let item_service = ItemService::new(pool.clone());
    let market_service = MarketService::new(pool.clone(), item_service.clone());
    let region_service = RegionService::new(pool.clone());
    let resolve_service = ResolveService::new(pool.clone());

    let root = warp::any().and(warp::path!("v1" / ..)).boxed();
    let combined = api::blueprint::filter(blueprint_service, root.clone())
        .or(api::item::filter(item_service, root.clone()))
        .or(api::market::filter(market_service, root.clone()))
        .or(api::region::filter(region_service, root.clone()))
        .or(api::resolve::filter(resolve_service, root.clone()))
        .boxed();
    warp::serve(combined)
        .run("localhost:10101".parse::<SocketAddr>().unwrap())
        .await;

    Ok(())
}