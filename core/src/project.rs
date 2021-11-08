use crate::MarketService;

use caph_connector::{CharacterId, GroupId, ItemId, LocationId, TypeId, SystemId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::{HashSet, HashMap, VecDeque};
use tracing::instrument;
use uuid::Uuid;

/// A project id is just a UUID, this type is just for clarification
pub type ProjectId  = Uuid;
/// An id of a tracking entry
pub type TrackingId = Uuid;

/// Wrapper for managing projects
#[derive(Clone)]
pub struct ProjectService {
    /// Database pool
    pool:   PgPool,

    /// [MarketService] for handling market requests
    market: MarketService
}

impl ProjectService {
    /// Creates a new service instance.
    ///
    /// # Params
    ///
    /// * `pool` -> Connection pool to the postgres
    ///
    /// # Returns
    ///
    /// New instance of the service.
    ///
    pub fn new(
        pool:   PgPool,
        market: MarketService
    ) -> Self {
        Self {
            pool,
            market
        }
    }

    /// Fetches an project by its identifier.
    ///
    /// # Params
    ///
    /// * `pid` -> [ProjectId] of the project
    ///
    /// # Errors
    ///
    /// Will throw an error if the database is not available.
    ///
    /// # Returns
    ///
    /// `Some(_)` if the project exists, otherwise `Ok(None)`.
    ///
    #[instrument(err)]
    pub async fn by_id(
        &self,
        pid: ProjectId,
    ) -> Result<Option<Project>, ProjectError> {
        let entry = sqlx::query!("
                SELECT
                    id,
                    owner,
                    name,
                    containers
                FROM   projects p
                JOIN   project_products pp
                  ON   p.id = pp.project
                WHERE  id = $1
            ",
                pid
            )
            .fetch_optional(&self.pool)
            .await?
            .map(|x| {
                let containers = x.containers
                    .into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<_>>();

                Project {
                    id:         x.id,
                    owner:      x.owner.into(),
                    name:       x.name,
                    containers: containers,
                    products:   Vec::new()
                }
            });

        if let Some(mut x) = entry {
            x.products = self.fetch_products(pid).await?;
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }

    /// Filters all projects.
    ///
    /// # Params
    ///
    /// * `cid` -> [CharacterId] of the requesting user
    ///
    /// # Errors
    ///
    /// When the database access is faulty
    ///
    /// # Returns
    ///
    /// List all project ids the user has access to.
    ///
    #[instrument(err)]
    pub async fn all(
        &self,
        cid: CharacterId
    ) -> Result<Vec<ProjectInfo>, ProjectError> {
        let entries = sqlx::query!(r#"
                SELECT
                    id,
                    name,
                    pinned,
                    status AS "status: ProjectStatus"
                FROM   projects
                WHERE  owner = $1
                ORDER BY name
            "#,
                *cid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| ProjectInfo {
                id:     x.id,
                name:   x.name,
                pinned: x.pinned,
                status: x.status
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Creates a new project.
    ///
    /// # Params
    ///
    /// * `cid`  -> [CharacterId] of the user that create the project
    /// * `info` -> Holds all information needed to create a project
    ///
    /// # Errors
    ///
    /// When communicating with the database fails
    ///
    /// # Returns
    ///
    /// [Uuid] of the created entry.
    ///
    #[instrument(err)]
    pub async fn create(
        &self,
        cid:  CharacterId,
        info: ProjectConfig
    ) -> Result<Uuid, ProjectError> {
        let containers = info.containers
            .iter()
            .map(|x| **x)
            .collect::<Vec<_>>();
        let pid = sqlx::query!("
                INSERT INTO projects
                (
                    owner,
                    name,
                    containers
                )
                VALUES ($1, $2, $3)
                RETURNING id
            ",
                *cid,
                info.name,
                &containers
            )
            .fetch_one(&self.pool)
            .await
            .map(|x| x.id)?;

        self.insert_products(pid, info.products).await?;
        Ok(pid)
    }

    /// Updates the given entry with the given data.
    ///
    /// # Params
    ///
    /// * `pid`  -> [ProjectId] of the project to update
    /// * `info` -> Updated info for the project
    ///
    /// # Errors
    ///
    /// If updating the entry in the database fails.
    ///
    /// # Returns
    ///
    /// [ProjectId] of the updated project
    ///
    #[instrument(err)]
    pub async fn edit(
        &self,
        pid:  ProjectId,
        info: ProjectConfig
    ) -> Result<ProjectId, ProjectError> {
        let containers = info.containers
            .iter()
            .map(|x| **x)
            .collect::<Vec<_>>();
        sqlx::query!("
                UPDATE projects
                   SET name = $2,
                       containers = $3
                WHERE  id = $1
            ",
                pid,
                info.name,
                &containers
            )
            .execute(&self.pool)
            .await?;
        self.insert_products(pid, info.products).await?;
        Ok(pid)
    }

    /// Deletes a project and its products
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project to delete
    ///
    /// # Errors
    ///
    /// If the database access fails
    ///
    /// # Returns
    ///
    /// `Some([ProjectId])` if the project existed, `None` otherwise.
    ///
    #[instrument(err)]
    pub async fn delete(
        &self,
        pid: ProjectId
    ) -> Result<Option<ProjectId>, ProjectError> {
        let entry = sqlx::query!("
                DELETE FROM projects
                WHERE id = $1
                RETURNING id
            ",
                pid
            )
            .fetch_optional(&self.pool)
            .await?
            .map(|x| x.id);
        Ok(entry)
    }

    /// Fetches a list of all required blueprints for a project.
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project
    ///
    /// # Errors
    ///
    /// If the database operation fails.
    ///
    /// # Returns
    ///
    /// List of all blueprints and if the character has those blueprints stored
    /// additional information, like the remaining runs and the efficiency.
    ///
    #[instrument(err)]
    pub async fn required_blueprints(
        &self,
        pid: ProjectId
    ) -> Result<Vec<ProjectBlueprint>, ProjectError> {
        let entries = sqlx::query!("
                SELECT tree
                FROM blueprint_tree
                WHERE type_id = ANY(
                    SELECT type_id
                    FROM project_products pp
                    WHERE pp.project = $1
                )
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await?;

        let mut parsed_tree = VecDeque::new();
        for entry in entries {
            let parsed: BlueprintTree = serde_json::from_value(entry.tree)
                .map_err(ProjectError::SerdeError)?;
            parsed_tree.push_back(parsed);
        }

        let mut unique_entries = HashSet::new();
        while let Some(entry) = parsed_tree.pop_front() {
            unique_entries.insert(entry.key);

            if let Some(x) = entry.children {
                parsed_tree.extend(x);
            }
        }

        let unique_entries = unique_entries
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>();

        let entries = sqlx::query!(r#"
                SELECT
                    b.type_id AS "type_id!",
                    i.name AS "name!",
                    ab.runs = -1 AS original,
                    ab.runs,
                    ab.material_efficiency,
                    ab.time_efficiency,
                    a.reference_id,
                    a.location_id,
                    a.reference_id = ANY(
                        SELECT UNNEST(containers) AS container
                        FROM projects
                        WHERE id = $1
                    ) AS stored
                FROM blueprint b
                JOIN blueprint_material bm
                ON bm.blueprint = b.id
                JOIN item i
                ON i.type_id = b.type_id
                LEFT JOIN asset a
                ON a.type_id = i.type_id
                LEFT JOIN asset_blueprint ab
                ON ab.item_id = a.item_id
                WHERE bm.type_id = ANY($2)
                AND bm.is_product = TRUE
            "#,
                pid,
                &unique_entries,
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                (x.type_id, ProjectBlueprint {
                    type_id:      x.type_id.into(),
                    name:         x.name,
                    original:     x.original,
                    runs:         x.runs,
                    material_eff: x.material_efficiency,
                    time_eff:     x.time_efficiency,
                    location_id:  x.location_id.map(|x| x.into()),
                    container_id: x.reference_id.map(|x| x.into()),
                })
            })
            .collect::<HashMap<_, _>>()
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();

        Ok(entries)
    }

    /// Gets all items that are stored in the configured project containers.
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project to fetch the materials for
    ///
    /// # Errors
    ///
    /// If access to the database fails.
    ///
    /// # Returns
    ///
    /// List of all items that are stored in the configured containers.
    ///
    #[instrument(err)]
    pub async fn stored_materials(
        &self,
        pid: ProjectId
    ) -> Result<Vec<ProjectMaterial>, ProjectError> {
        let entries = sqlx::query!(r#"
                SELECT
                    a.location_id,
                    a.reference_id AS "container_id!",
                    a.item_id,
                    a.type_id,
                    a.quantity,
                    i.name,
                    i.group_id
                FROM asset a
                JOIN item i
                  ON a.type_id = i.type_id
                WHERE a.reference_id = ANY(
                    SELECT UNNEST(p.containers) AS container
                    FROM projects p
                    WHERE p.id = $1
                )
                "#,
                    pid
                )
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|x| {
                    ProjectMaterial {
                        group_id:     x.group_id.into(),
                        type_id:      x.type_id.into(),
                        quantity:     x.quantity as i32,
                        name:         x.name,
                        item_id:      Some(x.item_id.into()),
                        container_id: Some(x.container_id.into()),
                        location_id:  Some(x.location_id.into()),
                    }
                })
                .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Gets the raw required materials.
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project
    ///
    /// # Errors
    ///
    /// If the database access fails.
    ///
    /// # Returns
    ///
    /// List of the raw resources that are required to complete the project.
    ///
    #[instrument(err)]
    pub async fn raw_materials(
        &self,
        pid: ProjectId
    ) -> Result<Vec<ProjectMaterial>, ProjectError> {
        /// Temporary struct only used to hold temporary information
        #[derive(Debug)]
        struct ProjectInfo {
            /// Uuid of the project the entry links to
            pid:     Uuid,
            /// [TypeId] of the item that should be produced
            tid:     i32,
            /// Runs required to produce the amount
            runs:    i32,
        }

        // All products that we want to produce
        let project_products = sqlx::query!(r#"
                SELECT
                    pp.project,
                    pp.type_id,
                    pp.count,
                    CEIL(
                        pp.count::FLOAT / bm.quantity::FLOAT
                    ) AS "runs!",
                    bm.quantity AS per_run
                FROM project_products pp
                JOIN blueprint_material bm
                  ON bm.type_id = pp.type_id
                WHERE pp.project = $1
                  AND bm.is_product = TRUE
            "#,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| ProjectInfo {
                pid:  x.project,
                tid:  x.type_id,
                runs: x.runs as i32,
            })
            .collect::<Vec<_>>();

        let stored_blueprints = sqlx::query!(r#"
                SELECT
                    ab.ptype_id            AS "type_id?",
                    ab.material_efficiency AS "me?"
                FROM projects p
                JOIN project_products pp
                  ON pp.project = p.id
                LEFT JOIN asset a
                  ON a.reference_id = ANY(p.containers)
                LEFT JOIN asset_blueprint ab
                  ON a.item_id = ab.item_id
                WHERE p.id = $1
            "#,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .filter(|x| x.type_id.is_some() && x.me.is_some())
            .map(|x| (x.type_id.unwrap(), x.me.unwrap()))
            .collect::<HashMap<_, _>>();

        let mut resources = HashMap::new();
        for product in project_products {
            let mut entries = sqlx::query!(r#"
                    SELECT
                        bf.mtype_id                 AS "type_id!",
                        CEIL(
                            pp.count::FLOAT / bf.produces::FLOAT
                        ) * SUM(bf.quantity)::FLOAT AS "quantity!"
                    FROM blueprint_flat bf
                    JOIN project_products pp ON pp.type_id = bf.type_id
                    JOIN item i ON i.type_id = bf.mtype_id
                    WHERE pp.project = $1
                      AND pp.type_id = $2
                    GROUP BY bf.mtype_id, bf.produces, bf.quantity, pp.count, i.name
                    ORDER BY i.name
                "#,
                    product.pid,
                    product.tid
                )
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|x| x.type_id)
                .collect::<Vec<_>>();
            entries.push(product.tid);

            sqlx::query!(r#"
                    SELECT
                        --bm.ptype_id AS "ptype_id!",
                        bm.quantity AS "produces!",
                        bm.type_id  AS "type_id!",
                        bm.quantity AS "quantity!",
                        i.name      AS "name!",
                        i.group_id  AS "group_id!"
                    FROM blueprint_material bm
                    JOIN item i
                      ON i.type_id = bm.type_id
                    WHERE bm.is_product = FALSE
                      AND bm.ptype_id = ANY($1)
                      AND bm.type_id != ALL($1)
                      AND (bm.activity = 2 OR bm.activity = 3)
                    ORDER BY i.name
                "#,
                    &entries
                )
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .for_each(|x| {
                    let bp = stored_blueprints.get(&product.tid);
                    let quantity = if let Some(me) = bp {
                        let me = *me as f64 / 100f64;
                        let bonus = x.quantity as f64 * me;
                        let bonus = if x.quantity == 1 { 0f64 } else { bonus };

                        let bonus = bonus * product.runs as f64;
                        let quantity = x.quantity as f64 * product.runs as f64 - bonus;

                        // Raitaru, Azbel and Sotiyo have a bonus of 1%
                        let struct_bonus = quantity * 0.01;
                        let struct_bonus = if x.quantity == 1 { 0f64 } else { struct_bonus };
                        let quantity = quantity - struct_bonus;

                        quantity.ceil() as i32
                    } else {
                        product.runs * x.quantity
                    };

                    resources
                        .entry(x.type_id)
                        .and_modify(|x: &mut ProjectMaterial| {
                            x.quantity += quantity
                        })
                        .or_insert(ProjectMaterial {
                            group_id:     x.group_id.into(),
                            type_id:      x.type_id.into(),
                            quantity:     quantity,
                            name:         x.name,
                            ..Default::default()
                        });
                });
        }

        let entries = resources
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Fetches the current market prices for all required raw materials.
    ///
    /// # Params
    ///
    /// * `pid` -> If of the project to calculate the cost
    /// * `sid` -> [Systemid] of the system to fetch the prices from
    ///
    /// # Errors
    ///
    /// If the database access fails
    ///
    /// # Returns
    ///
    /// List of all raw materials that are required for the project and their price.
    ///
    #[instrument(err)]
    pub async fn market_buy_price(
        &self,
        pid: ProjectId,
        sid: SystemId
    ) -> Result<Vec<ProjectMarketItemPrice>, ProjectError> {
        let raw_materials = self.raw_materials(pid).await?;
        let tids = raw_materials
            .iter()
            .map(|x| x.type_id)
            .collect::<Vec<_>>();

        let entries = self.market
            .item_cost_bulk(tids, sid, true)
            .await
            .map_err(ProjectError::MarketError)?
            .into_iter()
            .map(|x| {
                let count = raw_materials
                    .iter()
                    .filter(|y| y.type_id == x.type_id)
                    .map(|x| x.quantity)
                    .sum();

                ProjectMarketItemPrice {
                    a_min: count as f64 * x.min,
                    a_max: count as f64 * x.max,
                    a_avg: count as f64 * x.avg,
                    s_min: x.min,
                    s_max: x.max,
                    s_avg: x.avg,
                    type_id: x.type_id,
                    count: count,
                    name:  x.name
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Fetches the current market prices for all produced materials.
    ///
    /// # Params
    ///
    /// * `pid` -> If of the project to calculate the cost
    /// * `sid` -> [Systemid] of the system to fetch the prices from
    ///
    /// # Errors
    ///
    /// If the database access fails
    ///
    /// # Returns
    ///
    /// List of all produced materials from the project and their price.
    ///
    #[instrument(err)]
    pub async fn market_sell_price(
        &self,
        pid: ProjectId,
        sid: SystemId
    ) -> Result<Vec<ProjectMarketItemPrice>, ProjectError> {
        let products = self.fetch_products(pid).await?;
        let tids = products
            .iter()
            .map(|x| x.type_id)
            .collect::<Vec<_>>();

        let entries = self.market
            .item_cost_bulk(tids, sid, false)
            .await
            .map_err(ProjectError::MarketError)?
            .into_iter()
            .map(|x| {
                let count = products
                    .iter()
                    .filter(|y| y.type_id == x.type_id)
                    .map(|x| x.count)
                    .sum();

                ProjectMarketItemPrice {
                    a_min: count as f64 * x.min,
                    a_max: count as f64 * x.max,
                    a_avg: count as f64 * x.avg,
                    s_min: x.min,
                    s_max: x.max,
                    s_avg: x.avg,
                    type_id: x.type_id,
                    count: count,
                    name:  x.name
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Gets a list  of builsteps to perform for a project.
    ///
    /// # Params
    ///
    /// * `pid`      -> Id of the project
    /// * `activity` -> Activity that should be performed
    ///
    /// # Errors
    ///
    /// If the database access failes.
    ///
    /// # Returns
    ///
    /// List of all buildsteps for the given action.
    ///
    pub async fn buildsteps(
        &self,
        pid:      ProjectId,
        activity: Activity
    ) -> Result<Vec<ProjectBuildstep>, ProjectError> {
        // TODO: time should contain the number of required runs
        // TODO: count the needed runs

        let entries = sqlx::query!("
                SELECT
                    i.name,
                    bf.mtype_id,
                    b.manufacture,
                    b.reaction
                FROM project_products pp
                JOIN blueprint_flat bf
                  ON bf.type_id = pp.type_id
                JOIN blueprint_material bm
                  ON bm.ptype_id = bf.mtype_id
                JOIN blueprint b
                  ON b.id = bm.blueprint
                JOIN item i ON bf.mtype_id = i.type_id
                WHERE pp.project = $1
                  AND bm.is_product = TRUE
                  AND bm.activity = $2
                ORDER BY bf.mtype_id
            ",
                pid,
                activity.as_i16()
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                let time = match activity {
                    Activity::Manufacture => x.manufacture,
                    Activity::Reaction    => x.reaction,
                }
                .unwrap_or_default();

                ProjectBuildstep {
                    name:    x.name,
                    type_id: x.mtype_id.into(),
                    time:    time,
                    runs:    0
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Gets a list of all costs that where added by a user.
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project
    ///
    /// # Errors
    ///
    /// When the database access fails
    ///
    /// # Returns
    ///
    /// All costs recorded by a user.
    ///
    #[instrument(err)]
    pub async fn trackings(
        &self,
        pid: ProjectId
    ) -> Result<Vec<ProjectCostTracking>, ProjectError> {
        let entries = sqlx::query!("
                SELECT *
                FROM project_trackings
                WHERE project = $1
                ORDER BY created_at ASC
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                ProjectCostTracking {
                    id:          x.id,
                    character:   x.character.into(),
                    amount:      x.amount,
                    description: x.description,
                    created_at:  x.created_at.timestamp_millis()
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Adds a new tracking entry.
    ///
    /// # Params
    ///
    /// * `pid`  -> Project the cost should link to
    /// * `body` -> Cost information
    ///
    /// # Errors
    ///
    /// When the database access fails.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    #[instrument(err)]
    pub async fn add_tracking(
        &self,
        pid: ProjectId,
        body: ProjectAddCostTracking
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                INSERT INTO project_trackings
                (
                    project,
                    character,
                    amount,
                    description
                )
                VALUES ($1, $2, $3, $4)
            ",
                pid,
                *body.character,
                body.amount,
                body.description
            )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Edits a tracking entry.
    ///
    /// # Params
    ///
    /// * `pid`  -> Project the cost should link to
    /// * `tid`  -> Tracking id of an entry
    /// * `body` -> Cost information
    ///
    /// # Errors
    ///
    /// When the database access fails.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    #[instrument(err)]
    pub async fn edit_tracking(
        &self,
        pid:  ProjectId,
        tid:  TrackingId,
        body: ProjectCostTracking
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                UPDATE project_trackings
                SET
                    character = $1,
                    amount = $2,
                    description = $3
                WHERE id = $4
            ",
                *body.character,
                body.amount,
                body.description,
                body.id
            )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Delets a tracking entry.
    ///
    /// # Params
    ///
    /// * `pid`  -> Project the cost should link to
    /// * `tid`  -> Tracking id of an entry
    ///
    /// # Errors
    ///
    /// When the database access fails.
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    #[instrument(err)]
    pub async fn delete_tracking(
        &self,
        pid:  ProjectId,
        tid:  TrackingId,
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                DELETE FROM project_trackings
                WHERE id = $1
            ",
                tid
            )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Fetches all products for a project.
    ///
    /// # Params
    ///
    /// * `pid` -> [ProjectId] of the project to fetch the products for
    ///
    /// # Errors
    ///
    /// When the database connection fails.
    ///
    /// # Returns
    ///
    /// List of all products and there count.
    ///
    #[instrument(err)]
    async fn fetch_products(
        &self,
        pid: ProjectId
    ) -> Result<Vec<ProjectProduct>, ProjectError> {
        let entries = sqlx::query!("
                SELECT pp.*, i.name
                FROM project_products pp
                JOIN item i
                  ON pp.type_id = i.type_id
                WHERE pp.project = $1
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| ProjectProduct {
                name:    x.name,
                count:   x.count,
                type_id: x.type_id.into(),
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Inserts a list of products into the database.
    /// If an item is already linked with the project, the count gets
    /// overwriten with the new value.
    ///
    /// # Params
    ///
    /// * `pid`      -> [ProjectId] the products should link to
    /// * `products` -> List of [ProjectProduct] that should be added
    ///
    /// # Errors
    ///
    /// When there was an error while inserting the entries
    ///
    /// # Returns
    ///
    /// Nothing.
    ///
    #[instrument(err)]
    async fn insert_products(
        &self,
        pid:      ProjectId,
        products: Vec<ProjectProductConfig>,
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                 DELETE FROM project_products
                 WHERE project = $1
            ",
                pid
            )
            .execute(&self.pool)
            .await?;

        let type_ids = products
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let counts = products
            .iter()
            .map(|x| x.count)
            .collect::<Vec<_>>();
        sqlx::query!("
                INSERT INTO project_products
                (
                    project,
                    type_id,
                    count
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
                ON CONFLICT (project, type_id)
                DO UPDATE SET count = EXCLUDED.count
            ",
                pid,
                &type_ids,
                &counts
            )
            .execute(&self.pool)
            .await
            .map(drop)
            .map_err(ProjectError::DatabaseError)
    }

}

impl std::fmt::Debug for ProjectService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectService").finish()
    }
}

/// Contains all errors that can happen in this service
#[derive(Debug)]
pub enum ProjectError {
    /// Generic database error during execution
    DatabaseError(sqlx::Error),
    /// Error while parsing json
    SerdeError(serde_json::Error),
    /// Errors from the [MarketService]
    MarketError(crate::MarketError),
    /// An entry was not found
    NotFound
}

crate::error_derive!(ProjectError);

/// Represents a single project
#[derive(Debug, Serialize)]
pub struct Project {
    /// Unique identifier for the project
    pub id:         Uuid,
    /// Every project belongs to exactly one person
    pub owner:      CharacterId,
    /// Name of the project
    pub name:       String,
    /// Containers that are linked with this project
    pub containers: Vec<ItemId>,
    /// List of all products that should be created
    pub products:   Vec<ProjectProduct>
}

/// Represents basic information about a project
#[derive(Debug, Serialize)]
pub struct ProjectInfo {
    /// Id of the project
    pub id:     ProjectId,
    /// Project name
    pub name:   String,
    /// Determines if the project is shown in the sidebar or not
    pub pinned: bool,
    /// Current project status
    pub status: ProjectStatus
}

/// Determines what status a project currently has
#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    /// The project is done
    Done,
    /// The project is currently in progress
    InProgress,
    /// The project is currently not active
    Halted
}

/// Represents a product that is build within the project
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectProduct {
    /// Name of the product
    pub name:    String,
    /// Number of items that should be build
    pub count:   i32,
    /// TypeId of the product
    pub type_id: TypeId,
}

/// Represents an blueprint from a product
#[derive(Debug, Serialize)]
pub struct ProjectBlueprint {
    /// [TypeId] of the blueprint
    pub type_id:     TypeId,
    /// Name of the blueprint
    pub name:         String,

    // The following entries are only available if the user has those items
    /// True if the blueprint is an original otherwise false
    pub original:     Option<bool>,
    /// Number of runs remaining
    pub runs:         Option<i32>,
    /// Material efficiency of the blueprint
    pub material_eff: Option<i32>,
    /// Time efficiency of the blueprint
    pub time_eff:     Option<i32>,
    /// [ItemId] of the container the blueprint is located in
    pub container_id: Option<ItemId>,
    /// Location of the container
    pub location_id:  Option<LocationId>
}

/// Holds the prices for an item
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMarketItemPrice {
    /// Cost of all items calculated with the min value
    a_min:   f64,
    /// Cost of all items calculated with the max value
    a_max:   f64,
    /// Cost of all items calculated with the avg value
    a_avg:   f64,
    /// Cost of a single item calculated with the min value
    s_min:   f64,
    /// Cost of a single item calculated with the max value
    s_max:   f64,
    /// Cost of a single item calculated with the avg value
    s_avg:   f64,
    /// Required amount
    count:   i32,
    /// Id of the item
    type_id: TypeId,
    /// Name of the item
    name:    String,
}

/// Contains all information for creating a new project
#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    /// Name of the project
    pub name:       String,
    /// List of all containers that are linked with this project
    pub containers: Vec<ItemId>,
    /// List of all products that should be build
    pub products:   Vec<ProjectProductConfig>
}

/// Configuration for a product
#[derive(Debug, Deserialize)]
pub struct ProjectProductConfig {
    /// Number of items that should be build
    pub count:   i32,
    /// TypeId of the product
    pub type_id: TypeId,
}

impl From<ProjectProduct> for ProjectProductConfig {
    fn from(x: ProjectProduct) -> Self {
        Self {
            count:   x.count,
            type_id: x.type_id
        }
    }
}

/// Represents a single stored material
#[derive(Debug, Serialize)]
pub struct ProjectMaterial {
    /// Group of the item
    pub group_id:     GroupId,
    /// [TypeId] of the item that is stored
    pub type_id:      TypeId,
    /// Number of items that are stored in this container
    pub quantity:     i32,
    /// Name of the item that is stored
    pub name:         String,

    /// [ItemId] if the item itself
    pub item_id:      Option<ItemId>,
    /// [ItemId] of the container the item is stored in
    pub container_id: Option<ItemId>,
    /// [LocationId] of the container
    pub location_id:  Option<LocationId>,
}

impl Default for ProjectMaterial {
    fn default() -> Self {
        Self {
            group_id:     0.into(),
            type_id:      0.into(),
            quantity:     0,
            name:         String::new(),
            item_id:      None,
            container_id: None,
            location_id:  None,
        }
    }
}

/// Complete tree of a blueprint and all its required materials
#[derive(Debug, Deserialize, Serialize)]
pub struct BlueprintTree {
    /// [TypeId] of the current item
    pub key:      TypeId,
    /// Name if the item
    pub label:    String,
    /// Number of resources needed
    pub quantity: i32,
    /// Required items to build the item
    pub children: Option<Vec<BlueprintTree>>
}

/// Contains all valid activities
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
    /// Manufacture activity
    Manufacture,
    /// Reaction activity
    Reaction
}

impl Activity {
    /// Every activity has an specific number.
    ///
    /// # Returns
    ///
    /// Number value of the activity
    pub fn as_i16(&self) -> i16 {
        match self {
            Self::Manufacture => 2i16,
            Self::Reaction    => 3i16
        }
    }
}

/// Represents a single buildstep
#[derive(Debug, Serialize)]
pub struct ProjectBuildstep {
    /// Name of the item
    pub name:    String,
    /// TypeId of the item to produce
    pub type_id: TypeId,
    /// Time it takes for a single run
    pub time:    i32,
    /// Number of required runs
    pub runs:    i32,
}

/// Represents a single cost tracking
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCostTracking {
    /// Unique id of the tracking entry
    pub id:          TrackingId,
    /// Cost amount
    pub amount:      f64,
    /// Short description for what the cost was
    pub description: String,
    /// User that created this cost
    pub character:   CharacterId,
    /// Timestamp when this tracking was created
    pub created_at:  i64,
}

/// Represents a single cost tracking
#[derive(Debug, Deserialize)]
pub struct ProjectAddCostTracking {
    /// User that created this cost
    pub character:   CharacterId,
    /// Cost amount
    pub amount:      f64,
    /// Short description for what the cost was
    pub description: String,
}

