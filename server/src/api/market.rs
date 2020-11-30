use crate::services::{Market, MarketFilter};
use crate::State;

use tide::{Body, Request, Result};

pub async fn fetch(mut req: Request<State>) -> Result<Body> {
    let filter: MarketFilter = req.body_json().await?;

    let results = req.state().market_service.all(filter).await;
    /*let mut grouped: HashMap<u32, Vec<Market>> = HashMap::new();
    for result in results {
        grouped
            .entry(result.type_id as u32)
            .and_modify(|x| x.push(result.clone()))
            .or_insert(vec![result]);
    }*/

    /*let mut results: HashMap<u32, Vec<Market>> = HashMap::new();
    for (id, entries) in grouped.iter_mut() {
        if let Some(x) = query_params.sort_price.clone() {
            match x {
                Sort::Asc => {
                    entries.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
                }
                Sort::Desc => {
                    entries.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal))
                }
            }
        };

        if let Some(x) = query_params.max_items {
            results.insert(
                *id,
                entries.clone().into_iter().take(x).collect::<Vec<Market>>(),
            );
        } else {
            results.insert(
                *id,
                entries.clone(),
            );
        }
    }*/

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