use crate::error::ServerError;
use crate::eve::LoggedInCharacter;
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
    character:        LoggedInCharacter,
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let res = industry_service.jobs(cid).await?;
    Ok((StatusCode::OK, Json(res)))
}
