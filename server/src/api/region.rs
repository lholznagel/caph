use crate::services::RegionService;

use serde::Deserialize;
use warp::{filters::BoxedFilter, get, path, Filter, Rejection, Reply};

#[derive(Deserialize)]
struct RouteQueryParams {
    origin: u32,
    destination: u32,
}

pub fn filter(service: RegionService, root: BoxedFilter<()>) -> BoxedFilter<(impl Reply,)> {
    let root = root
        .and(path!("regions" / ..))
        .and(with_service(service.clone()));

    let regions = root
        .clone()
        .and(warp::path::end())
        .and(get())
        .and_then(regions);

    let route = root
        .clone()
        .and(path!("route"))
        .and(warp::query::<RouteQueryParams>())
        .and(get())
        .and_then(route);

    regions.or(route).boxed()
}

fn with_service(service: RegionService) -> BoxedFilter<(RegionService,)> {
    warp::any().map(move || service.clone()).boxed()
}

async fn regions(service: RegionService) -> Result<impl Reply, Rejection> {
    service
        .all()
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn route(service: RegionService, query: RouteQueryParams) -> Result<impl Reply, Rejection> {
    service
        .route(query.origin, query.destination)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}
