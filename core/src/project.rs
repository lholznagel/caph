use crate::MarketService;

use caph_connector::{CharacterId, GroupId, TypeId, SystemId, AllianceId, CorporationId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::{HashMap, VecDeque};
use tracing::instrument;
use uuid::Uuid;

/// An id of a tracking entry
pub type BudgetId    = Uuid;
/// Id of a virtual container
pub type ContainerId = Uuid;
/// A project id is just a UUID, this type is just for clarification
pub type ProjectId   = Uuid;

#[derive(Clone, Debug)]
struct ManufactureCacheEntry {
    quantity_manufacture: i64,
    mtype_id:             TypeId,
    quantity_material:    i64,
}

impl ManufactureCacheEntry {
    /// Calculates the number of required runs.
    /// 
    /// # Params
    /// 
    /// * `quantity` -> Required quantity of the item
    /// 
    /// # Returns
    /// 
    /// Number of runs requuired to fullfil the quantity.
    /// 
    pub fn required(&self, quantity: i64) -> i64 {
        (
            (
                quantity as f32 / self.quantity_manufacture as f32
            ) * self.quantity_material as f32
        ).ceil() as i64
    }
}

/// Wrapper for managing projects
/// TODO: split file into multiple files
#[derive(Clone)]
pub struct ProjectService {
    /// Database pool
    pool:   PgPool,
    /// [MarketService] for handling market requests
    market: MarketService,
    /// Cache for all blueprints
    cache: HashMap<TypeId, Vec<ManufactureCacheEntry>>
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
            market,
            cache: HashMap::new()
        }
    }

    /// Populates the blueprint manufacture cache.
    /// 
    /// # Errors
    /// 
    /// If there is a problem with accessing the database.
    /// 
    /// # Returns
    /// 
    /// Nothing
    pub async fn populate_cache(&mut self) -> Result<(), ProjectError> {
        sqlx::query!(r#"
                SELECT
                    bman.ptype_id AS "ptype_id!",
                    bman.quantity AS "quantity_manufacture!",
                    bm.mtype_id   AS "mtype_id!",
                    bm.quantity   AS "quantity_material!"
                FROM blueprint_manufacture bman
                JOIN blueprint_materials bm
                  ON bm.bp_id = bman.bp_id
            "#)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .for_each(|x| {
                let entry = ManufactureCacheEntry {
                    quantity_manufacture: x.quantity_manufacture,
                    mtype_id:             x.mtype_id.into(),
                    quantity_material:    x.quantity_material,
                };

                self.cache
                    .entry(x.ptype_id.into())
                    .and_modify(|e| e.push(entry.clone()))
                    .or_insert(vec![entry]);
            });

        Ok(())
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
        let entry = sqlx::query!(r#"
                SELECT
                    p.project,
                    p.owner,
                    p.name,
                    p.status AS "status: Status"
                FROM projects p
                JOIN project_products pp
                  ON p.project = pp.project
                WHERE p.project = $1
            "#,
                pid
            )
            .fetch_optional(&self.pool)
            .await?
            .map(|x| {
                Project {
                    project:     x.project,
                    owner:       x.owner.into(),
                    name:        x.name,
                    status:      x.status,
                    products:    Vec::new(),
                }
            });

        if let Some(mut x) = entry {
            x.products = self.fetch_products(pid).await?;
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }

    /// Minimal version of all projects that the user has access to.
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
    ) -> Result<Vec<Info>, ProjectError> {
        let entries = sqlx::query!(r#"
                SELECT
                    p.project,
                    p.name,
                    p.owner,
                    p.status AS "status: Status"
                FROM project_members pm
                JOIN projects p
                  ON p.project = pm.project
                WHERE character_id = $1
                ORDER BY name
            "#,
                *cid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| Info {
                project: x.project,
                name:    x.name,
                owner:   x.owner.into(),
                status:  x.status
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Creates a new project.
    ///
    /// # Params
    ///
    /// * `cid` -> [CharacterId] of the user that create the project
    /// * `cfg` -> Holds all information needed to create a project
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
        cid: CharacterId,
        cfg: Config
    ) -> Result<ProjectId, ProjectError> {
        let pid = sqlx::query!("
                INSERT INTO projects
                (
                    owner,
                    name
                )
                VALUES ($1, $2)
                RETURNING project
            ",
                *cid,
                cfg.name,
            )
            .fetch_one(&self.pool)
            .await
            .map(|x| x.project)?;

        sqlx::query!("
                INSERT INTO project_members
                (
                    project,
                    character_id
                )
                VALUES($1, $2)
            ",
                pid,
                *cid
            )
            .execute(&self.pool)
            .await?;

        self.insert_products(pid, cfg.products).await?;
        Ok(pid)
    }

    /// Updates the given entry with the given data.
    ///
    /// # Params
    ///
    /// * `pid` -> [ProjectId] of the project to update
    /// * `cfg` -> Updated info for the project
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
        pid: ProjectId,
        cfg: Config
    ) -> Result<ProjectId, ProjectError> {
        sqlx::query!("
                UPDATE projects
                   SET name = $2,
                       status = $3
                WHERE project = $1
            ",
                pid,
                cfg.name,
                cfg.status as _
            )
            .execute(&self.pool)
            .await?;
        self.insert_products(pid, cfg.products).await?;
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
                WHERE project = $1
                RETURNING project
            ",
                pid
            )
            .fetch_optional(&self.pool)
            .await?
            .map(|x| x.project);
        Ok(entry)
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
    ) -> Result<Vec<Material>, ProjectError> {
        let entries = sqlx::query!(r#"
                    SELECT
                        pa.type_id,
                        pa.quantity,
                        i.name,
                        i.group_id
                    FROM project_assets pa
                    JOIN items i
                      ON i.type_id = pa.type_id
                    WHERE pa.project = $1
                    ORDER BY i.name
                "#,
                    pid
                )
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|x| {
                    Material {
                        type_id:  x.type_id.into(),
                        quantity: x.quantity as i64,
                        name:     x.name,
                        group_id: x.group_id.into(),
                    }
                })
                .collect::<Vec<_>>();
        Ok(entries)
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
    /// List of all blueprints that are required for the project.
    /// TODO: review, blueprint_flat and blueprint_material are deprecated
    #[instrument(err)]
    pub async fn required_blueprints(
        &self,
        pid: ProjectId
    ) -> Result<Vec<Blueprint>, ProjectError> {
        let steps = self
            .buildstep_manufacturing(pid)
            .await?
            .into_iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let bps = sqlx::query!(r#"
                SELECT
                    bman.btype_id AS "btype_id!",
                    i.name        AS "name!"
                FROM blueprint_manufacture bman
                JOIN items i
                  ON i.type_id = bman.btype_id
                WHERE bman.ptype_id = ANY($1)
            "#,
                &steps
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| Blueprint {
                type_id:        x.btype_id.into(),
                name:           x.name,
                is_reaction:    true,
                is_manufacture: true,
                iters:          0
            })
            .collect::<Vec<_>>();
        Ok(bps)
    }

    /// Fetches a list of all required blueprints and checks if they are stored.
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
    /// List of all stored blueprints for the project and additional information.
    /// TODO: reimplement when storage is done
    #[instrument(err)]
    pub async fn info_blueprints(
        &self,
        pid: ProjectId
    ) -> Result<Vec<BlueprintInfo>, ProjectError> {
        Ok(Vec::new())
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
    ) -> Result<Vec<Material>, ProjectError> {
        let mut materials = HashMap::new();

        self.buildstep_manufacturing(pid)
            .await?
            .into_iter()
            .for_each(|b| {
                b.materials
                    .into_iter()
                    .for_each(|m| {
                        materials
                            .entry(m.type_id)
                            .and_modify(|x: &mut Material| x.quantity += m.quantity)
                            .or_insert(m);
                    });
            });
        let mut materials = materials
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        materials.sort_by_key(|x| x.group_id);
        Ok(materials)
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
                let count: i64 = raw_materials
                    .iter()
                    .filter(|y| y.type_id == x.type_id)
                    .map(|x| x.quantity)
                    .sum();

                ProjectMarketItemPrice {
                    a_min:   count as f64 * x.min,
                    a_max:   count as f64 * x.max,
                    a_avg:   count as f64 * x.avg,
                    s_min:   x.min,
                    s_max:   x.max,
                    s_avg:   x.avg,
                    type_id: x.type_id,
                    count:   count,
                    name:    x.name
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
                let count: i64 = products
                    .iter()
                    .filter(|y| y.type_id == x.type_id)
                    .map(|x| x.count as i64)
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

    /// Gets all required buildsteps that are required to build the project.
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project
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
    ) -> Result<Buildstep, ProjectError> {
        let buildstep = Buildstep {
            manufacture: self.buildstep_manufacturing(pid).await?,
            inventions:  self.buildstep_invention(pid).await?
        };
        Ok(buildstep)
    }

    /// Gets a list of either all manufacture jobs or all reaction jobs.
    ///
    /// # Params
    ///
    /// * `pid`      -> Id of the project
    /// * `activity` -> [Activity], either manufacture or reaction
    ///
    /// # Errors
    ///
    /// If the database fails.
    ///
    /// # Returns
    ///
    /// List of the buildsteps.
    /// TODO: extract and refactor to seperate function
    async fn buildstep_manufacturing(
        &self,
        pid:      ProjectId,
    ) -> Result<Vec<BuildstepEntry>, ProjectError> {
        // 1. Get the base items and calculate the number of runs
        let mut products = HashMap::new();
        sqlx::query!(r#"
                SELECT
                    bmc.ptype_id,
                    bmc.quantity AS product_quantity,
                    bmc.time,
                    bmc.reaction,
                    CEIL(
                        pp.count::FLOAT / bmc.quantity::FLOAT
                    )::INTEGER   AS "runs!",
                    i.name       AS product_name,
                    i.group_id   AS produt_group_id,
                    bm.mtype_id,
                    bm.quantity  AS material_quantity,
                    ii.name      AS material_name,
                    ii.group_id  AS material_group_id
                FROM project_products pp
                JOIN blueprint_manufacture bmc
                  ON bmc.ptype_id = pp.type_id
                JOIN blueprint_materials bm
                  ON bm.bp_id = bmc.bp_id
                JOIN items i
                  ON i.type_id = bmc.ptype_id
                JOIN items ii
                  ON ii.type_id = bm.mtype_id
                WHERE pp.project = $1
            "#,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .for_each(|x| {
                let material = Material {
                    type_id:  x.mtype_id.into(),
                    quantity: x.runs as i64 * x.material_quantity,
                    name:     x.material_name,
                    group_id: x.material_group_id.into()
                };
                products
                    .entry(x.ptype_id)
                    .and_modify(|x: &mut BuildstepEntry| x.materials.push(material.clone()))
                    .or_insert(BuildstepEntry {
                        name:         x.product_name,
                        type_id:      x.ptype_id.into(),
                        time_per_run: x.time,
                        runs:         x.runs,
                        time_total:   x.runs * x.time,
                        produces:     x.runs as i64 * x.product_quantity,
                        materials:    vec![material],
                    });
            });
        let products = products
            .values()
            .cloned()
            .collect::<Vec<_>>();

        // 2. Resolve the required materials and calculate the number of
        // required materials.
        let mut materials = HashMap::new();
        for product in products.iter() {
            let a = self.no_clue(product.type_id, product.produces).await?;
            for (t, q) in a {
                materials
                    .entry(t)
                    .and_modify(|x: &mut i64| *x += q)
                    .or_insert(q);
            }
        }

        // 3. Collect all components together and join them with the required amount
        let mut component_materials = HashMap::new();
        sqlx::query!(r#"
                SELECT
                    bman.ptype_id  AS "ptype_id!",
                    bman.quantity  AS produces,
                    bman.time,
                    i.name         AS product_name,
                    bmat.mtype_id,
                    bmat.quantity  AS required,
                    ii.name        AS material_name,
                    ii.group_id    AS material_group_id
                FROM blueprint_manufacture bman
                JOIN blueprint_materials bmat
                  ON bmat.bp_id = bman.bp_id
                JOIN items i
                  ON i.type_id = bman.ptype_id
                JOIN items ii
                  ON ii.type_id = bmat.mtype_id
                WHERE bman.ptype_id = ANY(
                    SELECT bmatc.mtype_id
                    FROM blueprint_manufacture_components bmanc
                    JOIN blueprint_materials bmatc ON bmatc.bp_id = bmanc.bp_id
                    WHERE bmanc.ptype_id = ANY(
                        SELECT type_id
                        FROM project_products
                        WHERE project = $1
                    )
                )
            "#,
                pid,
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .for_each(|x| {
                let quantity = materials
                    .get(&x.ptype_id.into())
                    .cloned()
                    .unwrap_or_default();
                let runs = (quantity as f32 / x.produces as f32).ceil() as i32;

                let material = Material {
                    type_id:  x.mtype_id.into(),
                    quantity: runs as i64 * x.required,
                    name:     x.material_name,
                    group_id: x.material_group_id.into()
                };

                component_materials
                    .entry(x.ptype_id)
                    .and_modify(|e: &mut BuildstepEntry| {
                        e.materials.push(material.clone());
                    })
                    .or_insert(BuildstepEntry {
                        type_id:      x.ptype_id.into(),
                        name:         x.product_name,
                        time_per_run: x.time,
                        materials:    vec![material],
                        time_total:   runs * x.time,
                        produces:     runs as i64 * x.produces,
                        runs:         runs,
                    });
           });

        let unordered = component_materials
            .values()
            .into_iter()
            .map(|x| {
                let materials = x.materials
                    .iter()
                    .map(|x| x.type_id)
                    .collect::<Vec<_>>();
                (x.type_id, materials)
            })
            .collect::<Vec<_>>();
        let mut sorted_steps = Vec::new();
        let order = ManufactureOrder(unordered).sort();

        for tid in order {
            let entry = if let Some(x) = component_materials.get(&tid) {
                x.clone()
            } else {
                continue;
            };
            sorted_steps.push(entry);
        }
        sorted_steps.extend(products);
        Ok(sorted_steps)
    }

    /// TODO: proper name, refactor, implement caching
    async fn no_clue(
        &self,
        ptype_id: TypeId,
        required: i64
    ) -> Result<HashMap<TypeId, i64>, ProjectError> {
        let mut quantities = HashMap::new();

        let mut queue: VecDeque<(i32, i64)> = VecDeque::new();
        queue.push_back((*ptype_id, required));
        while let Some((t, q)) = queue.pop_front() {
            if let Some(x) = self.cache.get(&t.into()) {
                for y in x {
                    let required = y.required(q);
                    queue.push_back((*y.mtype_id, required));
                    quantities
                        .entry(y.mtype_id)
                        .and_modify(|e: &mut i64| *e += required)
                        .or_insert(required);
                }
            }
        }

        Ok(quantities)
    }

    /// Creates a list of all inventions that are required for a the given
    /// project.
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project
    ///
    /// # Errors
    ///
    /// If the database access fails
    ///
    /// # Returns
    ///
    /// List of all inventions and their required datacores
    ///
    async fn buildstep_invention(
        &self,
        pid: ProjectId
    ) -> Result<Vec<BuildstepEntryInvention>, ProjectError> {
        let mut inventions = HashMap::new();

        sqlx::query!("
                SELECT
                    bi.itype_id,
                    bi.time,
                    bi.probability,
                    bim.mtype_id,
                    bim.quantity,
                    i.name      AS iname, -- item name
                    ii.name     AS dname,  -- datacore name
                    ii.group_id AS dgroup_id -- datacore group id
                FROM project_products pp
                JOIN blueprint_inventions bi
                  ON bi.ttype_id = pp.type_id
                JOIN blueprint_materials bim
                  ON bim.bp_id = bi.bp_id
                JOIN items i
                  ON i.type_id = bi.itype_id
                JOIN items ii
                  ON ii.type_id = bim.mtype_id
                WHERE pp.project = $1
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .for_each(|x| {
                inventions
                    .entry(x.itype_id)
                    .and_modify(|e: &mut BuildstepEntryInvention| {
                        e.materials.push(Material {
                            type_id:  x.mtype_id.into(),
                            quantity: x.quantity,
                            name:     x.dname.clone(),
                            group_id: x.dgroup_id.into(),
                        });
                    })
                    .or_insert({
                        let material = Material {
                            type_id:  x.mtype_id.into(),
                            quantity: x.quantity,
                            name:     x.dname,
                            group_id: x.dgroup_id.into(),
                        };

                        BuildstepEntryInvention {
                            name:         x.iname,
                            time_per_run: x.time,
                            time_total:   x.time,
                            type_id:      x.itype_id.into(),
                            probability:  x.probability,
                            materials:    vec![material]
                        }
                    });
            });
        let inventions = inventions
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(inventions)
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
    pub async fn budget(
        &self,
        pid: ProjectId
    ) -> Result<Vec<BudgetEntry>, ProjectError> {
        let entries = sqlx::query!(r#"
                SELECT
                    budget,
                    project,
                    character,
                    amount,
                    created_at,
                    category    AS "category: BudgetCategory",
                    description
                FROM project_budget
                WHERE project = $1
                ORDER BY created_at ASC
            "#,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                BudgetEntry {
                    budget:      x.budget,
                    character:   x.character.into(),
                    amount:      x.amount,
                    category:    x.category,
                    created_at:  x.created_at.timestamp_millis(),

                    description: x.description,
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Gets a single budget entry.
    ///
    /// # Params
    ///
    /// * `pid` -> Id of the project
    /// * `bid` -> Id of the budget
    ///
    /// # Errors
    ///
    /// When the database access fails
    ///
    /// # Returns
    ///
    /// Single budget entry
    ///
    #[instrument(err)]
    pub async fn budget_entry(
        &self,
        pid: ProjectId,
        bid: BudgetId,
    ) -> Result<Option<BudgetEntry>, ProjectError> {
        let entries = sqlx::query!(r#"
                SELECT
                    budget,
                    project,
                    character,
                    amount,
                    created_at,
                    category    AS "category: BudgetCategory",
                    description
                FROM project_budget
                WHERE project = $1
                  AND budget = $2
                ORDER BY created_at ASC
            "#,
                pid,
                bid
            )
            .fetch_optional(&self.pool)
            .await?
            .map(|x| {
                BudgetEntry {
                    budget:      x.budget,
                    character:   x.character.into(),
                    amount:      x.amount,
                    category:    x.category,
                    created_at:  x.created_at.timestamp_millis(),

                    description: x.description,
                }
            });
        Ok(entries)
    }

    /// Adds a new tracking entry.
    ///
    /// # Params
    ///
    /// * `pid`   -> Project the cost should link to
    /// * `entry` -> Cost information
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
    pub async fn add_budget_entry(
        &self,
        pid:   ProjectId,
        entry: AddBudgetEntry
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                INSERT INTO project_budget
                (
                    project,
                    character,
                    amount,
                    category,
                    description
                )
                VALUES ($1, $2, $3, $4, $5)
            ",
                pid,
                *entry.character,
                entry.amount,
                entry.category as _,
                entry.description
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
    /// * `bid`  -> Budget id of an entry
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
    pub async fn edit_budget_entry(
        &self,
        pid:   ProjectId,
        bid:   BudgetId,
        entry: BudgetEntry
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                UPDATE project_budget
                SET
                    character = $1,
                    amount = $2,
                    description = $3
                WHERE budget = $4
            ",
                *entry.character,
                entry.amount,
                entry.description,
                entry.budget
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
    /// * `bid`  -> Budget id of an entry
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
    pub async fn delete_budget_entry(
        &self,
        pid:  ProjectId,
        bid:  BudgetId,
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                DELETE FROM project_budget
                WHERE budget = $1
            ",
                bid
            )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Fetches all members of a project
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
    /// List of all memebers that are part of the project
    ///
    #[instrument(err)]
    pub async fn members(
        &self,
        pid: ProjectId
    ) -> Result<Vec<Member>, ProjectError> {
        sqlx::query!("
                SELECT
                    c.character_id,
                    c.character_name,
                    c.corporation_id,
                    c.corporation_name,
                    c.alliance_id,
                    c.alliance_name
                FROM project_members pm
                JOIN characters c
                  ON c.character_id = pm.character_id
                WHERE pm.project = $1
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
            .map(|x| {
                x.into_iter()
                    .map(|x| Member {
                        character_id:     x.character_id.into(),
                        character_name:   x.character_name,
                        corporation_id:   x.corporation_id.into(),
                        corporation_name: x.corporation_name,
                        alliance_id:      x.alliance_id.map(|x| x.into()),
                        alliance_name:    x.alliance_name,
                    })
                    .collect::<Vec<_>>()
            })
    }

    /// Adds a character to a project.
    ///
    /// # Params
    ///
    /// * `pid` -> [ProjectId] of the project
    /// * `cid` -> [CharacterId] of the new character
    ///
    /// # Errors
    ///
    /// When the database connection fails.
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    #[instrument(err)]
    pub async fn add_member(
        &self,
        pid: ProjectId,
        cid: CharacterId
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                INSERT INTO project_members
                (
                    project,
                    character_id
                )
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
            ",
                pid,
                *cid
            )
            .execute(&self.pool)
            .await
            .map(drop)
            .map_err(Into::into)
    }

    /// Kicks a member from a project.
    ///
    /// # Params
    ///
    /// * `pid` -> [ProjectId] of the project
    /// * `cid` -> [CharacterId] of the character to kick
    ///
    /// # Errors
    ///
    /// When the database connection fails.
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    #[instrument(err)]
    pub async fn kick_member(
        &self,
        pid: ProjectId,
        cid: CharacterId
    ) -> Result<(), ProjectError> {
        sqlx::query!("
                DELETE FROM project_members
                WHERE project = $1
                  AND character_id = $2
            ",
                pid,
                *cid
            )
            .execute(&self.pool)
            .await
            .map(drop)
            .map_err(Into::into)
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
    ) -> Result<Vec<Product>, ProjectError> {
        let entries = sqlx::query!("
                SELECT pp.*, i.name
                FROM project_products pp
                JOIN items i
                  ON pp.type_id = i.type_id
                WHERE pp.project = $1
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| Product {
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
        products: Vec<ProductConfig>,
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
    pub project:     ProjectId,
    /// Every project belongs to exactly one person
    pub owner:       CharacterId,
    /// Name of the project
    pub name:        String,
    /// Status of the project
    pub status:      Status,
    /// All projects that should be produced in this project
    pub products:    Vec<Product>,
}

/// Represents basic information about a project
#[derive(Debug, Serialize)]
pub struct Info {
    /// Id of the project
    pub project: ProjectId,
    /// Project name
    pub name:    String,
    /// Owner of the project
    pub owner:   CharacterId,
    /// Current project status
    pub status:  Status
}

/// Contains all information for creating a new project
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Name of the project
    pub name:       String,
    /// List of all products that should be build
    pub products:   Vec<ProductConfig>,
    /// Status of the project
    pub status:     Option<Status>
}

/// Represents a product that is build within the project
#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    /// Name of the product
    pub name:    String,
    /// Number of items that should be build
    pub count:   i32,
    /// TypeId of the product
    pub type_id: TypeId,
}

/// Determines what status a project currently has
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// The project is aborted
    Aborted,
    /// The project is finished
    Done,
    /// The project is currently in progress
    InProgress,
    /// The project is currently paused
    Paused
}

/// Determines what status a project currently has
#[derive(Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "PROJECT_BUDGET_CATEGORY")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BudgetCategory {
    /// Budget category that contains all purchases of materials
    Purchase,
    /// Budget category that contains all wins
    Sold,
    /// Budget category that contains all manufacture related costs
    Manufacture,
    /// Budget category that contains all research related costs
    Research,
    /// Undefined budget category
    Other
}

/// Represents a single stored material
#[derive(Clone, Debug, Serialize)]
pub struct Material {
    /// [TypeId] of the item that is stored
    pub type_id:  TypeId,
    /// Number of items that are stored in this container
    pub quantity:  i64,
    /// Name of the item that is stored
    pub name:     String,
    /// Group of the item
    pub group_id: GroupId,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            type_id:  0.into(),
            quantity: 0,
            name:     String::new(),
            group_id: 0.into(),
        }
    }
}

/// Represents an blueprint from a product
#[derive(Clone, Debug, Serialize)]
pub struct Blueprint {
    /// [TypeId] of the blueprint
    pub type_id:       TypeId,
    /// Name of the blueprint
    pub name:           String,
    /// True if the blueprint is a reaction blueprint
    pub is_reaction:    bool,
    /// True if the blueprint is a manufacture blueprint
    pub is_manufacture: bool,
    /// Number of blueprint iterations needed
    pub iters:          i32,
}

/// Represents blueprint information for a required blueprint
#[derive(Debug, Serialize)]
pub struct BlueprintInfo {
    /// [TypeId] of the blueprint
    pub type_id:      TypeId,
    /// True if the blueprint is an original otherwise false
    pub original:     bool,
    /// Number of runs remaining
    pub runs:         i32,
    /// Material efficiency of the blueprint
    pub material_eff: i32,
    /// Time efficiency of the blueprint
    pub time_eff:     i32,
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
    count:   i64,
    /// Id of the item
    type_id: TypeId,
    /// Name of the item
    name:    String,
}

/// Configuration for a product
#[derive(Debug, Deserialize)]
pub struct ProductConfig {
    /// Number of items that should be build
    pub count:   i32,
    /// TypeId of the product
    pub type_id: TypeId,
}

impl From<Product> for ProductConfig {
    fn from(x: Product) -> Self {
        Self {
            count:   x.count,
            type_id: x.type_id
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

/// Represents a single buildstep
#[derive(Debug, Serialize)]
pub struct Buildstep {
    /// Contains all required manufactures
    pub manufacture: Vec<BuildstepEntry>,
    /// Contains all required inventions
    pub inventions:  Vec<BuildstepEntryInvention>
}

/// Represents a buildstep of a blueprint
#[derive(Clone, Debug, Serialize)]
pub struct BuildstepEntry {
    /// Name of the blueprint
    pub name:         String,
    /// Time it takes for one run
    pub time_per_run: i32,
    /// Total time it takes to run all runs
    pub time_total:   i32,
    /// [TypeId] of the blueprint
    pub type_id:      TypeId,
    /// Number of runs
    pub runs:         i32,
    /// Count of item that is produced by all runs
    pub produces:     i64,
    /// Cores that are required for invention
    pub materials:    Vec<Material>
}

/// Represents a buildstep of a invention
#[derive(Debug, Serialize)]
pub struct BuildstepEntryInvention {
    /// Name of the blueprint
    pub name:         String,
    /// Time it takes for one run
    pub time_per_run: i32,
    /// Total time it takes to run all runs
    pub time_total:   i32,
    /// [TypeId] of the blueprint
    pub type_id:      TypeId,
    /// Number of runs
    pub probability:  f64,
    /// Cores that are required for invention
    pub materials:    Vec<Material>
}

/// Represents a single cost tracking
#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetEntry {
    /// Unique id of the tracking entry
    pub budget:      BudgetId,
    /// Cost amount
    pub amount:      f64,
    /// User that created this cost
    pub character:   CharacterId,
    /// Timestamp when this tracking was created
    pub created_at:  i64,
    /// Category of the budget entry
    pub category:    BudgetCategory,

    /// Short description for what the cost was
    pub description: Option<String>,
}

/// Represents a single cost tracking
#[derive(Debug, Deserialize)]
pub struct AddBudgetEntry {
    /// User that created this cost
    pub character:   CharacterId,
    /// Cost amount
    pub amount:      f64,
    /// Category of the budget
    pub category:    BudgetCategory,

    /// Short description for what the cost was
    pub description: Option<String>,
}

struct ManufactureOrder(Vec<(TypeId, Vec<TypeId>)>);

impl ManufactureOrder {
    pub fn sort(mut self) -> Vec<TypeId> {
        let mut result = self.0
            .iter()
            .map(|(x, _)| *x)
            .collect::<Vec<_>>();

        while let Some((entry, materials)) = self.0.pop() {
            let mut highest_index = usize::MAX;

            for material in materials {
                let index = result
                    .iter()
                    .position(|x| *x == material);

                if let Some(x) = index {
                    if highest_index == usize::MAX {
                        highest_index = x;
                    } else if x > highest_index{
                        highest_index = x;
                    }
                }
            }

            if highest_index == usize::MAX {
                continue;
            }

            let index = result
                .iter()
                .position(|x| *x == entry)
                .unwrap_or_default();

            result.remove(index);
            if index > highest_index {
                result.insert(highest_index + 1, entry);
            } else {
                result.insert(highest_index, entry);
            }
        }

        result
    }
}

/// Represents a member of a project
#[derive(Clone, Debug, Serialize)]
pub struct Member {
    /// ID of the character
    pub character_id:     CharacterId,
    /// Name of the character
    pub character_name:   String,
    /// If of the character corporation
    pub corporation_id:   CorporationId,
    /// Name of the character corporation
    pub corporation_name: String,
    /// Id of the alliance the corporation is in
    pub alliance_id:      Option<AllianceId>,
    /// Name of the alliance
    pub alliance_name:    Option<String>,
}
