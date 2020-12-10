use crate::State;

use tide::{Body, Request, Result};

pub async fn resolve(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>().unwrap())?;
    let results = req.state().resolve_service.resolve(id).await.unwrap();
    Ok(Body::from_json(&results).unwrap())
}