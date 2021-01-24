use crate::services::ResolveService;

use warp::{filters::BoxedFilter, get, path, post, Filter, Rejection, Reply};

pub fn filter(service: ResolveService, root: BoxedFilter<()>) -> BoxedFilter<(impl Reply,)> {
    let root = root
        .and(path!("resolve" / ..))
        .and(with_service(service.clone()));

    let resolve_to_name = root
        .clone()
        .and(path!(u32))
        .and(get())
        .and_then(resolve_to_name);

    let bulk_resolve_to_name = root
        .clone()
        .and(warp::path::end())
        .and(post())
        .and(warp::body::json())
        .and_then(bulk_resolve_to_name);

    resolve_to_name.or(bulk_resolve_to_name).boxed()
}

fn with_service(service: ResolveService) -> BoxedFilter<(ResolveService,)> {
    warp::any().map(move || service.clone()).boxed()
}

async fn resolve_to_name(service: ResolveService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .resolve_to_name(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn bulk_resolve_to_name(
    service: ResolveService,
    id: Vec<u32>,
) -> Result<impl Reply, Rejection> {
    service
        .bulk_resolve_to_name(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}
