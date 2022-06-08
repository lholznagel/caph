use crate::{AuthUser, ProjectService, ProjectId, Info, Config, Project, Material, Blueprint, Buildstep, BudgetEntry, AddBudgetEntry, BudgetId, StorageEntry, Modify, GodProject, ModifyRequest, BlueprintStorageEntry, ProjectBlueprintService, ProjectStorageService};
use crate::error::Error;

use appraisal::AppraisalInformation;
use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use axum::routing::{delete, get, put};
use caph_connector::{TypeId, CharacterId};
use reqwest::StatusCode;
use serde::Serialize;
use sqlx::PgPool;

use super::dependency::Dependency;

pub struct ProjectApi;

impl ProjectApi {
    pub fn router() -> Router {
        Router::new()
            .route("/", get(Self::get_all).post(Self::create))
            .nest(
                "/:pid",
                Router::new()
                    .route("/", get(Self::by_id).put(Self::edit).delete(Self::delete_project))
                    .route("/god", get(Self::god))
                    .route("/name", get(Self::name))
                    .route("/blueprints", get(Self::required_blueprints))
                    .route("/blueprints/stored", get(Self::blueprints_stored))
                    .route("/blueprints/import", put(Self::import_blueprints))
                    .route("/buildsteps", get(Self::buildsteps))
                    .route("/budget", get(Self::budget_entries).post(Self::add_budget_entry))
                    .route("/budget/:bid", get(Self::budget_entry).put(Self::edit_budget_entry).delete(Self::delete_budget_entry))
                    .route("/market", get(Self::market_price))
                    .route("/materials/raw", get(Self::raw_materials))
                    .route("/materials/stored", get(Self::stored_materials))
                    .route("/members", get(Self::members).post(Self::add_member))
                    .route("/members/:cid", delete(Self::kick_member))
                    .route("/storage", get(Self::storage).put(Self::storage_modify).post(Self::set_storage))
                    .route("/storage/:tid", get(Self::storage_by_id))
            )
    }

    /// Gets a specific project name
    async fn name(
        Extension(pool): Extension<PgPool>,
        Path(pid):       Path<ProjectId>
    ) -> Result<impl IntoResponse, Error> {
        let entry = ProjectService::by_id(&pool, &pid).await?;
        if let Some(x) = entry {
            #[derive(Serialize)]
            struct R {
                name:  String,
                owner: CharacterId,
            }

            Ok(Json(R {
                name:  x.name,
                owner: x.owner
            }))
        } else {
            Err(Error::NotFound)
        }
    }

