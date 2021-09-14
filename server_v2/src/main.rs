mod asset;
mod character;
mod error;
mod eve;

use crate::asset::*;
use crate::character::*;
use crate::eve::*;

use axum::{AddExtensionLayer, Json, Router};
use axum::extract::{self, Extension, Query, TypedHeader};
use axum::handler::get;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::response::{IntoResponse, Redirect};
use axum::routing::BoxRoute;
use error::ServerError;
use headers::Cookie;
use sqlx::postgres::PgPoolOptions;
use std::str::FromStr;
use uuid::Uuid;

const POSTGRES_URI: &str = "postgres://postgres:1234567890@localhost:5432/caph";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(POSTGRES_URI)
        .await?;

    let asset_service = AssetService::new(pool.clone());
    let character_service = CharacterService::new(pool.clone());
    let eve_service = EveService::new(pool.clone());

    let app = Router::new()
        .nest("/api", api_routes())
        .layer(AddExtensionLayer::new(asset_service))
        .layer(AddExtensionLayer::new(character_service))
        .layer(AddExtensionLayer::new(eve_service));

    axum::Server::bind(&"0.0.0.0:10101".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/eve/login", get(eve_login))
        .route("/eve/login/alt", get(eve_login_alt))
        .route("/eve/auth", get(eve_auth))
        .route("/whoami", get(whoami))

        .route("/character/alts", get(character_alts))

        .route("/assets", get(assets))
        .route("/assets/blueprints", get(assets_blueprints))
        .boxed()
}

async fn eve_login(
    eve_service: extract::Extension<EveService>
) -> Result<impl IntoResponse, ServerError> {
    let url = eve_service.login().await?;
    Ok(Redirect::temporary(url))
}

async fn eve_login_alt(
    eve_service: extract::Extension<EveService>,
    TypedHeader(cookie): TypedHeader<Cookie>
) -> Result<impl IntoResponse, ServerError> {
    let token = cookie
        .get("token")
        .ok_or(ServerError::InvalidUser)?;
    let token = Uuid::from_str(token).unwrap();
    let url = eve_service.login_alt(token).await?;
    Ok(Redirect::temporary(url))
}

async fn eve_auth(
    eve_service: Extension<EveService>,
    query:       Query<EveAuthQuery>
) -> Result<impl IntoResponse, ServerError> {
    let token = eve_service
        .auth(&query.code, query.state)
        .await?;

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, "https://eve.caph.xyz".parse().unwrap());

    if let Some(x) = token {
        let cookie = format!(
            "token={}; Path=/; Secure; HttpOnly; Max-Age={}",
            x, 31557800 // 10 years
        );

        headers.insert(SET_COOKIE, cookie.parse().unwrap());
    }
    Ok((StatusCode::MOVED_PERMANENTLY, headers))
}

async fn whoami(
    character_service: Extension<CharacterService>,
    character: LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let character_id = character.character_id().await?;
    let whoami = character_service.info(character_id, None).await?;
    Ok((StatusCode::OK, Json(whoami)))
}

async fn character_alts(
    character_service: Extension<CharacterService>,
    character: LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let character_id = character.character_id().await?;
    let alts = character_service.alts(character_id).await?;
    Ok((StatusCode::OK, Json(alts)))
}

async fn assets(
    asset_service: Extension<AssetService>,
    character: LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let cids = character.character_alts().await?;
    let alts = asset_service.assets(cids).await?;
    Ok((StatusCode::OK, Json(alts)))
}

async fn assets_blueprints(
    asset_service: Extension<AssetService>,
    character: LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let cids = character.character_alts().await?;
    let alts = asset_service.blueprints(cids).await?;
    Ok((StatusCode::OK, Json(alts)))
}


