use appraisal::{Appraisal, AppraisalInformation, Janice};
use caph_connector::{CharacterId, GroupId, TypeId, AllianceId, CorporationId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::instrument;
use uuid::Uuid;

use crate::{Error, ProjectBlueprintService, Blueprint, project::dependency::DependencyGroup};
use super::dependency::{DependencyCache, Dependency};

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
    materials_raw:    Vec<Dependency>,
    buildsteps:       Buildstep,
    members:          Vec<Member>,
    bp_required:      Vec<Blueprint>,
    market:           AppraisalInformation
}

/// Wrapper for managing projects
/// TODO: split file into multiple files
#[derive(Clone)]
pub struct ProjectService {
    /// Database pool
    pool:      PgPool,

    /// Service for handling blueprints
    blueprint:        ProjectBlueprintService,

    /// Cache for specific entries
    dependency_cache: DependencyCache
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

        blueprint:        ProjectBlueprintService,

        dependency_cache: DependencyCache
    ) -> Self {
        Self {
            pool,

            blueprint,

            dependency_cache
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
    /// [Option::Some(_)] if the project exists, otherwise [Option::None].
    ///
    #[instrument(err)]
    pub async fn by_id(
        pool: &PgPool,
        pid:  &ProjectId,
    ) -> Result<Option<Project>, Error> {
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
            .fetch_optional(pool)
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
            x.products = Self::fetch_products(pool, pid).await?;
            Ok(Some(x))
        } else {
            Ok(None)
        }
    }

    pub async fn god(
        &self,
        pid: ProjectId
    ) -> Result<GodProject, Error> {
        let info = ProjectService::by_id(&self.pool, &pid).await?.unwrap();
        let materials_stored = self.stored_materials(pid).await?;
        let materials_raw = self.raw_materials(pid).await?;
        let buildsteps = self.buildsteps(pid).await?;
        let members = self.members(pid).await?;
        let market = self.market_price(pid).await?;
        let steps = self.buildstep_manufacturing(pid).await?;
        let bp_required = self.blueprint.required(steps).await?;

        Ok(GodProject {
            info,
            materials_stored,
            materials_raw,
            buildsteps,
            members,
            bp_required,
            market
        })
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
    ) -> Result<Vec<Info>, Error> {
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
    ) -> Result<Vec<Dependency>, Error> {
        let bp_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, x.me.unwrap_or_default() as u8))
            .collect::<HashMap<_, _>>();
        let structure_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 1u8))
            .collect::<HashMap<_, _>>();

        let products = sqlx::query!("
                    SELECT type_id, count
                    FROM project_products
                    WHERE project = $1
                ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                let mut dependency = Dependency::from_cache(
                    &self.dependency_cache,
                    x.type_id.into(),
                );
                dependency.set_product_quantity(x.count as i64);
                dependency.apply_material_bonus(&bp_bonus);

                // TODO: extract, and add rigs
                dependency.apply_material_bonus(&structure_bonus);
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
    pub async fn market_price(
        &self,
        pid: ProjectId,
    ) -> Result<AppraisalInformation, Error> {
        // TODO: check if janice is available, if not fallback to evepraisal (or do both)
        let janice = Janice::init().map_err(Error::AppraisalError)?;

        let raw_materials = self
            .raw_materials(pid)
            .await?
            .into_iter()
            .map(|x| format!("{} {}", x.name, x.products))
            .collect::<Vec<_>>();

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
    ) -> Result<Vec<Dependency>, Error> {
        // TODO: Move
        let bp_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, x.me.unwrap_or_default() as u8))
            .collect::<HashMap<_, _>>();
        let structure_bonus = self.blueprint
            .stored(pid)
            .await?
            .into_iter()
            .map(|x| (x.ptype_id, 1u8))
            .collect::<HashMap<_, _>>();

        let dependencies = sqlx::query!("
                    SELECT type_id, count
                    FROM project_products
                    WHERE project = $1
                ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                let mut dependency = Dependency::from_cache(
                    &self.dependency_cache,
                    x.type_id.into(),
                );

                dependency.set_product_quantity(x.count as i64);
                //dependency.apply_material_bonus(&bp_bonus);
                //dependency.apply_material_bonus(&structure_bonus);
                dependency
            })
            .collect::<Vec<_>>();

        //let mut group = DependencyGroup::from_dependencies(dependencies);
        //let dependencies = group.sort();

        //let mut dependency_group = DependencyGroup::from_dependencies(dependencies);
        //let dependencies = dependency_group.sort();
        //let components = dependency_group.collect_components();

        let dependencies = dependencies
            .into_iter()
            .map(|x| x.collect_components())
            .collect::<Vec<_>>();
        let mut dependencies = dependencies
            .into_iter()
            .reduce(|mut acc, e| {
                acc.merge(e);
                acc
            })
            .unwrap_or_default();

        let old = dependencies.clone();
        dependencies.recalculate();
        dependencies.fix(old);

        let dependencies = dependencies.sort();

        #[derive(Debug, serde::Serialize)]
        struct Temp {
            name: String,
            quantity: i64
        }
        //let xyz: std::collections::HashMap<TypeId, Temp> = dependencies.clone()
        let xyz = dependencies.clone()
            .into_iter()
            .map(|x| (x.ptype_id, x))
            .collect::<HashMap<_, _>>();

        use std::fs::File;
        let mut file = File::create("dependencies.json").unwrap();
        serde_json::to_writer_pretty(&mut file, &xyz).unwrap();

        Ok(dependencies)
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
    ) -> Result<Vec<Member>, Error> {
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
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
            .map_err(Error::DatabaseError)
    }
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
    pub manufacture: Vec<Dependency>,
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

