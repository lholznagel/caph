use crate::{AuthService, AuthUser};
use crate::error::ServerError;

use axum::Router;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use serde::Deserialize;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/login/alt", get(login_alt))
}

/// Performs the login of a new user.
async fn login(
    auth_service: AuthService
) -> Result<impl IntoResponse, ServerError> {
    let url = auth_service.login().await?;
    Ok(Redirect::temporary(url))
}

/// Performs a login, but for an toon of a already logged in character.
/// The already logged in character is the owner of all his toons.
async fn login_alt(
    auth_service: AuthService,
    user:         AuthUser,
) -> Result<impl IntoResponse, ServerError> {
    let cid = user.character_id().await?;
    let url = auth_service.login_alt(cid).await?;
    Ok(Redirect::temporary(url))
}

/// Login query that is send by the eve auth servers
#[derive(Debug, Deserialize)]
pub struct EveAuthQuery {
    pub code:  String,
    pub state: String,
}
