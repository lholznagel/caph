use crate::services::ItemService;

use serde::Deserialize;
use std::collections::HashMap;
use warp::{filters::BoxedFilter, get, path, post, Filter, Rejection, Reply};

pub fn filter(service: ItemService, root: BoxedFilter<()>) -> BoxedFilter<(impl Reply,)> {
    let root = root
        .and(path!("items" / ..))
        .and(with_service(service.clone()));

    let fetch_all = root
        .clone()
        .and(warp::path::end())
        .and(get())
        .and_then(fetch_all);

    let fetch_by_id = root
        .clone()
        .and(path!(u32))
        .and(get())
        .and_then(fetch_by_id);

    let reprocessing = root
        .clone()
        .and(path!(u32 / "reprocessing"))
        .and(get())
        .and_then(reprocessing);

    let fetch_my_items = root
        .clone()
        .and(path!("my"))
        .and(get())
        .and_then(fetch_my_items);

    let fetch_my_item = root
        .clone()
        .and(path!("my" / u32))
        .and(get())
        .and_then(fetch_my_item);

    let push_my_items = root
        .clone()
        .and(path!("my"))
        .and(post())
        .and(warp::body::json())
        .and_then(push_my_items);

    let search = root
        .clone()
        .and(path!("search"))
        .and(get())
        .and(warp::query::<SearchQueryParams>())
        .and_then(search);

    fetch_all
        .or(fetch_by_id)
        .or(reprocessing)
        .or(fetch_my_items)
        .or(fetch_my_item)
        .or(push_my_items)
        .or(search)
        .boxed()
}

fn with_service(service: ItemService) -> BoxedFilter<(ItemService,)> {
    warp::any().map(move || service.clone()).boxed()
}

async fn fetch_all(service: ItemService) -> Result<impl Reply, Rejection> {
    service
        .all()
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn fetch_by_id(service: ItemService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .by_id(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn reprocessing(service: ItemService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .reprocessing(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn fetch_my_items(service: ItemService) -> Result<impl Reply, Rejection> {
    service
        .fetch_my_items()
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn fetch_my_item(service: ItemService, id: u32) -> Result<impl Reply, Rejection> {
    service
        .fetch_my_item(id)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn push_my_items(
    service: ItemService,
    items: HashMap<u32, u64>,
) -> Result<impl Reply, Rejection> {
    service
        .push_my_items(items)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

async fn search(
    service: ItemService,
    query: SearchQueryParams
) -> Result<impl Reply, Rejection> {
    service
        .search(query.exact.unwrap_or_default(), &query.name)
        .await
        .map(|x| warp::reply::json(&x))
        .map_err(Rejection::from)
}

#[derive(Deserialize)]
struct SearchQueryParams {
    name: String,
    exact: Option<bool>,
}