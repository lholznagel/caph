use crate::character::CharacterService;
use crate::error::ServerError;
use crate::eve::{EveAuthQuery, EveService, LoggedInCharacter};

use axum::{Json, Router};
use axum::extract::{Extension, Query, TypedHeader};
use axum::handler::get;
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::BoxRoute;
use headers::{Cookie, HeaderMap};
use std::str::FromStr;
use uuid::Uuid;

/// Url to redirect after login
const REDIRECT: &str         = "REDIRECT";

/// Returns the router so that it can be used for a subroute
///
/// # Returns
///
/// New router
///
pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/callback", get(callback)).boxed()
        .route("/login", get(login)).boxed()
        .route("/login/alt", get(login_alt)).boxed()
        .route("/whoami", get(whoami)).boxed()
}

/// Route: `/api/auth/callback`
///
/// Called after a character successfully logged in over at the EVE login page
///
/// # Params
///
/// * `eve_service` -> Service for handling EVE authentication stuff
/// * `query`       -> Query params that come from the EVE servers after login
///
/// # Fails
///
/// Fails if the new user cannot be saved in the database
///
/// # Returns
///
/// Cookie containing a unique id of the logged in character and a redirect
/// to the main page of the webside
///
async fn callback(
    eve_service: Extension<EveService>,
    query:       Query<EveAuthQuery>
) -> Result<impl IntoResponse, ServerError> {
    let token = eve_service
        .auth(&query.code, query.state)
        .await?;

    let redirect = std::env::var(REDIRECT)
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

/// Route: `/api/auth/login`
///
/// Login for a main account
///
/// # Params
///
/// * `eve_service` -> Service to handle eve authentication stuff
///
/// # Errors
///
/// Fails if a database operation is not successfull
///
/// # Returns
///
/// Redirect to the EVE login page
///
async fn login(
    Extension(eve_service): Extension<EveService>
) -> Result<impl IntoResponse, ServerError> {
    let url = eve_service.login().await?;
    Ok(Redirect::temporary(url))
}

/// Route: `/api/auth/login/alt`
///
/// Login for an alt character
///
/// # Params
///
/// * `eve_service` -> Service to handle eve authentication stuff
/// * `cookie`      -> Cookie of the currently logged in character
///
/// # Errors
///
/// Fails if the cookie is not in the database and any database operation for
/// login an alt fails
///
/// # Returns
///
/// Redirect to the EVE login page
///
async fn login_alt(
    Extension(eve_service): Extension<EveService>,
    TypedHeader(cookie):    TypedHeader<Cookie>
) -> Result<impl IntoResponse, ServerError> {
    let token = cookie
        .get("token")
        .ok_or(ServerError::InvalidUser)?;
    let token = Uuid::from_str(token).unwrap();
    let url = eve_service.login_alt(token).await?;
    Ok(Redirect::temporary(url))
}

async fn whoami(
    character_service: Extension<CharacterService>,
    character:         LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let client = character
        .eve_auth_client()
        .await?;
    let cid = character
        .character_id()
        .await?;
    let whoami = character_service
        .info(client, cid, None)
        .await?;
    Ok((StatusCode::OK, Json(whoami)))
}
