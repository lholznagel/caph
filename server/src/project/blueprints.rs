
use crate::{ProjectId, Error, CharacterService};

use caph_connector::{TypeId, GroupId, CharacterId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;

use super::dependency::Dependency;

/// Service for managing project storage
#[derive(Clone)]
pub struct ProjectBlueprintService {
    pool: PgPool,

    character: CharacterService
}

impl ProjectBlueprintService {
    /// Creates a new service instance.
    /// 
    /// # Params
    /// 
    /// * `pool` -> Postgres pool
    /// 
    /// # Returns
    /// 
    /// New instance.
    /// 
    pub fn new(
        pool: PgPool,

        character: CharacterService
    ) -> Self {
        Self {
            pool,

            character
        }
    }

    /// Gets a list of all stored blueprints of a project.
    /// 
    /// # Params
    /// 
    /// * `pid` -> Id of the project
    /// 
    /// # Errors
    /// 
    /// When the database access fails.
    /// 
    /// # Returns
    /// 
    /// List of all stored blueprints
    /// 
    /// 
    pub async fn stored(
        &self,
        pid: ProjectId
    ) -> Result<Vec<BlueprintStorageEntry>, Error> {
        let entries = sqlx::query!("
                SELECT
                    bm.btype_id,
                    bm.ptype_id,
                    pb.runs,
                    pb.me,
                    pb.te,
                    i.name,
                    i.group_id
                FROM project_blueprints pb
                JOIN blueprint_manufacture bm
                  ON bm.btype_id = pb.type_id
                JOIN items i
                  ON i.type_id = pb.type_id
                WHERE pb.project = $1
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| BlueprintStorageEntry {
                btype_id: x.btype_id.into(),
                ptype_id: x.ptype_id.into(),
                runs:     x.runs,
                me:       x.me,
                te:       x.te,
                name:     x.name,
                group_id: x.group_id.into()
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Gets a single stored blueprint.
    /// 
    /// # Params
    /// 
    /// * `pid` -> Id of the project
    /// * `tid` -> TypeId of the stored item
    /// 
    /// # Errors
    /// 
    /// When the database access fails.
    /// 
    /// # Returns
    /// 
    /// If the item exists, information about that stored blueprint
    /// 
    /// 
    pub async fn by_id(
        &self,
        pid: ProjectId,
        tid: TypeId,
    ) -> Result<Option<BlueprintStorageEntry>, Error> {
        let entry = sqlx::query!("
                SELECT
                    bm.btype_id,
                    bm.ptype_id,
                    pb.runs,
                    pb.me,
                    pb.te,
                    i.name,
                    i.group_id
                FROM project_blueprints pb
                JOIN blueprint_manufacture bm
                  ON bm.btype_id = pb.type_id
                JOIN items i
                  ON i.type_id = pb.type_id
                WHERE pb.project = $1
                  AND pb.type_id = $2
            ",
                pid,
                *tid
            )
            .fetch_optional(&self.pool)
            .await?;
        if let Some(x) = entry {
            Ok(
                Some(BlueprintStorageEntry {
                    btype_id: x.btype_id.into(),
                    ptype_id: x.ptype_id.into(),
                    runs:     x.runs,
                    me:       x.me,
                    te:       x.te,
                    name:     x.name,
                    group_id: x.group_id.into()
                })
            )
        } else {
            Ok(None)
        }
    }

    /// Modifies the storage entries in the database.
    /// If the quantity is negativ it will subtract it from the current quantity.
    /// 
    /// # Params
    /// 
    /// * `pid`     -> Project id to modify the storage
    /// * `entries` -> Modification entries
    /// 
    /// # Errors
    /// 
    /// If there there is a problem with the database
    /// 
    /// # Returns
    /// 
    /// Nothing.
    /// 
    pub async fn set(
        &self,
        pid:   ProjectId,
        entry: Vec<ModifyBlueprintEntry>
    ) -> Result<(), Error> {
        let type_ids = entry
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let runs = entry
            .iter()
            .map(|x| x.runs)
            .collect::<Vec<_>>();
        let mes = entry
            .iter()
            .map(|x| x.me)
            .collect::<Vec<_>>();
        let tes = entry
            .iter()
            .map(|x| x.te)
            .collect::<Vec<_>>();

        sqlx::query!("
            INSERT INTO project_blueprints
            (
                project,
                type_id,

                runs,
                me,
                te
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::INTEGER[],
                $5::INTEGER[]
            )
            ON CONFLICT(project, type_id) DO UPDATE
            SET runs = EXCLUDED.runs,
                me = EXCLUDED.me,
                te = EXCLUDED.te
        ",
            pid,
            &type_ids,

            &runs as _,
            &mes  as _,
            &tes  as _
        )
        .execute(&self.pool)
        .await
        .map(drop)
        .map_err(Error::DatabaseError)
    }

    /// Fetches all blueprints owned by any of the connected accounts and adds
    /// them to the project storage.
    /// 
    /// # Params
    /// 
    /// * `pid`        -> [ProjectId] the blueprints should be linked to
    /// * `cid`        -> [CharacterId] of the main character
    /// * `buildsteps` -> TODO: validate after Self::required is refactored
    /// 
    /// # Errors
    /// 
    /// If the API is not available or the database is not available
    /// 
    /// # Returns
    /// 
    /// Nothing
    /// 
    pub async fn import_from_character(
        &self,
        pid:        ProjectId,
        cid:        CharacterId,
        buildsteps: Vec<Dependency>,
    ) -> Result<(), Error> {
        let mut character_bps = self.character.blueprints(cid).await?;
        let corporation_bps = self.character.corporation_blueprints(cid).await?;
        character_bps.extend(corporation_bps);

        let required_bps = self.required(buildsteps).await?;

        let mut res = HashMap::new();

        for btid in required_bps.iter().map(|x| x.type_id) {
            let entry = character_bps
                .iter()
                .find(|x| x.type_id == btid);
            if let Some(e) = entry {
                res
                    .entry(btid)
                    .and_modify(|x: &mut ModifyBlueprintEntry| {
                        // TODO: allow multiple to be stored and visible in the ui
                        if x.me.unwrap_or(0)    == e.material_efficiency &&
                           x.te.unwrap_or(0)    == e.time_efficiency &&
                           x.runs.unwrap_or(-3) == e.runs {
                        } else if e.material_efficiency > x.me.unwrap_or(0) {
                            x.me       = Some(e.material_efficiency);
                            x.te       = Some(e.time_efficiency);
                            x.runs     = Some(e.runs);
                        }
                    })
                    .or_insert(ModifyBlueprintEntry {
                        type_id:  btid,
                        runs:     Some(e.runs),
                        me:       Some(e.material_efficiency),
                        te:       Some(e.time_efficiency),
                    });
            } else {
                continue;
            }
        }

        self
            .set(pid, res.values().cloned().collect::<Vec<_>>())
            .await?;

        Ok(())
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
    /// 
    pub async fn required(
        &self,
        dependencies: Vec<Dependency>
    ) -> Result<Vec<Blueprint>, Error> {
        /*let mut required_blueprints = HashMap::new();
        let mut queue = VecDeque::from(dependencies);

        while let Some(x) = queue.pop_front() {
            if x.dependency_type.is_material() {
                continue;
            }

            let blueprint = Blueprint {
                type_id:        x.btype_id.into(),
                name:           x.blueprint_name.clone(),
                is_stored:      false,
                is_blueprint:   x.dependency_type.is_blueprint(),
                is_reaction:    x.dependency_type.is_reaction(),
                iters:          x.runs()
            };

            required_blueprints
                .entry(x.btype_id)
                .and_modify(|x: &mut Blueprint| x.iters += blueprint.iters)
                .or_insert(blueprint);

            queue.extend(x.components);
        }*/

        let steps = dependencies
            .iter()
            .map(|x| x
                    .collect_ptype_ids()
                    .into_iter()
                    .map(|x| *x)
                    .collect::<Vec<_>>()
            )
            .flatten()
            .collect::<Vec<_>>();

        let bps = sqlx::query!(r#"
                SELECT
                    bman.btype_id AS "btype_id!",
                    bman.reaction AS "reaction!",
                    i.name        AS "name!"
                FROM blueprint_manufacture bman
                JOIN items i
                  ON i.type_id = bman.btype_id
                WHERE bman.ptype_id = ANY($1)
                ORDER BY i.name
            "#,
                &steps
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| Blueprint {
                type_id:        x.btype_id.into(),
                name:           x.name,
                is_stored:      false,
                is_blueprint:   !x.reaction,
                is_reaction:    x.reaction,
                iters:          0
            })
            .collect::<Vec<_>>();

        Ok(bps)
    }
}

/// Represents a modification object
#[derive(Clone, Debug, Deserialize)]
pub struct ModifyBlueprintEntry {
    /// TypeId that should be modified
    pub type_id: TypeId,
    /// If the entry is a blueprint it will have either -1 or > 0 runs
    /// If the value is -1 then the blueprint is a bpo, otherwise a bpc
    pub runs:     Option<i32>,
    /// If the entry is a blueprint it will have material efficiency
    pub me:       Option<i32>,
    /// If the entry is a blueprint it will have time efficiency
    pub te:       Option<i32>,
}

/// Represents and entry of a stored item
#[derive(Clone, Debug, Serialize)]
pub struct BlueprintStorageEntry {
    /// [TypeId] of the product
    pub ptype_id: TypeId,
    /// [TypeId] of the blueprint
    pub btype_id: TypeId,
    /// Item category
    pub group_id: GroupId,
    /// If the entry is a blueprint it will have either -1 or > 0 runs
    /// If the value is -1 then the blueprint is a bpo, otherwise a bpc
    pub runs:     Option<i32>,
    /// If the entry is a blueprint it will have material efficiency
    pub me:       Option<i32>,
    /// If the entry is a blueprint it will have time efficiency
    pub te:       Option<i32>,
    /// Name of the item
    pub name:     String,
}

/// Represents an blueprint from a product
#[derive(Clone, Debug, Serialize)]
pub struct Blueprint {
    /// [TypeId] of the blueprint
    pub type_id:      TypeId,
    /// Name of the blueprint
    pub name:         String,
    /// True if the blueprint is a manufacture blueprint
    pub is_blueprint: bool,
    /// True if the blueprint is a reaction blueprint
    pub is_reaction:  bool,
    /// True if the blueprint is stored
    pub is_stored:    bool,
    /// Number of blueprint iterations needed
    pub iters:        u32,
}
