use crate::{AssetFilter, BlueprintFilter};
use crate::asset::AssetService;
use crate::error::ServerError;
use crate::eve::LoggedInCharacter;

use axum::{Json, Router};
use axum::extract::{Extension, Path, Query};
use axum::handler::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use caph_connector::{ItemId, LocationId, TypeId};

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(assets)).boxed()
        .route("/:iid", get(asset_by_id)).boxed()
        .route("/:iid/name", get(asset_name)).boxed()
        .route("/blueprints", get(blueprints)).boxed()
        .route("/blueprints/:tid/:iid", get(character_blueprint)).boxed()
        .route("/blueprints/:tid/material", get(blueprint_material)).boxed()
        .route("/blueprints/:tid/product", get(blueprint_product)).boxed()
        .route("/location/:sid/name", get(location_name)).boxed()
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

async fn asset_name(
    asset_service: Extension<AssetService>,
    character:     LoggedInCharacter,
    Path(iid):     Path<ItemId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let res = asset_service.asset_name(cid, iid).await?;
    Ok((StatusCode::OK, Json(res)))
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

async fn blueprint_material(
    asset_service: Extension<AssetService>,
    Path(tid):     Path<TypeId>
) -> Result<impl IntoResponse, ServerError> {
    let bp = asset_service.blueprint_material(tid).await?;
    Ok((StatusCode::OK, Json(bp)))
}

async fn blueprint_product(
    asset_service: Extension<AssetService>,
    Path(tid):     Path<TypeId>
) -> Result<impl IntoResponse, ServerError> {
    let bp = asset_service.blueprint_product(tid).await?;
    Ok((StatusCode::OK, Json(bp)))
}

async fn character_blueprint(
    asset_service:    Extension<AssetService>,
    character:        LoggedInCharacter,
    Path((tid, iid)): Path<(TypeId, ItemId)>,
) -> Result<impl IntoResponse, ServerError> {
    let cids = character.character_alts().await?;
    let bp = asset_service.character_blueprint(cids, tid, iid).await?;
    Ok((StatusCode::OK, Json(bp)))
}

async fn blueprints(
    asset_service: Extension<AssetService>,
    character:     LoggedInCharacter,
    Query(filter): Query<BlueprintFilter>
) -> Result<impl IntoResponse, ServerError> {
    let cids = character.character_alts().await?;
    let alts = asset_service.blueprints(cids, filter).await?;
    Ok((StatusCode::OK, Json(alts)))
}

async fn location_name(
    asset_service: Extension<AssetService>,
    Path(sid):     Path<LocationId>
) -> Result<impl IntoResponse, ServerError> {
    if let Some(name) = asset_service.station_name(sid).await? {
        Ok((StatusCode::OK, Json(name)))
    } else {
        Ok((StatusCode::NOT_FOUND, Json("Not found".into())))
    }
}
