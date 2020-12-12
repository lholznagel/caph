use crate::services::ResolveService;

use warp::{Filter, Rejection, Reply, filters::BoxedFilter, get, path};

pub fn filter(service: ResolveService, root: BoxedFilter<()>) -> BoxedFilter<(impl Reply,)> {
    let root = root
        .and(path!("resolve" / ..))
        .and(with_service(service.clone()));

    let resolve= root
        .clone()
        .and(path!(u32))
        .and(get())
        .and_then(resolve);

    resolve.boxed()
}

fn with_service(service: ResolveService) -> BoxedFilter<(ResolveService,)> {
    warp::any().map(move || service.clone()).boxed()
}

async fn resolve(service: ResolveService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .resolve(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}