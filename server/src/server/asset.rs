use crate::{AssetFilter, ResolveIdNameFilter};
use crate::asset::AssetService;
use crate::error::ServerError;
use crate::eve::LoggedInCharacter;

use axum::{Json, Router};
use axum::extract::{Extension, Path, Query};
use axum::handler::{get, post};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use caph_connector::{ItemId, TypeId};

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(assets)).boxed()
        .route("/:iid", get(asset_by_id)).boxed()
        .route("/:iid/name", get(asset_name)).boxed()
        .route("/:tid/blueprint/material", get(asset_blueprint_material)).boxed()
        .route("/:tid/blueprint/flat", get(asset_blueprint_flat)).boxed()
        .route("/:tid/blueprint/tree", get(asset_blueprint_tree)).boxed()
        .route("/:tid/blueprint/raw", get(asset_blueprint_raw)).boxed()
        .route("/all/buildable", get(general_assets_buildable)).boxed()
        .route("/resolve/id", post(resolve_id_from_name_bulk)).boxed()
}

async fn asset_by_id(
    asset_service: Extension<AssetService>,
    character:     LoggedInCharacter,
    Path(iid):     Path<ItemId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let res = asset_service.asset(cid, iid).await?;
    Ok((StatusCode::OK, Json(res)))
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
