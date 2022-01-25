use crate::{AssetService, AuthUser};
use crate::error::ServerError;

use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::extract::{Extension, Path};
use axum::routing::{get, put};
use caph_connector::{SystemId, TypeId};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all).post(create))
        .nest(
            "/:pid",
            Router::new()
                .route("/", get(by_id).put(edit).delete(delete))
                .route("/blueprints/required", get(required_blueprints))
                .route("/blueprints/info", get(info_blueprints))
                .route("/buildsteps", get(buildsteps))
                .route("/budget", get(trackings).post(add_budget_entry))
                .route("/budget/:tid", put(edit_budget_entry).delete(delete_budget_entry))
                .route("/market/:sid/buy", get(market_buy))
                .route("/market/:sid/sell", get(market_sell))
                .route("/materials/raw", get(raw_materials))
                .route("/materials/stored", get(stored_materials))
        )
}

/// Gets a specific project by its id
async fn by_id(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>
) -> Result<Json<caph_core::Project>, ServerError> {
    user.assert_access().await?;

    let entry = service
        .by_id(pid)
        .await?;
    if let Some(x) = entry {
        Ok(Json(x))
    } else {
        Err(ServerError::NotFound)
    }
}

/// Gets all projects the user has access to
async fn get_all(
    user:    AuthUser,
    service: Extension<caph_core::ProjectService>,
) -> Result<Json<Vec<caph_core::Info>>, ServerError> {
    user.assert_access().await?;

    let cid = user.character_id().await?;
    service
        .all(cid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Creates a new project
async fn create(
    user:       AuthUser,
    service:    Extension<caph_core::ProjectService>,
    Json(body): Json<caph_core::Config>
) -> Result<impl IntoResponse, ServerError> {
    user.assert_access().await?;

    let cid = user.character_id().await?;
    service
        .create(cid, body)
        .await
        .map(|x| (StatusCode::CREATED, Json(x)))
        .map_err(ServerError::CaphCoreProject)
}

/// Edits a project and overwrites it with the given data
async fn edit(
    user:       AuthUser,
    service:    Extension<caph_core::ProjectService>,
    Path(pid):  Path<caph_core::ProjectId>,
    Json(body): Json<caph_core::Config>
) -> Result<Json<caph_core::ProjectId>, ServerError> {
    user.assert_access().await?;

    service
        .edit(pid, body)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Deletes the given project
async fn delete(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>,
) -> Result<Json<caph_core::ProjectId>, ServerError> {
    user.assert_access().await?;

    let entry = service
        .delete(pid)
        .await?;
    if let Some(x) = entry {
        Ok(Json(x))
    } else {
        Err(ServerError::NotFound)
    }
}

/// Gets all raw materials needed for the project
async fn raw_materials(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>
) -> Result<Json<Vec<caph_core::Material>>, ServerError> {
    user.assert_access().await?;

    service
        .raw_materials(pid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Gets all stored materials
async fn stored_materials(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>
) -> Result<Json<Vec<caph_core::Material>>, ServerError> {
    user.assert_access().await?;

    service
        .stored_materials(pid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Gets all blueprints that are required for the project
async fn required_blueprints(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>
) -> Result<Json<Vec<caph_core::Blueprint>>, ServerError> {
    user.assert_access().await?;

    service
        .required_blueprints(pid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Gets all stored blueprints for a project
async fn info_blueprints(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>
) -> Result<Json<Vec<caph_core::BlueprintInfo>>, ServerError> {
    user.assert_access().await?;

    service
        .info_blueprints(pid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

async fn buildsteps(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>
) -> Result<Json<caph_core::Buildstep>, ServerError> {
    user.assert_access().await?;

    service
        .buildsteps(pid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Gets a list of all raw items and their pricing
async fn market_buy(
    user:             AuthUser,
    service:          Extension<caph_core::ProjectService>,
    Path((pid, sid)): Path<(caph_core::ProjectId, SystemId)>
) -> Result<Json<Vec<caph_core::ProjectMarketItemPrice>>, ServerError> {
    user.assert_access().await?;

    service
        .market_buy_price(pid, sid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Gets a list of products and their current pricing
async fn market_sell(
    user:             AuthUser,
    service:          Extension<caph_core::ProjectService>,
    Path((pid, sid)): Path<(caph_core::ProjectId, SystemId)>
) -> Result<Json<Vec<caph_core::ProjectMarketItemPrice>>, ServerError> {
    user.assert_access().await?;

    service
        .market_sell_price(pid, sid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Fetches all costs that where added to the project
async fn trackings(
    user:      AuthUser,
    service:   Extension<caph_core::ProjectService>,
    Path(pid): Path<caph_core::ProjectId>
) -> Result<Json<Vec<caph_core::BudgetEntry>>, ServerError> {
    user.assert_access().await?;

    service
        .budget(pid)
        .await
        .map(Json)
        .map_err(ServerError::CaphCoreProject)
}

/// Adds a new cost to the project
async fn add_budget_entry(
    user:       AuthUser,
    service:    Extension<caph_core::ProjectService>,
    Path(pid):  Path<caph_core::ProjectId>,
    Json(body): Json<caph_core::AddBudgetEntry>
) -> Result<impl IntoResponse, ServerError> {
    user.assert_access().await?;

    service
        .add_budget_entry(pid, body)
        .await
        .map(|_| (StatusCode::CREATED, ""))
        .map_err(ServerError::CaphCoreProject)
}

/// Edits a tracking entry
async fn edit_budget_entry(
    user:             AuthUser,
    service:          Extension<caph_core::ProjectService>,
    Path((pid, tid)): Path<(caph_core::ProjectId, caph_core::BudgetId)>,
    Json(body):       Json<caph_core::BudgetEntry>
) -> Result<impl IntoResponse, ServerError> {
    user.assert_access().await?;

    service
        .edit_budget_entry(pid, tid, body)
        .await
        .map(|_| (StatusCode::OK, ""))
        .map_err(ServerError::CaphCoreProject)
}

/// Deletes a tracking entry
async fn delete_budget_entry(
    user:             AuthUser,
    service:          Extension<caph_core::ProjectService>,
    Path((pid, tid)): Path<(caph_core::ProjectId, caph_core::BudgetId)>,
) -> Result<impl IntoResponse, ServerError> {
    user.assert_access().await?;

    service
        .delete_budget_entry(pid, tid)
        .await
        .map(|_| (StatusCode::OK, ""))
        .map_err(ServerError::CaphCoreProject)
}

#[derive(Clone)]
pub struct ProjectService {
    pool:  PgPool,

    asset: AssetService
}

impl ProjectService {
    pub fn new(
        pool:  PgPool,

        asset: AssetService
    ) -> Self {
        Self {
            pool,

            asset
        }
    }

    /*pub async fn project_cost(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<ProjectCost, ServerError> {
        let mut material_cost = HashMap::new();
        sqlx::query!(r#"
                SELECT
                  bmm.type_id AS "type_id!",
                  CEIL(ptp.count::FLOAT / bm.quantity::FLOAT) AS runs,
                  CEIL(
                    CEIL(
                        ptp.count::FLOAT / bm.quantity::FLOAT
                    ) * bmm.quantity
                  ) AS "quantity!",
                  CEIL(
                    CEIL(
                            ptp.count::FLOAT / bm.quantity::FLOAT
                        ) * bmm.quantity * mp.adjusted_price
                    ) AS "price!",
                  i.name
                FROM project p
                JOIN project_template_product ptp
                  ON ptp.template_id = p.template
                JOIN blueprint_material bm
                  ON bm.type_id = ptp.type_id
                JOIN blueprint_material bmm
                  ON bmm.blueprint = bm.blueprint
                JOIN market_price mp
                  ON mp.type_id = bmm.type_id
                JOIN item i
                  ON i.type_id = bmm.type_id
                WHERE bm.activity = 2
                  AND bmm.activity = 2
                  AND bm.is_product = TRUE
                  AND bmm.is_product = FALSE
                  AND p.owner = $1
                  AND p.id = $2
            "#,
                *cid,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .for_each(|x| {
                material_cost
                    .entry(x.type_id)
                    .and_modify(|y: &mut ProjectSubCost| {
                        y.quantity += x.quantity as i32;
                        y.price    += x.price    as i32;
                    })
                    .or_insert(ProjectSubCost {
                        name:     x.name,
                        type_id:  (x.type_id as i32).into(),
                        quantity: x.quantity as i32,
                        price:    x.price    as i32
                    });
            });

        let materials = material_cost
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        let material_total_cost: f32 = materials
            .iter()
            .map(|x| x.price as f32)
            .sum();

        let facility_tax_perc = 3f32; // TODO: replace
        let system_cost_index_perc = sqlx::query!("
                SELECT cost_index
                FROM industry_system
                WHERE system_id = $1
                  AND activity = 'manufacturing'
            ",
                30002001 // TODO: replace with actual system id
            )
            .fetch_one(&self.pool)
            .await?
            .cost_index as f32;
        let facility_bonus_perc = 4f32; // TODO replace with actual structure bonus

        let system_cost_index = f32::round(material_total_cost * system_cost_index_perc);
        let facility_bonus = f32::round(system_cost_index * (facility_bonus_perc / 100f32));

        let mut production_cost = system_cost_index - facility_bonus;
        let facility_tax = f32::round(production_cost * (facility_tax_perc as f32 / 100f32));
        production_cost += facility_tax;

        let mut products = HashMap::new();
        sqlx::query!(r#"
                SELECT
                  bm.type_id AS "type_id!",
                  CEIL(ptp.count::FLOAT / bm.quantity::FLOAT) AS runs,
                  CEIL(
                    CEIL(
                        ptp.count::FLOAT / bm.quantity::FLOAT
                        ) * bm.quantity
                    ) AS "quantity!",
                  CEIL(
                    CEIL(
                        ptp.count::FLOAT / bm.quantity::FLOAT
                        ) * bm.quantity * mp.adjusted_price
                    ) AS "price!",
                  i.name
                FROM project p
                JOIN project_template_product ptp
                  ON ptp.template_id = p.template
                JOIN blueprint_material bm
                  ON bm.type_id = ptp.type_id
                JOIN market_price mp
                  ON mp.type_id = bm.type_id
                JOIN item i
                  ON i.type_id = bm.type_id
                WHERE bm.activity = 2
                  AND bm.is_product = TRUE
                  AND p.owner = $1
                  AND p.id = $2
            "#,
                *cid,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .for_each(|x| {
                products
                    .entry(x.type_id)
                    .and_modify(|y: &mut ProjectSubCost| {
                        y.quantity += x.quantity as i32;
                        y.price    += x.price    as i32;
                    })
                    .or_insert(ProjectSubCost {
                        name:     x.name,
                        type_id:  (x.type_id as i32).into(),
                        quantity: x.quantity as i32,
                        price:    x.price    as i32
                    });
            });

        let products = products
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        let sell_price: f32 = products
            .iter()
            .map(|x| x.price as f32)
            .sum();
        let total_cost = material_total_cost + production_cost;

        Ok(ProjectCost {
            products,
            materials,

            material_total_cost,
            system_cost_index,
            system_cost_index_perc,
            facility_bonus,
            facility_bonus_perc,
            facility_tax,
            facility_tax_perc,
            production_cost,
            total_cost,
            sell_price,
        })
    }*/
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCost {
    pub products:                Vec<ProjectSubCost>,
    pub materials:               Vec<ProjectSubCost>,

    pub material_total_cost:     f32,
    pub system_cost_index:       f32,
    pub system_cost_index_perc:  f32,
    pub facility_bonus:          f32,
    pub facility_bonus_perc:     f32,
    pub facility_tax:            f32,
    pub facility_tax_perc:       f32,
    pub production_cost:         f32,
    pub total_cost:              f32,
    pub sell_price:              f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSubCost {
    pub name:     String,
    pub type_id:  TypeId,
    pub quantity: i32,
    pub price:    i32,
}
