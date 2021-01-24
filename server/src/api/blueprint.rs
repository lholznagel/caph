use crate::services::BlueprintService;

use warp::{filters::BoxedFilter, get, path, Filter, Rejection, Reply};

pub fn filter(service: BlueprintService, root: BoxedFilter<()>) -> BoxedFilter<(impl Reply,)> {
    let root = root
        .and(path!("blueprints" / ..))
        .and(with_service(service.clone()));

    let cost = root
        .clone()
        .and(path!(u32))
        .and(get())
        .and_then(cost)
        .boxed();

    cost
}

fn with_service(service: BlueprintService) -> BoxedFilter<(BlueprintService,)> {
    warp::any().map(move || service.clone()).boxed()
}

async fn cost(service: BlueprintService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .calc_bp_cost(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}
