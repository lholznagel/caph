use crate::services::{MarketFilter, Market};
use crate::State;

use std::cmp::Ordering;
use std::collections::HashMap;
use tide::{Body, Request, Result};

pub async fn fetch(mut req: Request<State>) -> Result<Body> {
    #[derive(Clone, serde::Deserialize)]
    enum Sort {
        #[serde(alias = "asc")]
        #[serde(alias = "ASC")]
        Asc,
        #[serde(alias = "desc")]
        #[serde(alias = "DESC")]
        Desc
    }

    #[derive(serde::Deserialize)]
    struct QueryParams {
        sort_price: Option<Sort>,
        max_items: Option<usize>,
    }

    let filter: MarketFilter = req.body_json().await?;
    let query_params: QueryParams = req.query()?;

    let results = req.state().market_service.all(filter).await;
    let mut grouped: HashMap<u32, Vec<Market>> = HashMap::new();
    for result in results {
        grouped
            .entry(result.type_id as u32)
            .and_modify(|x| x.push(result.clone()))
            .or_insert(vec![result]);
    }

    let mut results: HashMap<u32, Vec<Market>> = HashMap::new();
    for (id, entries) in grouped.iter_mut() {
        if let Some(x) = query_params.sort_price.clone() {
            match x {
                Sort::Asc => entries.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal)),
                Sort::Desc => entries.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal))
            }
        };

        if let Some(x) = query_params.max_items {
            results.insert(*id, entries.clone().into_iter().take(x).collect::<Vec<Market>>());
        }
    }

    Ok(Body::from_json(&results).unwrap())
}
