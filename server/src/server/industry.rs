use crate::auth_user::AuthUser;
use crate::error::ServerError;
use crate::industry::IndustryService;

use axum::{Json, Router};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/jobs", get(jobs))
}

async fn jobs(
    industry_service: Extension<IndustryService>,
    user:             AuthUser,
) -> Result<impl IntoResponse, ServerError> {
    let cid = user.character_id().await?;
    let res = industry_service.jobs(cid).await?;
    Ok((StatusCode::OK, Json(res)))
}
