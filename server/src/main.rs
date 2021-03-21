mod error;
mod reprocessing;
mod services;

use self::services::*;

use cachem::ConnectionPool;
use metrix_exporter::Metrix;
use serde::Deserialize;
use warp::hyper::StatusCode;
use warp::http::Response;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec!["tracing".into()]);

    let metrix = Metrix::new(env!("CARGO_PKG_NAME").into(), "0.0.0.0:8889").await?;
    let pool = ConnectionPool::new("0.0.0.0:9999".into(), metrix.get_sender(), 10).await?;

    let character_service = CharacterService::new(pool.clone());
    let item_service = ItemService::new(pool.clone());
    let market_service = MarketService::new(pool);

    ApiServer::new(
        character_service,
        item_service,
        market_service,
    )
    .serve()
    .await;

    Ok(())
}

#[derive(Clone)]
pub struct ApiServer{
    character: CharacterService,
    items: ItemService,
    market: MarketService,
}

impl ApiServer {
    pub fn new(
        character: CharacterService,
        items: ItemService,
        market: MarketService,
    ) -> Self {

        Self {
            character,
            items,
            market,
        }
    }

    pub async fn serve(&self) {
        let _self = self.clone();

        let root = warp::any()
            .map(move || _self.clone())
            .and(warp::path!("api" / ..));

        let eve = root
            .clone()
            .and(warp::path!("eve" / ..));
        let eve_auth = eve
            .clone()
            .and(warp::path!("auth"))
            .and(warp::get())
            .and(warp::query())
            .and_then(Self::eve_auth);
        let eve_login = eve
            .clone()
            .and(warp::path!("login"))
            .and(warp::get())
            .and_then(Self::eve_login);
        let eve_whoami = eve
            .clone()
            .and(warp::path!("whoami"))
            .and(warp::get())
            .and(warp::cookie("user"))
            .and_then(Self::eve_whoami);
        let eve = eve_auth
            .or(eve_login)
            .or(eve_whoami);

        let character = root
            .clone()
            .and(warp::path!("character" / ..));
        let character_assets = character
            .clone()
            .and(warp::path!("assets"))
            .and(warp::get())
            .and(warp::cookie("user"))
            .and_then(Self::character_assets);
        let character_blueprints = character
            .clone()
            .and(warp::path!("blueprints"))
            .and(warp::get())
            .and(warp::cookie("user"))
            .and_then(Self::character_blueprints);
        let character_name = character
            .clone()
            .and(warp::path!("name"))
            .and(warp::get())
            .and(warp::cookie("user"))
            .and_then(Self::character_name);
        let character_portrait = character
            .clone()
            .and(warp::path!("portrait"))
            .and(warp::get())
            .and(warp::cookie("user"))
            .and_then(Self::character_portrait);
        let character = character_assets
            .or(character_blueprints)
            .or(character_name)
            .or(character_portrait);

        let item = root
            .clone()
            .and(warp::path!("items" / ..));
        let item_by_id = item
            .clone()
            .and(warp::path!(u32))
            .and(warp::get())
            .and_then(Self::item_by_id);
        let item_bulk = item
            .clone()
            .and(warp::path!("bulk"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::item_bulk);
        let item_resolve = item
            .clone()
            .and(warp::path!("resolve" / u32))
            .and(warp::get())
            .and_then(Self::item_resolve);
        let item_resolve_bulk = item
            .clone()
            .and(warp::path!("resolve" / "bulk"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::item_resolve_bulk);
        let item_blueprint = item
            .clone()
            .and(warp::path!(u32 / "blueprint" / "graph"))
            .and(warp::get())
            .and_then(Self::item_blueprint_graph);
        let item_reprocessing = item
            .clone()
            .and(warp::path!(u32 / "reprocessing"))
            .and(warp::get())
            .and_then(Self::item_reprocessing);
        let item = item_by_id
            .or(item_bulk)
            .or(item_resolve)
            .or(item_resolve_bulk)
            .or(item_blueprint)
            .or(item_reprocessing);

        let market = root
            .clone()
            .and(warp::path!("market" / ..));
        let market_items = market
            .clone()
            .and(warp::path!("items"))
            .and(warp::get())
            .and_then(Self::market_items);
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
        let market = market_items
            .or(market_stats_buy)
            .or(market_stats_sell)
            .or(market_top_order)
            .or(market_history);

        let api = character
            .or(eve)
            .or(item)
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

    async fn item_bulk(
        self: Self,
        item_ids: Vec<u32>,
    ) -> Result<impl Reply, Rejection> {
        self
            .items
            .bulk(item_ids)
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

    async fn item_resolve_bulk(
        self: Self,
        ids: Vec<u32>
    ) -> Result<impl Reply, Rejection> {
        self
            .items
            .resolve_bulk(ids)
            .await
            .map(|x| warp::reply::json(&x))
            .map_err(Into::into)
    }

    async fn item_blueprint_graph(
        self: Self,
        item_id: u32,
    ) -> Result<impl Reply, Rejection> {
        self
            .items
            .blueprint_graph(item_id)
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

    async fn market_items(
        self: Self,
    ) -> Result<impl Reply, Rejection> {
        self
            .market
            .items()
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

    async fn eve_auth(
        self: Self,
        query: EveQuery,
    ) -> Result<impl Reply, Rejection> {
        let user = caph_eve_online_api::retrieve_authorization_token(&query.code).await.unwrap();
        self.character.save_login(user.clone()).await?;

        Ok(Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("location", "https://eve.caph.xyz")
            .header("Set-Cookie", format!("user={}; Path=/", user.user_id))
            .body("")
            .unwrap())
    }

    async fn eve_login(
        self: Self,
    ) -> Result<impl Reply, Rejection> {
        let auth_uri = caph_eve_online_api::eve_auth_uri().unwrap();

        let uri = warp::http::uri::Builder::new()
            .scheme(auth_uri.scheme())
            .authority(auth_uri.host_str().unwrap_or_default())
            .path_and_query(&format!("{}?{}", auth_uri.path(), auth_uri.query().unwrap_or_default()))
            .build()
            .unwrap();

        Ok(warp::redirect::redirect(uri))
    }

    async fn eve_whoami(
        self: Self,
        character_id: u32
    ) -> Result<impl Reply, Rejection> {
        if let None = self.character.lookup(character_id).await? {
            Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Set-Cookie", "user=; expires=Thu, 01 Jan 1970 00:00:00 GMT")
                .body("")
                .unwrap())
        } else {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body("")
                .unwrap())
        }
    }

    async fn character_name(
        self: Self,
        character_id: u32,
    ) -> Result<impl Reply, Rejection> {
        let name = self
            .character
            .name(character_id)
            .await?;

        Ok(warp::reply::json(&name))
    }

    async fn character_portrait(
        self: Self,
        character_id: u32
    ) -> Result<impl Reply, Rejection> {
        let image = self
            .character
            .portrait(character_id)
            .await?;

        Ok(warp::reply::json(&image))
    }

    async fn character_assets(
        self: Self,
        character_id: u32
    ) -> Result<impl Reply, Rejection> {
        let assets = self
            .character
            .assets(character_id)
            .await?;

        Ok(warp::reply::json(&assets))
    }

    async fn character_blueprints(
        self: Self,
        character_id: u32
    ) -> Result<impl Reply, Rejection> {
        let assets = self
            .character
            .blueprints(character_id)
            .await?;

        Ok(warp::reply::json(&assets))
    }

}


#[derive(Deserialize)]
struct MarketQuery {
    buy: bool,
}

#[derive(Debug, Deserialize)]
struct EveQuery {
    code: String,
    state: String,
}
