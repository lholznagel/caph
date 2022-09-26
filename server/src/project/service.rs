use appraisal::{Appraisal, AppraisalInformation, Janice};
use caph_connector::{CharacterId, GroupId, TypeId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::convert::Infallible;
use tracing::instrument;
use uuid::Uuid;
use warp::Filter;

use crate::{Error, structure::structure::{StructureType, StructureRig}, project::dependency_v2::{dependency::{Dependency, DependencyTree, StructureMapping}}};
use super::dependency_v2::dependency::DependencyTreeEntry;
use crate::structure::structure::{Structure, Security};

/// An id of a tracking entry
pub type BudgetId    = Uuid;
/// Id of a virtual container
pub type ContainerId = Uuid;
/// A project id is just a UUID, this type is just for clarification
pub type ProjectId   = Uuid;

#[derive(Debug, Serialize)]
pub struct GodProject {
    info:             Project,
    materials_stored: Vec<Material>,
    //materials_raw:    Vec<Dependency>,
    buildsteps:       Buildstep,
    //bp_required:      Vec<Blueprint>,
    //market:           AppraisalInformation
}

/// Wrapper for managing projects
/// TODO: split file into multiple files
#[derive(Clone)]
#[deprecated]
pub struct ProjectService {
    /// Database pool
    pool:      PgPool,

    //blueprint:        ProjectBlueprintService,
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
        pool:             PgPool,

        //blueprint:        ProjectBlueprintService,
    ) -> Self {
        Self {
            pool,

            //blueprint,
        }
    }

    pub async fn god(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<GodProject, Error> {
        let p_service = ProjectServiceV2::new(self.pool.clone());

        let info = p_service.by_id(&pid).await?;
        let materials_stored = self.stored_materials(pid).await?;
        //let materials_raw = self.raw_materials(pid).await?;
        let buildsteps = self.buildsteps(pid).await?;
        //let market = self.market_price(pid).await?;
        //let steps = self.buildstep_manufacturing(pid).await?;
        //let bp_required = self.blueprint.required(steps).await?;

        Ok(GodProject {
            info,
            materials_stored,
            //materials_raw,
            buildsteps,
            //bp_required,
            //market
        })
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
    ) -> Result<ProjectId, Error> {
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
    ) -> Result<Option<ProjectId>, Error> {
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
    ) -> Result<Vec<Material>, Error> {
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
                        type_id:         x.type_id.into(),
                        quantity:        x.quantity as i64,
                        quantity_single: x.quantity as i64,
                        name:            x.name,
                        group_id:        x.group_id.into(),
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
    // ) -> Result<Vec<Dependency>, Error> {
    ) -> Result<Vec<String>, Error> {
        /*let bp_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, x.me.unwrap_or_default() as f32))
            .collect::<HashMap<_, _>>();
        let structure_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 1f32))
            .collect::<HashMap<_, _>>();
        let rig_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 4.2f32))
            .collect::<HashMap<_, _>>();
        let reaction_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 2.6f32))
            .collect::<HashMap<_, _>>();

        let products = sqlx::query!("
                    SELECT type_id, count, data
                    FROM project_products pp
                    JOIN blueprint_json bj
                      ON bj.ptype_id = pp.type_id
                    WHERE project = $1
                ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                let dependency: DatabaseDependency = serde_json::from_value(x.data).unwrap();
                let mut dependency = Dependency::from(dependency);

                if dependency.dependency_type == DependencyType::Material ||
                    dependency.dependency_type == DependencyType::Blueprint {

                    dependency.apply_material_bonus(&bp_bonus);
                    dependency.apply_material_bonus(&rig_bonus);
                    dependency.apply_material_bonus(&structure_bonus);
                } else if dependency.dependency_type == DependencyType::Reaction {
                    dependency.apply_material_bonus(&reaction_bonus);
                }
                dependency.round_material_bonus();

                dependency
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x.collect_raw_materials())
            .reduce(|mut acc, e| {
                acc.merge(e);
                acc
            })
            .unwrap_or_default()
            .into_inner();

        let mut materials = products
            .values()
            .cloned()
            .collect::<Vec<_>>();
        materials.sort_by_key(|x| x.name.clone());
        Ok(materials)*/
        Ok(Vec::new())
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
    pub async fn market_price(
        &self,
        pid: ProjectId,
    ) -> Result<AppraisalInformation, Error> {
        // TODO: check if janice is available, if not fallback to evepraisal (or do both)
        let janice = Janice::init().map_err(Error::AppraisalError)?;

        /*let raw_materials = self
            .raw_materials(pid)
            .await?
            .into_iter()
            .map(|x| format!("{} {}", x.name, x.product()))
            .collect::<Vec<_>>();*/
        let raw_materials = Vec::new();

        janice.create(
            true,
            raw_materials
        )
        .await
        .map_err(Error::AppraisalError)
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
    ) -> Result<Buildstep, Error> {
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
    ///
    /// # Errors
    ///
    /// If the database fails.
    ///
    /// # Returns
    ///
    /// List of the buildsteps.
    /// TODO: extract and refactor to seperate function
    pub async fn buildstep_manufacturing(
        &self,
        pid: ProjectId,
    //) -> Result<Vec<Dependency>, Error> {
    ) -> Result<Vec<String>, Error> {
        // TODO: Move
        /*let bp_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, x.me.unwrap_or_default() as f32))
            .collect::<HashMap<_, _>>();
        let structure_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 1f32))
            .collect::<HashMap<_, _>>();
        let rig_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 4.2f32))
            .collect::<HashMap<_, _>>();
        let reaction_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 2.6f32))
            .collect::<HashMap<_, _>>();
        //let structures = Self::structures();

        let dependencies = sqlx::query!("
                    SELECT type_id, count, data
                    FROM project_products pp
                    JOIN blueprint_json bj
                    ON bj.ptype_id = pp.type_id
                    WHERE project = $1
                ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                let dependency: DatabaseDependency = serde_json::from_value(x.data).unwrap();
                let mut dependency = Dependency::from(dependency);

                dependency.set_product_quantity(x.count as u32);

                /*if dependency.dependency_type == DependencyType::Material ||
                    dependency.dependency_type == DependencyType::Blueprint {

                    dependency.apply_material_bonus(&bp_bonus);
                    dependency.apply_material_bonus(&rig_bonus);
                    dependency.apply_material_bonus(&structure_bonus);
                } else if dependency.dependency_type == DependencyType::Reaction {
                    dependency.apply_material_bonus(&reaction_bonus);
                }
                dependency.round_material_bonus();*/

                dependency
            })
            .collect::<Vec<_>>();

        let mut dependencies = DependencyGroup::from_dependencies(dependencies)
            .collect_components();
        dependencies.recalculate();
        let dependencies = dependencies.build_order();

        //let dependencies = Vec::new();
        Ok(dependencies)*/
        Ok(Vec::new())
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
    ) -> Result<Vec<BuildstepEntryInvention>, Error> {
        let mut inventions = HashMap::new();

        sqlx::query!("
                SELECT
                    bi.itype_id,
                    bi.time,
                    bi.probability,
                    bim.mtype_id,
                    bim.quantity,
                    i.name      AS iname,    -- item name
                    ii.name     AS dname,    -- datacore name
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
                            type_id:         x.mtype_id.into(),
                            quantity:        x.quantity,
                            quantity_single: x.quantity,
                            name:            x.dname.clone(),
                            group_id:        x.dgroup_id.into(),
                        });
                    })
                    .or_insert({
                        let material = Material {
                            type_id:         x.mtype_id.into(),
                            quantity:        x.quantity,
                            quantity_single: x.quantity,
                            name:            x.dname,
                            group_id:        x.dgroup_id.into(),
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
    ) -> Result<Vec<BudgetEntry>, Error> {
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
    ) -> Result<Option<BudgetEntry>, Error> {
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
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
        pool: &PgPool,
        pid:  &ProjectId,
    ) -> Result<Vec<Product>, Error> {
        let entries = sqlx::query!("
                SELECT pp.*, i.name
                FROM project_products pp
                JOIN items i
                  ON pp.type_id = i.type_id
                WHERE pp.project = $1
            ",
                pid
            )
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|x| Product {
                name:    x.name,
                count:   x.count as u32,
                // FIXME:
                meff:    0u32,
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
    ) -> Result<(), Error> {
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
            .map(|x| x.count as i32)
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
            .map_err(Error::DatabaseError)
    }

    /*fn structures() -> Vec<Structure> {
        vec![
            Structure::new(
                "Enablement Station - RAMI".into(),
                "F-NMX6".into(),
                crate::Security::Nullsec,
                crate::StructureType::Tatara,
                vec![
                    crate::StructureRig::ReactorEfficiencyII,
                ]
            ),
            Structure::new(
                "Shipyard".into(),
                "Q-5211".into(),
                crate::Security::Nullsec,
                crate::StructureType::Azbel,
                vec![
                    crate::StructureRig::AdvancedComponentManufacturingEfficiencyI,
                ]
            ),
            Structure::new(
                "CAPSBEL".into(),
                "Q-5211".into(),
                crate::Security::Nullsec,
                crate::StructureType::Azbel,
                vec![
                    crate::StructureRig::CapitalShipManufacturingEfficiencyI,
                    crate::StructureRig::CapitalComponentManufacturingEfficiencyI,
                ]
            ),
        ]
    }*/
}

impl std::fmt::Debug for ProjectService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectService").finish()
    }
}

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
pub struct ProjectInfo {
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
    pub count:   u32,
    /// Material efficiency
    pub meff:    u32,
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
    pub type_id:         TypeId,
    /// Number of items that are stored in this container
    pub quantity:        i64,
    /// Quantity required for a single run
    pub quantity_single: i64,
    /// Name of the item that is stored
    pub name:            String,
    /// Group of the item
    pub group_id:        GroupId,
}

