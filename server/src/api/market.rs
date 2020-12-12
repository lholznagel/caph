use crate::services::{MarketFilter, MarketService};

use warp::{Filter, Rejection, Reply, filters::BoxedFilter, get, post, path};

pub fn filter(service: MarketService, root: BoxedFilter<()>) -> BoxedFilter<(impl Reply,)> {
    let root = root
        .and(path!("market" / ..))
        .and(with_service(service.clone()));

    let filter_market = root
        .clone()
        .and(warp::path::end())
        .and(post())
        .and(warp::body::json())
        .and_then(filter_market);

    let fetch_by_item_id= root
        .clone()
        .and(path!(u32))
        .and(get())
        .and_then(fetch_by_item_id);

    let buy_stats= root
        .clone()
        .and(path!(u32 / "stats" / "buy"))
        .and(get())
        .and_then(buy_stats);

    let sell_stats= root
        .clone()
        .and(path!(u32 / "stats" / "sell"))
        .and(get())
        .and_then(sell_stats);

    filter_market
        .or(fetch_by_item_id)
        .or(buy_stats)
        .or(sell_stats)
        .boxed()
}

fn with_service(service: MarketService) -> BoxedFilter<(MarketService,)> {
    warp::any().map(move || service.clone()).boxed()
}

async fn filter_market(service: MarketService, filter: MarketFilter) -> Result<impl Reply, Rejection> {
    service
        .all(filter)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn fetch_by_item_id(service: MarketService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .fetch_by_item_id(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn buy_stats(service: MarketService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .stats(id, true)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn sell_stats(service: MarketService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .stats(id, false)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}