use crate::State;

use tide::{Body, Request, Result};

pub async fn resolve(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>().unwrap())?;
    let results = req.state().resolve_service.resolve(id).await.unwrap();
    Ok(Body::from_json(&results).unwrap())
}

pub async fn bulk_resolve(mut req: Request<State>) -> Result<Body> {
    let ids: Vec<u32> = req.body_json().await?;
    let result = req.state().resolve_service.bulk_resolve(ids).await.unwrap();
    Ok(Body::from_json(&result).unwrap())
}
