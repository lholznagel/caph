use crate::State;

use tide::{Body, Request, Result};

pub async fn fetch_regions(req: Request<State>) -> Result<Body> {
    let results = req.state().cache.fetch_regions().await;
    Ok(Body::from_json(&results).unwrap())
}
