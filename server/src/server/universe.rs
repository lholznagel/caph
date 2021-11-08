use crate::Station;
use crate::auth_user::AuthUser;
use crate::error::ServerError;
use crate::universe::UniverseService;

use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use caph_connector::{StationId, SystemId};

pub fn router() -> Router {
    Router::new()
        .route("/stations", get(stations).post(add_station))
        .route("/stations/:sid", get(station).delete(delete_station))
        .route("/systems", get(systems))
        .route("/systems/:sid", get(system))
}

async fn stations(
    universe_service: Extension<UniverseService>,
    user:             AuthUser,
) -> Result<impl IntoResponse, ServerError> {
    let cid = user.character_id().await?;
    let res = universe_service.stations(cid).await?;
    Ok((StatusCode::OK, Json(res)))
}

async fn add_station(
    universe_service: Extension<UniverseService>,
    user:             AuthUser,
    Json(body):       Json<Station>
) -> Result<impl IntoResponse, ServerError> {
    let cid = user.character_id().await?;
    universe_service.add_station(cid, body).await?;
    Ok((StatusCode::CREATED, Json(())))
}

async fn station(
    universe_service: Extension<UniverseService>,
    user:             AuthUser,
    Path(sid):        Path<StationId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = user.character_id().await?;
    // TODO: handle not found
    let res = universe_service.station(cid, sid).await?;
    Ok((StatusCode::OK, Json(res)))

    /*if let Some(name) = asset_service.station_name(sid).await? {
        Ok((StatusCode::OK, Json(name)))
    } else {
        return Ok((StatusCode::NOT_FOUND, "Not found".into()));
    }*/
}

async fn delete_station(
    universe_service: Extension<UniverseService>,
    user:             AuthUser,
    Path(sid):        Path<StationId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = user.character_id().await?;
    universe_service.delete_station(cid, sid).await?;
    Ok((StatusCode::OK, Json(())))
}

async fn systems(
    universe_service: Extension<UniverseService>,
) -> Result<impl IntoResponse, ServerError> {
    let res = universe_service.systems().await?;
    Ok((StatusCode::OK, Json(res)))
}


async fn system(
    universe_service: Extension<UniverseService>,
    Path(sid):        Path<SystemId>
) -> Result<impl IntoResponse, ServerError> {
    let res = universe_service.system(sid).await?;
    Ok((StatusCode::OK, Json(res)))
}
