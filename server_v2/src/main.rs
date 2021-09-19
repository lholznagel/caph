mod asset;
mod character;
mod error;
mod eve;

use crate::asset::*;
use crate::character::*;
use crate::eve::*;

use axum::{AddExtensionLayer, Json, Router};
use axum::extract::{self, Extension, Path, Query, TypedHeader};
use axum::handler::get;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::response::{IntoResponse, Redirect};
use error::ServerError;
use headers::Cookie;
use sqlx::postgres::PgPoolOptions;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::str::FromStr;
use uuid::Uuid;

/// ENV variable for the address the server should bind to
const BIND: &'static str         = "BIND";
/// ENV variable for the database URL
const PG_ADDR: &'static str      = "DATABASE_URL";
/// Url to redirect after login
const ENV_REDIRECT: &'static str = "REDIRECT";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let pg_addr = std::env::var(PG_ADDR)
        .expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&pg_addr)
        .await?;

    let asset_service = AssetService::new(pool.clone());
    let character_service = CharacterService::new(pool.clone());
    let eve_service = EveService::new(pool.clone());

    let app = Router::new()
        .nest("/api", Router::new()
            .route("/eve/login", get(eve_login)).boxed()
            .route("/eve/login/alt", get(eve_login_alt)).boxed()
            .route("/eve/auth", get(eve_auth)).boxed()
            .route("/whoami", get(whoami)).boxed()

            .route("/character/alts", get(character_alts)).boxed()
            .route("/character/ids", get(character_ids)).boxed()
            .route("/character/:id/name", get(character_name)).boxed()

            .route("/assets", get(assets)).boxed()
            .route("/assets/blueprints", get(assets_blueprints)).boxed()
        )
        .layer(AddExtensionLayer::new(asset_service))
        .layer(AddExtensionLayer::new(character_service))
        .layer(AddExtensionLayer::new(eve_service));

    let bind = std::env::var(BIND)
        .unwrap_or(String::from("0.0.0.0:8080"));
    axum::Server::bind(&bind.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn eve_login(
    Extension(eve_service): Extension<EveService>
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

    let redirect = std::env::var(ENV_REDIRECT)
        .unwrap_or(String::from("http://localhost:1337"));

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, redirect.parse().unwrap());

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
    character:         LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let client = character.eve_auth_client().await?;
    let cid = character.character_id().await?;
    let whoami = character_service.info(client, cid, None).await?;
    Ok((StatusCode::OK, Json(whoami)))
}

async fn character_alts(
    character_service: Extension<CharacterService>,
    character:         LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let client = character.eve_auth_client().await?;
    let cid = character.character_id().await?;
    let alts = character_service.alts(client, cid).await?;
    Ok((StatusCode::OK, Json(alts)))
}

async fn character_ids(
    character_service: Extension<CharacterService>,
    character:         LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let ids = character_service.ids(cid).await?;
    Ok((StatusCode::OK, Json(ids)))
}

async fn character_name(
    character_service:  Extension<CharacterService>,
    Path(character_id): Path<i32>
) -> Result<impl IntoResponse, ServerError> {
    let character_id = character_id.into();
    let name = character_service.by_id(character_id).await?.character;
    Ok((StatusCode::OK, Json(name)))
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


