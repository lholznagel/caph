use super::service::{ItemService, ResolveIdNameFilter};

use crate::Error;

use axum::{Json, Router};
use axum::extract::{Extension, Query, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use caph_connector::TypeId;

pub struct ItemApi;

impl ItemApi {
    /// Contains all routes that are under the `items` route.
    pub fn router() -> Router {
        Router::new()
            .route("/components", get(Self::components))
            .route("/buildable", get(Self::buildable))
            .route("/resolve", post(Self::resolve_id_from_name_bulk))
            .route("/resolve/id/:tid", post(Self::resolve_tid))
    }

    /// Fetches a list of all manufacturable and inventionable items.
    async fn components(
        service: Extension<ItemService>,
    ) -> Result<impl IntoResponse, Error> {
        service
            .components()
            .await
            .map(|x| (StatusCode::OK, Json(x)))
            .map_err(Into::into)
    }

    /// Fetches a list of items that are associated with a blueprint.
    async fn buildable(
        service: Extension<ItemService>,
    ) -> Result<impl IntoResponse, Error> {
        service
            .buildable()
            .await
            .map(|x| (StatusCode::OK, Json(x)))
            .map_err(Into::into)
    }

    /// Resolve a TypeId to its item
    async fn resolve_tid(
        service:       Extension<ItemService>,
        Path(tid):     Path<TypeId>
    ) -> Result<impl IntoResponse, Error> {
        service
            .resolve_id(tid)
            .await
            .map(|x| (StatusCode::OK, Json(x)))
            .map_err(Into::into)
    }

    /// Takes a list of names and resolves those names to [TypeId]s.
    async fn resolve_id_from_name_bulk(
        service:       Extension<ItemService>,
        Query(filter): Query<ResolveIdNameFilter>,
        Json(body):    Json<Vec<String>>
    ) -> Result<impl IntoResponse, Error> {
        service
            .resolve_id_from_name_bulk(body, filter)
            .await
            .map(|x| (StatusCode::OK, Json(x)))
            .map_err(Into::into)
    }
}
