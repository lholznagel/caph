use axum::{Router, Json};
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::routing::get;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{AuthUser, Error};

use super::{MoonService, Pull};

pub fn router() -> Router {
    Router::new()
        .nest(
            "/pulls",
            Router::new()
                .route(
                    "/",
                    get(pulls)
                    .post(create)
                )
                .route(
                    "/:id",
                    get(pull)
                        .put(update)
                )
        )
}

async fn pull(
    user:               AuthUser,
    Extension(service): Extension<MoonService>,
    Path(id):           Path<Uuid>,
) -> Result<Json<Pull>, Error> {
    let cid = user.character_id().await?;

    service
        .pull(cid, id)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn pulls(
    user:               AuthUser,
    Extension(service): Extension<MoonService>,
) -> Result<Json<Vec<Pull>>, Error> {
    let cid = user.character_id().await?;

    service
        .pulls(cid)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn create(
    user:               AuthUser,
    Extension(service): Extension<MoonService>,
    Json(pull):         Json<Pull>
) -> Result<impl IntoResponse, Error> {
    let cid = user.character_id().await?;

    service
        .create(cid, pull)
        .await
        .map(|_| (StatusCode::OK, ""))
        .map_err(Into::into)
}

async fn update(
    user:               AuthUser,
    Extension(service): Extension<MoonService>,
    Path(id):           Path<Uuid>,
    Json(pull):         Json<Pull>
) -> Result<impl IntoResponse, Error> {
    let cid = user.character_id().await?;

    service
        .update(cid, id, pull)
        .await
        .map(|_| (StatusCode::OK, ""))
        .map_err(Into::into)
}
