use crate::{AssetFilter, ResolveIdNameFilter};
use crate::asset::AssetService;
use crate::error::ServerError;
use crate::eve::LoggedInCharacter;

use axum::{Json, Router};
use axum::extract::{Extension, Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use caph_connector::{ItemId, TypeId};

/// Router for handling assets
///
/// # Returns
///
/// Router with all routes for assets
///
pub fn router() -> Router {
    Router::new()
        .route("/", get(assets))
        .route("/all/buildable", get(general_assets_buildable))
        .route("/resolve/id", post(resolve_id_from_name_bulk))
        .route("/:iid", get(asset_by_id))
        .route("/:iid/name", get(asset_name))
        // TODO: those are tid -> export to /item
        .route("/:iid/blueprint/material", get(asset_blueprint_material))
        .route("/:iid/blueprint/flat", get(asset_blueprint_flat))
        .route("/:iid/blueprint/tree", get(asset_blueprint_tree))
        .route("/:iid/blueprint/raw", get(asset_blueprint_raw))
}

/// Gets a single asset by its id
///
/// # Params
///
/// * `asset_service` -> Service for handling assets
/// * `character`     -> A logged in character
/// * `aid`           -> Id of the asset to get
///
/// # Errors
///
/// If the database is not available or if the asset is not found
///
/// # Returns
///
/// The requested item if it exists, if not StatusCode 404
///
async fn asset_by_id(
    asset_service: Extension<AssetService>,
    character:     LoggedInCharacter,
    Path(iid):     Path<ItemId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let entry = asset_service.asset(cid, iid).await?;

    if let Some(entry) = entry {
        Ok((StatusCode::OK, Json(entry)))
    } else {
        Err(ServerError::NotFound)
    }
}

async fn asset_blueprint_material(
    asset_service: Extension<AssetService>,
    Path(tid):     Path<TypeId>
) -> Result<impl IntoResponse, ServerError> {
    let entries = asset_service.blueprint_material(tid).await?;
    Ok((StatusCode::OK, Json(entries)))
}

async fn asset_blueprint_tree(
    asset_service: Extension<AssetService>,
    Path(tid):     Path<TypeId>
) -> Result<impl IntoResponse, ServerError> {
    let tree = asset_service.blueprint_tree(tid).await?;
    Ok((StatusCode::OK, Json(tree)))
}

async fn asset_blueprint_raw(
    asset_service: Extension<AssetService>,
    Path(tid):     Path<TypeId>
) -> Result<impl IntoResponse, ServerError> {
    let tree = asset_service.blueprint_raw(tid).await?;
    Ok((StatusCode::OK, Json(tree)))
}

async fn asset_blueprint_flat(
    asset_service: Extension<AssetService>,
    Path(tid):     Path<TypeId>
) -> Result<impl IntoResponse, ServerError> {
    let flat = asset_service.blueprint_flat(tid).await?;
    Ok((StatusCode::OK, Json(flat)))
}

async fn assets(
    asset_service: Extension<AssetService>,
    character:     LoggedInCharacter,
    Query(filter): Query<AssetFilter>
) -> Result<impl IntoResponse, ServerError> {
    let cids = character.character_alts().await?;
    let alts = asset_service.assets(cids, filter).await?;
    Ok((StatusCode::OK, Json(alts)))
}

async fn asset_name(
    asset_service: Extension<AssetService>,
    character:     LoggedInCharacter,
    Path(iid):     Path<ItemId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let name = asset_service.asset_name(cid, iid).await?;
    Ok((StatusCode::OK, Json(name)))
}

async fn general_assets_buildable(
    asset_service: Extension<AssetService>,
) -> Result<impl IntoResponse, ServerError> {
    let res = asset_service.general_assets_buildable().await?;
    Ok((StatusCode::OK, Json(res)))
}

async fn resolve_id_from_name_bulk(
    asset_service: Extension<AssetService>,
    Query(filter): Query<ResolveIdNameFilter>,
    Json(body):    Json<Vec<String>>
) -> Result<impl IntoResponse, ServerError> {
    let res = asset_service.resolve_id_from_name_bulk(body, filter).await?;
    Ok((StatusCode::OK, Json(res)))
}
