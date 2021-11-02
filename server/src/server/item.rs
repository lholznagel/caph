use crate::item::ItemService;
use crate::error::ServerError;

use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use caph_connector::TypeId;

pub fn router() -> Router {
    Router::new()
        .route("/:tid/name", get(item_name))
}

async fn item_name(
    item_service: Extension<ItemService>,
    Path(tid):    Path<TypeId>
) -> Result<impl IntoResponse, ServerError> {
    let res = item_service.name(tid).await?;
    Ok((StatusCode::OK, Json(res)))
}
