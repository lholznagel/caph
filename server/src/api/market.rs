use crate::cache::MarketFilter;
use crate::State;

use tide::{Body, Request, Result};

pub async fn fetch(mut req: Request<State>) -> Result<Body> {
    let filter: MarketFilter = req.body_json().await?;
    let results = req.state().cache.fetch_market(filter).await;
    Ok(Body::from_json(&results).unwrap())
}

pub async fn count(req: Request<State>) -> Result<Body> {
    let count = req.state().cache.count_market_entries().await;

    #[derive(serde::Serialize)]
    struct CountResult {
        count: usize
    }

    Ok(Body::from_json(&CountResult { count }).unwrap())
}