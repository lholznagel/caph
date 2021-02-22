mod error;
mod reprocessing;
mod services;

use self::services::*;

use cachem::ConnectionPool;
use serde::Deserialize;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec!["tracing".into()]);

    let pool = ConnectionPool::new("0.0.0.0:9999".into(), 10).await?;

    let item_service = ItemService::new(pool.clone());
    let market_service = MarketService::new(pool);

    ApiServer::new(
        item_service,
        market_service,
    )
    .serve()
    .await;

    Ok(())
}

#[derive(Clone)]
pub struct ApiServer{
    items: ItemService,
    market: MarketService,
}

impl ApiServer {
    pub fn new(
        items: ItemService,
        market: MarketService,
    ) -> Self {

        Self {
            items,
            market,
        }
    }

    pub async fn serve(&self) {
        let _self = self.clone();

        let root = warp::any()
            .map(move || _self.clone())
            .and(warp::path!("api" / ..));

        let item = root
            .clone()
            .and(warp::path!("items" / ..));
        let item_by_id = item
            .clone()
            .and(warp::path!(u32))
            .and(warp::get())
            .and_then(Self::item_by_id);
        let item_resolve = item
            .clone()
            .and(warp::path!(u32 / "resolve"))
            .and(warp::get())
            .and_then(Self::item_resolve);
        let item_reprocessing = item
            .clone()
            .and(warp::path!(u32 / "reprocessing"))
            .and(warp::get())
            .and_then(Self::item_reprocessing);
        let item = item_by_id
            .or(item_resolve)
            .or(item_reprocessing);

        let market = root
            .clone()
            .and(warp::path!("market" / ..));
        let market_stats_buy = market
            .clone()
            .and(warp::path!(u32 / "stats" / "buy"))
            .and(warp::get())
            .and_then(Self::market_stats_buy);
        let market_stats_sell = market
            .clone()
            .and(warp::path!(u32 / "stats" / "sell"))
            .and(warp::get())
            .and_then(Self::market_stats_sell);
        let market_top_order = market
            .clone()
            .and(warp::path!(u32 / "orders"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::market_top_order);
        let market_history = market
            .clone()
            .and(warp::path!(u32 / "historic"))
            .and(warp::query())
            .and(warp::get())
            .and_then(Self::market_historic);
        let market = market_stats_buy
            .or(market_stats_sell)
            .or(market_top_order)
            .or(market_history);

        let api = item
            .or(market);
        warp::serve(api)
            .run(([0, 0, 0, 0], 10101))
            .await;
    }

    async fn item_by_id(
        self: Self,
        item_id: u32,
    ) -> Result<impl Reply, Rejection> {
        self
            .items
            .by_id(item_id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn item_resolve(
        self: Self,
        item_id: u32,
    ) -> Result<impl Reply, Rejection> {
        self
            .items
            .resolve_id(item_id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn item_reprocessing(
        self: Self,
        item_id: u32,
    ) -> Result<impl Reply, Rejection> {
        self
            .items
            .reprocessing(item_id)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn market_stats_buy(
        self: Self,
        item_id: u32,
    ) -> Result<impl Reply, Rejection> {
        self
            .market
            .stats(item_id, true)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn market_stats_sell(
        self: Self,
        item_id: u32,
    ) -> Result<impl Reply, Rejection> {
        self
            .market
            .stats(item_id, false)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn market_top_order(
        self: Self,
        item_id: u32,
        body: TopOrderReq,
    ) -> Result<impl Reply, Rejection> {
        self
            .market
            .top_orders(item_id, body)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn market_historic(
        self: Self,
        item_id: u32,
        query: MarketQuery,
    ) -> Result<impl Reply, Rejection> {
        self
            .market
            .historic(item_id, query.buy)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }
}

#[derive(Deserialize)]
struct MarketQuery {
    buy: bool,
}
