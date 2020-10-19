use crate::State;

use tide::{Body, Request, Result};

pub async fn fetch_item(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>().unwrap()).unwrap();
    let results = req.state().cache.fetch_item(id).await;
    Ok(Body::from_json(&results).unwrap())
}

pub async fn fetch_items(req: Request<State>) -> Result<Body> {
    let results = req.state().cache.fetch_items().await;
    Ok(Body::from_json(&results).unwrap())
}