    /// Gets all information about a project
    async fn god(
        user:      AuthUser,
        service:   Extension<ProjectService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<GodProject>, Error> {
        user.assert_project_access(pid).await?;

        service
            .god(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Gets a specific project by its id
    async fn by_id(
        user:            AuthUser,
        Extension(pool): Extension<PgPool>,
        Path(pid):       Path<ProjectId>
    ) -> Result<Json<Project>, Error> {
        user.assert_project_access(pid).await?;

        let entry = ProjectService::by_id(&pool, &pid).await?;
        if let Some(x) = entry {
            Ok(Json(x))
        } else {
            Err(Error::NotFound)
        }
    }

    /// Gets all projects the user has access to
    async fn get_all(
        user:    AuthUser,
        service: Extension<ProjectService>,
    ) -> Result<Json<Vec<Info>>, Error> {
        let cid = user.character_id().await?;
        service
            .all(cid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Creates a new project
    async fn create(
        user:       AuthUser,
        service:    Extension<ProjectService>,
        Json(body): Json<Config>
    ) -> Result<impl IntoResponse, Error> {
        let cid = user.character_id().await?;
        service
            .create(cid, body)
            .await
            .map(|x| (StatusCode::CREATED, Json(x)))
            .map_err(Into::into)
    }

    /// Edits a project and overwrites it with the given data
    async fn edit(
        user:       AuthUser,
        service:    Extension<ProjectService>,
        Path(pid):  Path<ProjectId>,
        Json(body): Json<Config>
    ) -> Result<Json<ProjectId>, Error> {
        user.assert_project_access(pid).await?;

        service
            .edit(pid, body)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Deletes the given project
    async fn delete_project(
        user:      AuthUser,
        service:   Extension<ProjectService>,
        Path(pid): Path<ProjectId>,
    ) -> Result<Json<ProjectId>, Error> {
        user.assert_project_access(pid).await?;

        let entry = service
            .delete(pid)
            .await?;
        if let Some(x) = entry {
            Ok(Json(x))
        } else {
            Err(Error::NotFound)
        }
    }

    /// Gets all raw materials needed for the project
    async fn raw_materials(
        user:      AuthUser,
        service:   Extension<ProjectService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<Vec<Dependency>>, Error> {
        user.assert_project_access(pid).await?;

        service
            .raw_materials(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Gets all stored materials
    async fn stored_materials(
        user:      AuthUser,
        service:   Extension<ProjectService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<Vec<Material>>, Error> {
        user.assert_project_access(pid).await?;

        service
            .stored_materials(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Checks if any character has any of the required blueprints and adds them
    async fn import_blueprints(
        user:      AuthUser,
        project:   Extension<ProjectService>,
        service:   Extension<ProjectBlueprintService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<()>, Error> {
        let cid = user.character_id().await?;
        user.assert_project_access(pid).await?;

        let buildsteps = project
            .buildstep_manufacturing(pid)
            .await?;

        service
            .import_from_character(pid, cid, buildsteps)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Gets all blueprints that are required for the project
    async fn required_blueprints(
        user:      AuthUser,
        project:   Extension<ProjectService>,
        service:   Extension<ProjectBlueprintService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<Vec<Blueprint>>, Error> {
        user.assert_project_access(pid).await?;

        let buildsteps = project
            .buildstep_manufacturing(pid)
            .await?;

        service
            .required(buildsteps)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Gets all blueprints that are required for the project
    async fn blueprints_stored(
        user:      AuthUser,
        service:   Extension<ProjectBlueprintService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<Vec<BlueprintStorageEntry>>, Error> {
        user.assert_project_access(pid).await?;

        service
            .stored(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    async fn buildsteps(
        user:      AuthUser,
        service:   Extension<ProjectService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<Buildstep>, Error> {
        user.assert_project_access(pid).await?;

        service
            .buildsteps(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Gets a list of all items and their price
    async fn market_price(
        user:      AuthUser,
        service:   Extension<ProjectService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<AppraisalInformation>, Error> {
        user.assert_project_access(pid).await?;

        service
            .market_price(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Fetches all costs that where added to the project
    async fn budget_entries(
        user:      AuthUser,
        service:   Extension<ProjectService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<Vec<BudgetEntry>>, Error> {
        user.assert_project_access(pid).await?;

        service
            .budget(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Adds a new cost to the project
    async fn add_budget_entry(
        user:       AuthUser,
        service:    Extension<ProjectService>,
        Path(pid):  Path<ProjectId>,
        Json(body): Json<AddBudgetEntry>
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .add_budget_entry(pid, body)
            .await
            .map(|_| (StatusCode::CREATED, ""))
            .map_err(Into::into)
    }

    /// Gets a specific budget entry
    async fn budget_entry(
        user:             AuthUser,
        service:          Extension<ProjectService>,
        Path((pid, bid)): Path<(ProjectId, BudgetId)>
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .budget_entry(pid, bid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Edits a tracking entry
    async fn edit_budget_entry(
        user:             AuthUser,
        service:          Extension<ProjectService>,
        Path((pid, bid)): Path<(ProjectId, BudgetId)>,
        Json(body):       Json<BudgetEntry>
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .edit_budget_entry(pid, bid, body)
            .await
            .map(|_| (StatusCode::OK, ""))
            .map_err(Into::into)
    }

    /// Deletes a tracking entry
    async fn delete_budget_entry(
        user:             AuthUser,
        service:          Extension<ProjectService>,
        Path((pid, bid)): Path<(ProjectId, BudgetId)>,
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .delete_budget_entry(pid, bid)
            .await
            .map(|_| (StatusCode::OK, ""))
            .map_err(Into::into)
    }

    /// Gets a list of members
    async fn members(
        user:       AuthUser,
        service:    Extension<ProjectService>,
        Path(pid):  Path<ProjectId>,
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .members(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Adds a member to a project
    async fn add_member(
        user:       AuthUser,
        service:    Extension<ProjectService>,
        Path(pid):  Path<ProjectId>,
    ) -> Result<impl IntoResponse, Error> {
        let cid = user.character_id().await?;
        service
            .add_member(pid, cid)
            .await
            .map(|_| (StatusCode::OK, ""))
            .map_err(Into::into)
    }

    /// Kicks a member from a project
    async fn kick_member(
        user:             AuthUser,
        service:          Extension<ProjectService>,
        Path((pid, cid)): Path<(ProjectId, CharacterId)>,
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .kick_member(pid, cid)
            .await
            .map(|_| (StatusCode::OK, ""))
            .map_err(Into::into)
    }

    /// Gets a specific project by its id
    async fn storage(
        user:      AuthUser,
        service:   Extension<ProjectStorageService>,
        Path(pid): Path<ProjectId>
    ) -> Result<Json<Vec<StorageEntry>>, Error> {
        user.assert_project_access(pid).await?;

        service
            .stored(pid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Gets a specific project by its id
    async fn storage_by_id(
        user:             AuthUser,
        service:          Extension<ProjectStorageService>,
        Path((pid, tid)): Path<(ProjectId, TypeId)>
    ) -> Result<Json<Option<StorageEntry>>, Error> {
        user.assert_project_access(pid).await?;

        service
            .storage_by_id(pid, tid)
            .await
            .map(Json)
            .map_err(Into::into)
    }

    /// Edits a tracking entry
    async fn storage_modify(
        user:       AuthUser,
        service:    Extension<ProjectStorageService>,
        Path(pid):  Path<ProjectId>,
        Json(body): Json<ModifyRequest>
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .modify(pid, body)
            .await
            .map(|_| (StatusCode::OK, ""))
            .map_err(Into::into)
    }

    /// Edits a tracking entry
    async fn set_storage(
        user:       AuthUser,
        service:    Extension<ProjectStorageService>,
        Path(pid):  Path<ProjectId>,
        Json(body): Json<Vec<Modify>
        >
    ) -> Result<impl IntoResponse, Error> {
        user.assert_project_access(pid).await?;

        service
            .set_storage(pid, body)
            .await
            .map(|_| (StatusCode::OK, ""))
            .map_err(Into::into)
    }
}
