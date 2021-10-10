use crate::{CreateProject, ProjectService};
use crate::error::ServerError;
use crate::eve::LoggedInCharacter;

use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::handler::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use uuid::Uuid;

pub fn router() -> Router<BoxRoute> {
    Router::new()
        .route("/",
            get(projects)
            .post(create_project)
        ).boxed()
        .route("/:pid",
            get(by_id)
            .put(update)
            .delete(delete)
        ).boxed()
        .route("/:pid/containers",get(containers)).boxed()
        .route("/:pid/products",get(products)).boxed()
        .route("/:pid/blueprints/required",get(required_blueprints)).boxed()
        .route("/:pid/materials/stored",get(stored_materials)).boxed()
        .route("/:pid/raw",get(raw_materials)).boxed()
}

async fn projects(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let project = project_service.projects(cid).await?;
    Ok((StatusCode::OK, Json(project)))
}

async fn create_project(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Json(body):      Json<CreateProject>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let pid = project_service.create(cid, body).await?;
    Ok((StatusCode::OK, Json(pid)))
}

async fn by_id(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let project = project_service.by_id(cid, pid).await?;
    Ok((StatusCode::OK, Json(project)))
}

async fn containers(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let entries = project_service.containers(cid, pid).await?;
    Ok((StatusCode::OK, Json(entries)))
}

async fn products(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let products = project_service.products(cid, pid).await?;
    Ok((StatusCode::OK, Json(products)))
}

async fn update(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>,
    Json(body):      Json<CreateProject>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    project_service.update(cid, pid, body).await?;
    Ok((StatusCode::OK, Json(())))
}

async fn delete(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    project_service.delete(cid, pid).await?;
    Ok((StatusCode::OK, Json(())))
}

async fn required_blueprints(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let bps = project_service.required_blueprints(cid, pid).await?;
    Ok((StatusCode::OK, Json(bps)))
}

async fn stored_materials(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let entries = project_service.stored_materials(cid, pid).await?;
    Ok((StatusCode::OK, Json(entries)))
}

async fn raw_materials(
    project_service: Extension<ProjectService>,
    character:       LoggedInCharacter,
    Path(pid):       Path<Uuid>
) -> Result<impl IntoResponse, ServerError> {
    let cid = character.character_id().await?;
    let entries = project_service.required_raw_materials(cid, pid).await?;
    Ok((StatusCode::OK, Json(entries)))
}