impl Material {
    /// Applies a material efficiency bonus.
    /// 
    /// # Params
    /// 
    /// * `me` -> Bonus in percent
    /// 
    pub fn apply_me_bonus(&mut self, me: u8) {
        if  self.quantity >= 10 &&
            self.quantity_single > 1 {
            self.quantity = 
                self.quantity -
                (self.quantity as f32 * (me as f32 / 100f32)) as i64;
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            type_id:         0.into(),
            quantity:        0,
            quantity_single: 0,
            name:            String::new(),
            group_id:        0.into(),
        }
    }
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
    pub count:   u32,
    /// Material efficiency
    pub meff:    u32,
    /// TypeId of the product
    pub type_id: TypeId,
}

impl From<Product> for ProductConfig {
    fn from(x: Product) -> Self {
        Self {
            count:   x.count as u32,
            meff:    x.meff as u32,
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
    //pub manufacture: Vec<Dependency>,
    pub manufacture: Vec<String>,
    /// Contains all required inventions
    pub inventions:  Vec<BuildstepEntryInvention>
}

/// Represents a buildstep of a blueprint
#[derive(Clone, Debug, Serialize)]
pub struct BuildstepEntry {
    /// Name of the blueprint
    pub name:            String,
    /// Time it takes for one run
    pub time_per_run:    i32,
    /// Total time it takes to run all runs
    pub time_total:      i32,
    /// [TypeId] of the product
    pub type_id:         TypeId,
    /// [TypeId] of the blueprint
    pub btype_id:        TypeId,
    /// Number of runs
    pub runs:            i32,
    /// Count of item that is produced by all runs
    pub produces:        i64,
    /// Count of item that is produced by all runs
    pub produces_per_run: i64,
    /// Cores that are required for invention
    pub materials:       Vec<Material>
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
    pub description: String,
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
    pub description: String,
}

/// Contains all information for creating a new project
#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    /// Name of the project
    pub name:       String,
    /// List of all products that should be build
    pub products:   Vec<ProductConfig>,
    /// Status of the project
    pub status:     Option<Status>
}

pub struct ProjectServiceV2 {
    pool: PgPool
}

impl ProjectServiceV2 {
    pub fn new(
        pool: PgPool,
    ) -> Self {
        Self {
            pool
        }
    }

