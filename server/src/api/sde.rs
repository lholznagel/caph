use crate::State;

use tide::{Body, Request, Result};

pub async fn fetch_raw(req: Request<State>) -> Result<Body> {
    let data = req.state().sde_cache.fetch().await;

    Ok(Body::from_json(&data).unwrap())
}
