use crate::State;

use serde::Deserialize;
use tide::{Body, Request, Result};

pub async fn fetch_regions(req: Request<State>) -> Result<Body> {
    let results = req.state().region_service.all().await.unwrap();
    Ok(Body::from_json(&results).unwrap())
}

pub async fn route(req: Request<State>) -> Result<Body> {
    #[derive(Deserialize)]
    struct QueryParams {
        origin: u32,
        destination: u32,
    }

    let query: QueryParams = req.query().unwrap();
    let results = req
        .state()
        .region_service
        .route(query.origin, query.destination)
        .await
        .unwrap();
    Ok(Body::from_json(&results).unwrap())
}