    /// Minimal version of all projects that the user has access to.
    ///
    /// # Params
    ///
    /// * `cid` > [CharacterId] of the requesting user
    ///
    /// # Errors
    ///
    /// - If the database is not available
    ///
    /// # Returns
    ///
    /// List all project ids the user has access to.
    ///
    pub async fn all(
        &self,
        cid: CharacterId,
    ) -> Result<Vec<ProjectInfo>, Error> {
        let entries = sqlx::query!(r#"
                SELECT
                    p.project,
                    p.name,
                    p.owner,
                    p.status AS "status: Status"
                FROM projects p
                WHERE p.owner = $1
                ORDER BY p.name
            "#,
                *cid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| ProjectInfo {
                project: x.project,
                name:    x.name,
                owner:   x.owner.into(),
                status:  x.status
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Gets a single project by its id.
    /// 
    /// # Params
    /// 
    /// * `pid` > [Uuid] of the project
    /// 
    /// # Erros
    /// 
    /// - If the database is not available
    /// - If the project does not exist
    /// 
    /// # Returns
    /// 
    /// General information of the project.
    /// 
    pub async fn by_id(
        &self,
        pid: &ProjectId,
    ) -> Result<Project, Error> {
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
                pid,
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
            Ok(x)
        } else {
            Err(Error::NotFound)
        }
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
        cfg: ProjectConfig,
    ) -> Result<ProjectId, Error> {
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

        self.insert_products(pid, cfg.products).await?;
        Ok(pid)
    }

    pub async fn jobs(
        &self,
        pid: ProjectId,
    ) -> Result<Vec<DependencyTreeEntry>, Error> {
        use std::str::FromStr;
        let manufacturing_a = Structure::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
            "Sotiyo manufacturing".into(),
            "Here".into(),
            Security::Nullsec,
            StructureType::Sotiyo,
            vec![
                StructureRig::new(&self.pool, TypeId::from(37180)).await.unwrap(),
                StructureRig::new(&self.pool, TypeId::from(37178)).await.unwrap(),
                StructureRig::new(&self.pool, TypeId::from(43704)).await.unwrap(),
            ]
        );

        let reaction_a = Structure::new(
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            "Tatara reactions".into(),
            "Here".into(),
            Security::Nullsec,
            StructureType::Tatara,
            vec![
                StructureRig::new(&self.pool, TypeId::from(46497)).await.unwrap(),
            ]
        );

        let mapping = vec![
            StructureMapping {
                structure:      manufacturing_a.id,
                category_group: manufacturing_a.category_groups(),
            },
            StructureMapping {
                structure:      reaction_a.id,
                category_group: reaction_a.category_groups(),
            },
        ];

        /*let bonus = Bonus::new(
            vec![manufacturing_a, reaction_a],
            mapping,
            HashMap::new(),
        );*/

        let mut dependencies = Vec::new();
        let products = sqlx::query!("
                    SELECT count, data
                    FROM project_products pp
                    JOIN blueprint_json bj ON bj.ptype_id = pp.type_id
                    WHERE project = $1
                ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| (x.count as u32, x.data))
            .map(|(count, data)| Dependency::try_from(count, data))
            .collect::<Vec<Result<Dependency, Error>>>();

        let timer = std::time::Instant::now();

        for product in products {
            match product {
                Ok(x)  => dependencies.push(x),
                Err(_) => continue
            }
        }

        let tree = DependencyTree::from_dependencies(
                dependencies,
                vec![manufacturing_a, reaction_a],
                mapping,
                HashMap::new(),
            )
            .apply_bonus();
        //.flat_tree();
        //bonus.apply_blueprint_bonus(&mut tree);
        //bonus.apply_structure_bonus(&mut tree);

        let entries = tree
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();

        dbg!(timer.elapsed().as_millis());
        Ok(entries)
    }

    /// Fetches the products that should be produced in a project.
    /// 
    /// # Errors
    /// 
    /// - If the database is not available
    /// 
    /// # Returns
    /// 
    /// Vec of products
    /// 
    async fn fetch_products(
        &self,
        pid:  &ProjectId,
    ) -> Result<Vec<Product>, Error> {
        let entries = sqlx::query!("
                SELECT
                    pp.*,
                    i.name
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
                count:   x.count as u32,
                meff:    x.meff as u32,
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
    ) -> Result<(), Error> {
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
            .map(|x| x.count as i32)
            .collect::<Vec<_>>();
        let meff = products
            .iter()
            .map(|x| x.meff as i32)
            .collect::<Vec<_>>();
        sqlx::query!("
                INSERT INTO project_products
                (
                    project,
                    type_id,
                    count,
                    meff
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::INTEGER[],
                    $4::INTEGER[]
                )
                ON CONFLICT (project, type_id)
                DO UPDATE SET count = EXCLUDED.count
            ",
                pid,
                &type_ids,
                &counts,
                &meff
            )
            .execute(&self.pool)
            .await
            .map(drop)
            .map_err(Error::DatabaseError)
    }
}

impl std::fmt::Debug for ProjectServiceV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectService").finish()
    }
}

/// Filter for the API.
/// 
/// # Params
/// 
/// * `pool` > Open connection to postgres
/// 
/// # Returns
/// 
/// Initialized instance of [ProjectServiceV2]
/// 
pub fn with_project_service(
    pool: PgPool,
)  -> impl Filter<Extract = (ProjectServiceV2,), Error = Infallible> + Clone {
    warp::any()
        .map(move || ProjectServiceV2::new(pool.clone()))
}
