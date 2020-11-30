use crate::State;

use tide::{Body, Request, Result};

pub async fn fetch_item(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>().unwrap())?;
    let results = req.state().item_service.by_id(id).await.unwrap();
    Ok(Body::from_json(&results).unwrap())
}

pub async fn fetch_items(req: Request<State>) -> Result<Body> {
    let results = req.state().item_service.all().await.unwrap();
    Ok(Body::from_json(&results).unwrap())
}

pub async fn bulk_ids(mut req: Request<State>) -> Result<Body> {
    let ids: Vec<u32> = req.body_json().await?;
    let result = req.state().item_service.bulk_item_by_id(ids).await.unwrap();
    Ok(Body::from_json(&result).unwrap())
}

pub async fn search(req: Request<State>) -> Result<Body> {
    #[derive(Debug, serde::Deserialize)]
    struct QueryParams {
        name: String,
        exact: Option<bool>,
    }
    let query_params = req.query::<QueryParams>()?;
    let result = req
        .state()
        .item_service
        .search(query_params.exact.unwrap_or_default(), &query_params.name)
        .await
        .unwrap();
    Ok(Body::from_json(&result).unwrap())
}

pub async fn bulk_search(mut req: Request<State>) -> Result<Body> {
    #[derive(serde::Deserialize)]
    struct QueryParams {
        exact: bool,
    }

    let query_params = req.query::<QueryParams>()?;
    let names: Vec<String> = req.body_json().await?;

    let result = req
        .state()
        .item_service
        .bulk_search(query_params.exact, names)
        .await
        .unwrap();
    Ok(Body::from_json(&result).unwrap())
}

pub async fn reprocessing(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>().unwrap())?;
    let results = req.state().item_service.reprocessing(id).await.unwrap();
    Ok(Body::from_json(&results).unwrap())
}

pub async fn bulk_reprocessing(mut req: Request<State>) -> Result<Body> {
    let ids: Vec<u32> = req.body_json().await?;
    let result = req
        .state()
        .item_service
        .bulk_reprocessing(ids)
        .await
        .unwrap();
    Ok(Body::from_json(&result).unwrap())
}
