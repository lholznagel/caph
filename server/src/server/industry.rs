use crate::error::ServerError;
use crate::eve::LoggedInCharacter;
use crate::industry::IndustryService;

use axum::{Json, Router};
use axum::extract::Extension;
use axum::handler::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::BoxRoute;

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/jobs", get(jobs)).boxed()
}

async fn jobs(
    industry_service: Extension<IndustryService>,
    character:        LoggedInCharacter,
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let res = industry_service.jobs(cid).await?;
    Ok((StatusCode::OK, Json(res)))
}
