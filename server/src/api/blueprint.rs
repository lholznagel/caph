use crate::State;

use tide::{Body, Request, Result};

pub async fn blueprint_cost(req: Request<State>) -> Result<Body> {
    let id = req.param("id").map(|x| x.parse::<u32>().unwrap()).unwrap();
    let result = req.state().blueprint_service.calc_bp_cost(id).await;
    Ok(Body::from_json(&result).unwrap())
}
