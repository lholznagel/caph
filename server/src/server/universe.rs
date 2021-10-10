use crate::Station;
use crate::error::ServerError;
use crate::eve::LoggedInCharacter;
use crate::universe::UniverseService;

use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::handler::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use caph_connector::{StationId, SystemId};

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/stations", get(stations).post(add_station)).boxed()
        .route("/stations/:sid", get(station).delete(delete_station)).boxed()
        .route("/systems", get(systems)).boxed()
        .route("/systems/:sid", get(system)).boxed()
}

async fn stations(
    universe_service: Extension<UniverseService>,
    character:        LoggedInCharacter,
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let res = universe_service.stations(cid).await?;
    Ok((StatusCode::OK, Json(res)))
}

async fn add_station(
    universe_service: Extension<UniverseService>,
    character:        LoggedInCharacter,
    Json(body):       Json<Station>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    universe_service.add_station(cid, body).await?;
    Ok((StatusCode::CREATED, Json(())))
}

async fn station(
    universe_service: Extension<UniverseService>,
    character:        LoggedInCharacter,
    Path(sid):        Path<StationId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
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
    character:        LoggedInCharacter,
    Path(sid):        Path<StationId>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
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