#[cfg(test)]
mod tests {
    use caph_connector::TypeId;
    use sqlx::postgres::PgPoolOptions;
    use std::str::FromStr;

    use crate::{AuthService, CharacterService};
    use super::*;

    #[tokio::test]
    async fn what_da_fuck() {
        dotenv::dotenv().ok();
        let pg_addr = std::env::var("DATABASE_URL")
            .expect("Expected that a DATABASE_URL ENV is set");
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&pg_addr)
            .await
            .unwrap();

        let cache = DependencyCache::new(pool.clone()).await.unwrap();
        let auth_service = AuthService::new(pool.clone());
        let character_service = CharacterService::new(pool.clone(), auth_service);
        let bp_service = ProjectBlueprintService::new(pool.clone(), character_service);

        let service = ProjectService::new(pool, bp_service, cache);
        let xyz = service
            .buildstep_manufacturing(Uuid::from_str("b555b8e9-5957-4441-b850-33d33f88f234").unwrap())
            .await
            .unwrap()
            .into_iter()
            .map(|x| (x.ptype_id, x))
            .collect::<HashMap<_, _>>();

        assert_eq!(xyz.get(&TypeId(30303)).unwrap().products, 1875);
        assert_eq!(xyz.get(&TypeId(30303)).unwrap().components[1].products, 10);
        assert_eq!(xyz.get(&TypeId(30304)).unwrap().products, 525);
        assert_eq!(xyz.get(&TypeId(30304)).unwrap().components[1].products, 15);
        assert_eq!(xyz.get(&TypeId(30305)).unwrap().products, 1125);
        assert_eq!(xyz.get(&TypeId(30305)).unwrap().components[1].products, 50);
        assert_eq!(xyz.get(&TypeId(30306)).unwrap().products, 525);
        assert_eq!(xyz.get(&TypeId(30306)).unwrap().components[1].products, 20);
        assert_eq!(xyz.get(&TypeId(30307)).unwrap().products, 525);
        assert_eq!(xyz.get(&TypeId(30307)).unwrap().components[1].products, 25);
        assert_eq!(xyz.get(&TypeId(30308)).unwrap().products, 375);
        assert_eq!(xyz.get(&TypeId(30308)).unwrap().components[1].products, 15);
        assert_eq!(xyz.get(&TypeId(30308)).unwrap().products, 375);
        assert_eq!(xyz.get(&TypeId(30308)).unwrap().components[1].products, 15);
        assert_eq!(xyz.get(&TypeId(57457)).unwrap().products, 45170);
        assert_eq!(xyz.get(&TypeId(57457)).unwrap().components[0].products, 45200);
        assert_eq!(xyz.get(&TypeId(57457)).unwrap().components[1].products, 226);
        assert_eq!(xyz.get(&TypeId(57457)).unwrap().components[2].products, 45200);
        assert_eq!(xyz.get(&TypeId(57453)).unwrap().products, 45200);
        assert_eq!(xyz.get(&TypeId(57453)).unwrap().components[0].products, 1130);
        assert_eq!(xyz.get(&TypeId(57455)).unwrap().products, 45200);
        assert_eq!(xyz.get(&TypeId(57455)).unwrap().components[0].products, 1130);
        assert_eq!(xyz.get(&TypeId(57454)).unwrap().products, 402);
        assert_eq!(xyz.get(&TypeId(57454)).unwrap().components[0].products, 205);
        assert_eq!(xyz.get(&TypeId(16659)).unwrap().products, 35300);
        assert_eq!(xyz.get(&TypeId(16659)).unwrap().components[0].products, 885);
    }
}
