// mod api;
// mod error;
// mod reprocessing;
// mod services;

// use self::services::*;

// use std::net::SocketAddr;
// use cachem::ConnectionPool;
// use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    morgan::Morgan::init(vec!["warp".into(), "tracing".into()]);

    let pool = ConnectionPool::new("127.0.0.1:9999".into(), 10).await?;

    //let blueprint_service = BlueprintService::new(pool.clone());
    //let item_service = ItemService::new(pool.clone());
    let market_service = MarketService::new(pool.clone());
    //let region_service = RegionService::new(pool.clone());
    //let resolve_service = ResolveService::new(pool.clone());

    let root = warp::any().and(warp::path!("api" / "v1" / ..)).boxed();
    let combined = api::market::filter(market_service, root.clone())
        //.or(api::blueprint::filter(blueprint_service, root.clone()))
        //.or(api::item::filter(item_service, root.clone()))
        //.or(api::region::filter(region_service, root.clone()))
        //.or(api::resolve::filter(resolve_service, root.clone()))
        .boxed();
    warp::serve(combined)
        .run(([0.0.0.0], 10101))
        .await;
    */

    Ok(())
}
