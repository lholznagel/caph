use crate::auth_user::AuthUser;
use crate::character::CharacterService;
use crate::error::ServerError;

use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get};

pub fn router() -> Router {
    Router::new()
        .route("/alts", get(alts))
        .route("/ids", get(ids))
        .route("/:id", delete(remove))
        .route("/:id/name", get(name))
        .route("/:id/refresh", get(refresh))
}

async fn alts(
    character_service: Extension<CharacterService>,
    user:              AuthUser
) -> Result<impl IntoResponse, ServerError> {
    let client = user.eve_auth_client().await?;
    let cid = user.character_id().await?;
    let alts = character_service.alts(client, cid).await?;
    Ok((StatusCode::OK, Json(alts)))
}

async fn ids(
    character_service: Extension<CharacterService>,
    user:              AuthUser
) -> Result<impl IntoResponse, ServerError> {
    let cid = user.character_id().await?;
    let ids = character_service.ids(cid).await?;
    Ok((StatusCode::OK, Json(ids)))
}

async fn remove(
    character_service:  Extension<CharacterService>,
    Path(character_id): Path<i32>
) -> Result<impl IntoResponse, ServerError> {
    let character_id = character_id.into();
    character_service.remove(character_id).await?;
    Ok((StatusCode::OK, Json("")))
}

async fn name(
    character_service:  Extension<CharacterService>,
    Path(character_id): Path<i32>
) -> Result<impl IntoResponse, ServerError> {
    let character_id = character_id.into();
    let name = character_service.by_id(character_id).await?.character;
    Ok((StatusCode::OK, Json(name)))
}

async fn refresh(
    character_service:  Extension<CharacterService>,
    Path(character_id): Path<i32>
) -> Result<impl IntoResponse, ServerError> {
    let character_id = character_id.into();
    character_service.refresh(character_id).await?;
    Ok((StatusCode::OK, Json(())))
}
