
use crate::{AuthService, AuthUser, CharacterService, Scope, Character};
use crate::error::Error;

use axum::{Router, Json};
use axum::extract::{Extension, Query, Path};
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use headers::HeaderMap;
use reqwest::StatusCode;
use reqwest::header::{LOCATION, SET_COOKIE};
use serde::Deserialize;
use caph_connector::CharacterId;

/// Url to redirect after login
const REDIRECT: &str = "REDIRECT";

pub struct AuthApi;

impl AuthApi {
    pub fn router() -> Router {
        Router::new()
            .route("/callback", get(Self::callback))
            .route("/login", get(Self::login))
            .route("/login/alt", get(Self::login_alt))
            .route("/scopes/available", get(Self::scopes_available))
            // TODO:: remove
            .route("/scope/:cid/:scope", get(Self::add_scope))
            .route("/scopes/:cid/:scope", get(Self::add_scope))
            .route("/whoami", get(Self::whoami))
    }

    /// Route: `/api/auth/callback`
    ///
    /// Called after a character successfully logged in over at the EVE login page
    ///
    /// # Params
    ///
    /// * `auth_service` -> Service for handling authentication
    /// * `query`        -> Query params that come from the EVE servers after login
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
        Extension(auth_service): Extension<AuthService>,
        Query(query):            Query<EveAuthQuery>
    ) -> Result<impl IntoResponse, Error> {
        let token = auth_service
            .auth(&query.code, query.state)
            .await?;

        let redirect = std::env::var(REDIRECT)
            .unwrap_or_else(|_| String::from("http://localhost:1337"));
        let redirect = format!("{}/projects", redirect);

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
        Extension(auth_service): Extension<AuthService>
    ) -> Result<impl IntoResponse, Error> {
        let url = auth_service.login(Scope::Public).await?;
        Ok(Redirect::temporary(&url))
    }

    /// Route: `/api/auth/login/alt`
    ///
    /// Login for an alt character
    ///
    /// # Params
    ///
    /// * `auth_service` -> Service to handle authentication
    /// * `cookie`       -> Cookie of the currently logged in character
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
        Extension(auth_service): Extension<AuthService>,
        user:                    AuthUser
    ) -> Result<impl IntoResponse, Error> {
        let cid = user.character_id().await?;
        let url = auth_service.login_alt(cid, Scope::Public).await?;
        Ok(Redirect::temporary(&url))
    }

    async fn whoami(
        character_service: Extension<CharacterService>,
        user:              AuthUser
    ) -> Result<impl IntoResponse, Error> {
        let cid = user
            .character_id()
            .await?;
        let whoami = character_service
            .fetch_info(cid, None)
            .await?;
        Ok((StatusCode::OK, Json(whoami)))
    }

    async fn add_scope(
        Extension(auth_service): Extension<AuthService>,
        Path((cid, scope)):      Path<(CharacterId, String)>,
    ) -> Result<impl IntoResponse, Error> {
        let url = auth_service
            .add_scope(cid, scope)
            .await?;
        Ok(Redirect::temporary(&url))
    }

    async fn scopes_available(
        Extension(auth_service): Extension<AuthService>,
    ) -> Result<impl IntoResponse, Error> {
        let scopes = auth_service.available_scopes();
        Ok((StatusCode::OK, Json(scopes)))
    }
}

/// Login query that is send by the eve auth servers
#[derive(Debug, Deserialize)]
pub struct EveAuthQuery {
    pub code:  String,
    pub state: String,
}
