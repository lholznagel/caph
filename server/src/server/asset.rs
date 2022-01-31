use crate::asset::AssetService;
use crate::error::ServerError;

use axum::{Json, Router};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

/// Router for handling assets
///
/// # Returns
///
/// Router with all routes for assets
///
pub fn router() -> Router {
    Router::new()
        .route("/all/buildable", get(general_assets_buildable))
}

async fn general_assets_buildable(
    asset_service: Extension<AssetService>,
) -> Result<impl IntoResponse, ServerError> {
    let res = asset_service.general_assets_buildable().await?;
    Ok((StatusCode::OK, Json(res)))
}
