use crate::services::MarketFilter;
use crate::State;

use tide::{Body, Request, Result};

pub async fn fetch(mut req: Request<State>) -> Result<Body> {
    let filter: MarketFilter = req.body_json().await?;

    let results = req.state().market_service.all(filter).await;
    Ok(Body::from_json(&results).unwrap())
}

pub async fn fetch_by_item_id(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>())??;
    let results = req.state().market_service.fetch_by_item_id(id).await;
    Ok(Body::from_json(&results).unwrap())
}

pub async fn buy_stats(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>())??;
    let results = req.state().market_service.stats(id, true).await;
    Ok(Body::from_json(&results).unwrap())
}

pub async fn sell_stats(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>())??;
    let results = req.state().market_service.stats(id, false).await;
    Ok(Body::from_json(&results).unwrap())
}