use crate::{AssetService, BlueprintRaw, error::ServerError};

use caph_connector::{CharacterId, ItemId, LocationId, TypeId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

pub type ProjectId = Uuid;

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

    pub async fn projects(
        &self,
        cid: CharacterId
    ) -> Result<Vec<Project>, ServerError> {
        let entries = sqlx::query!(r#"
                SELECT
                    p.*,
                    ARRAY_AGG(DISTINCT pc.item_id) AS "containers!",
                    ARRAY_AGG(pp.type_id)          AS "product_type_id!",
                    ARRAY_AGG(pp.count)            AS "product_count!"
                FROM project p
                JOIN project_container pc
                  ON pc.project_id = p.id
                JOIN project_product pp
                  ON pp.project_id = p.id
                WHERE character_id = $1
                GROUP BY p.id
            "#,
                *cid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                let containers = x.containers
                    .into_iter()
                    .map(|x| {
                        ProjectContainer {
                            item_id: x.into()
                        }
                    })
                    .collect::<Vec<_>>();

                let products = x.product_type_id.iter()
                    .zip(x.product_count.iter())
                    .into_iter()
                    .map(|(type_id, count): (&i32, &i32)| {
                        ProjectProduct {
                            type_id: (*type_id).into(),
                            count:   *count
                        }
                    })
                    .collect::<Vec<_>>();

                Project {
                    id:   x.id,
                    name: x.name,

                    containers,
                    products
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Fetches all character infos from eve
    ///
    /// # Params
    ///
    /// * `cid` -> Character id to fetch
    ///
    /// # Returns
    ///
    /// Alliance, character and corporation information
    ///
    pub async fn create(
        &self,
        cid:     CharacterId,
        project: CreateProject
    ) -> Result<Uuid, ServerError> {
        let mut trans = self.pool
            .begin()
            .await
            .map_err(ServerError::TransactionBeginNotSuccessfull)?;

        let pid: ProjectId = sqlx::query!("
                INSERT INTO project
                (
                    character_id,
                    name
                )
                VALUES ($1, $2)
                RETURNING id
            ",
                *cid,
                project.name
            )
            .fetch_one(&mut trans)
            .await?
            .id;

        let containers = project
            .containers
            .into_iter()
            .map(|x| *x.item_id)
            .collect::<Vec<_>>();
        sqlx::query!("
                INSERT INTO project_container
                (
                    project_id,
                    item_id
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[]
                )
            ",
                &pid,
                &containers
            )
            .execute(&mut trans)
            .await?;

        let products_type_ids = project
            .products
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let product_counts = project
            .products
            .iter()
            .map(|x| x.count)
            .collect::<Vec<_>>();
        sqlx::query!("
                INSERT INTO project_product
                (
                    project_id,
                    type_id,
                    count
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
            ",
                &pid,
                &products_type_ids,
                &product_counts
            )
            .execute(&mut trans)
            .await?;

        trans.commit()
            .await
            .map_err(ServerError::TransactionCommitNotSuccessfull)?;

        Ok(pid)
    }

    pub async fn by_id(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<Option<Project>, ServerError> {
        let entry = sqlx::query!(r#"
                SELECT
                    p.*,
                    ARRAY_AGG(DISTINCT pc.item_id) AS "containers!",
                    ARRAY_AGG(pp.type_id)          AS "product_type_id!",
                    ARRAY_AGG(pp.count)            AS "product_count!"
                FROM project p
                JOIN project_container pc
                ON pc.project_id = p.id
                JOIN project_product pp
                ON pp.project_id = p.id
                WHERE character_id = $1
                  AND p.id = $2
                GROUP BY p.id
            "#,
                *cid,
                pid
            )
            .fetch_optional(&self.pool)
            .await?
            .map(|x| {
                let containers = x.containers
                .into_iter()
                .map(|x| {
                    ProjectContainer {
                        item_id: x.into()
                    }
                })
                .collect::<Vec<_>>();

            let products = x.product_type_id.iter()
                .zip(x.product_count.iter())
                .into_iter()
                .map(|(type_id, count): (&i32, &i32)| {
                    ProjectProduct {
                        type_id: (*type_id).into(),
                        count:   *count
                    }
                })
                .collect::<Vec<_>>();

                Project {
                    id:   x.id,
                    name: x.name,

                    containers,
                    products
                }
            });
        Ok(entry)
    }

    pub async fn products(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<Vec<ProjectProduct>, ServerError> {
        let entry = sqlx::query!("
                SELECT count, type_id
                FROM project_product
                WHERE (
                    SELECT character_id
                    FROM project
                    WHERE character_id = $1
                      AND id = $2
                ) IS NOT NULL
                  AND project_id = $2
            ",
                *cid,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                ProjectProduct {
                    count:   x.count,
                    type_id: x.type_id.into(),
                }
            })
            .collect::<Vec<_>>();
        Ok(entry)
    }

    pub async fn containers(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<Vec<ProjectContainer>, ServerError> {
        let entry = sqlx::query!("
                SELECT item_id
                FROM project_container
                WHERE (
                    SELECT character_id
                    FROM project
                    WHERE character_id = $1
                      AND id = $2
                ) IS NOT NULL
                  AND project_id = $2
            ",
                *cid,
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                ProjectContainer {
                    item_id: x.item_id.into(),
                }
            })
            .collect::<Vec<_>>();
        Ok(entry)
    }

    pub async fn delete(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<(), ServerError> {
        sqlx::query!("
                DELETE FROM project
                WHERE character_id = $1
                  AND id = $2
            ",
                *cid,
                pid
            )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update(
        &self,
        cid:     CharacterId,
        pid:     ProjectId,
        project: CreateProject
    ) -> Result<Uuid, ServerError> {
        let mut trans = self.pool
            .begin()
            .await
            .map_err(ServerError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("
                UPDATE project
                SET name = $3
                WHERE character_id = $1
                  AND id = $2
            ",
                *cid,
                pid,
                project.name
            )
            .execute(&mut trans)
            .await?;

        sqlx::query!("
                DELETE FROM project_container
                WHERE project_id = $1
            ",
                pid
            )
            .execute(&mut trans)
            .await?;
        sqlx::query!("
            DELETE FROM project_product
            WHERE project_id = $1
        ",
            pid
        )
        .execute(&mut trans)
        .await?;

        let containers = project
            .containers
            .into_iter()
            .map(|x| *x.item_id)
            .collect::<Vec<_>>();
        sqlx::query!("
                INSERT INTO project_container
                (
                    project_id,
                    item_id
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[]
                )
            ",
                &pid,
                &containers
            )
            .execute(&mut trans)
            .await?;

        let products_type_ids = project
            .products
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let product_counts = project
            .products
            .iter()
            .map(|x| x.count)
            .collect::<Vec<_>>();
        sqlx::query!("
                INSERT INTO project_product
                (
                    project_id,
                    type_id,
                    count
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
            ",
                &pid,
                &products_type_ids,
                &product_counts
            )
            .execute(&mut trans)
            .await?;

        trans.commit()
            .await
            .map_err(ServerError::TransactionCommitNotSuccessfull)?;

        Ok(pid)
    }

    pub async fn required_blueprints(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<Vec<ProjectRequiredBlueprint>, ServerError> {
        let entries = sqlx::query!("
                SELECT tree
                FROM blueprint_tree
                WHERE type_id = ANY(
                    SELECT pp.type_id
                    FROM project p
                    JOIN project_product pp
                      ON pp.project_id = p.id
                    WHERE p.character_id = $1
                      AND p.id = $2
                )
            ",
                *cid,
                pid
            )
            .fetch_all(&self.pool)
            .await?;
        let mut parsed_tree = VecDeque::new();
        for entry in entries {
            let parsed: BlueprintTree = serde_json::from_value(entry.tree)?;
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
                SELECT b.type_id AS "type_id!", i.name AS "name!"
                FROM blueprint b
                LEFT JOIN blueprint_material bm
                  ON bm.blueprint = b.id
                LEFT JOIN item i
                  ON i.type_id = b.type_id
                WHERE bm.type_id = ANY($1)
                  AND bm.is_product = TRUE
            "#,
                &unique_entries
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                ProjectRequiredBlueprint {
                    type_id:  x.type_id.into(),
                    name:     x.name,
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    pub async fn required_raw_materials(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<Vec<BlueprintRaw>, ServerError> {
        let required_blueprints = self
            .required_blueprints(cid, pid)
            .await?
            .into_iter()
            .map(|x| x.type_id)
            .collect::<Vec<_>>();

        let products = self.products(cid, pid)
            .await?
            .into_iter()
            .map(|x| (x.type_id, x.count))
            .collect::<HashMap<_, _>>();
        let blueprint_products = self.asset
            .blueprint_material_bulk(required_blueprints.clone(), Some(true))
            .await?;
        let tids = blueprint_products
            .iter()
            .map(|(x, _)| *x)
            .collect::<Vec<_>>();
        let mut raw_materials  = self.asset
            .blueprint_raw_bulk(tids)
            .await?;

        let mut merged = HashMap::new();
        for (type_id, count) in products {
            let produces_per_run = blueprint_products
                .get(&type_id)
                .unwrap()
                .quantity;

            let runs_needed = count as f32 / produces_per_run as f32;
            let runs_needed = runs_needed.ceil() as i32;

            raw_materials
                .get_mut(&type_id)
                .unwrap()
                .into_iter()
                .map(|x| {
                    x.quantity = x.quantity * runs_needed;
                    x
                })
                .for_each(|x| {
                    merged
                        .entry(x.type_id)
                        .and_modify(|e: &mut BlueprintRaw| x.quantity = e.quantity)
                        .or_insert(x.clone());
                });
        }

        let merged = merged
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();

        Ok(merged)
    }

    pub async fn stored_materials(
        &self,
        cid: CharacterId,
        pid: ProjectId
    ) -> Result<Vec<ProjectStoredMaterial>, ServerError> {
        let entries = sqlx::query!(r#"
                SELECT
                    a.location_id,
                    a.reference_id AS "reference_id!",
                    a.type_id,
                    a.quantity
                FROM asset a
                WHERE a.reference_id = ANY(
                    SELECT pc.item_id
                    FROM project p
                    JOIN project_container pc
                      ON p.id = pc.project_id
                    WHERE p.character_id = $1
                      AND p.id = $2
                )
            "#,
                *cid,
                pid
            )
            .fetch_all(&self.pool)
            .await?;

        let mut collected = HashMap::new();
        for entry in entries {
            collected
                .entry(entry.type_id)
                .and_modify(|x: &mut ProjectStoredMaterial| {
                    x.quantity += entry.quantity;
                })
                .or_insert({
                    ProjectStoredMaterial {
                        container_id: entry.reference_id.into(),
                        location_id:  entry.location_id.into(),
                        type_id:      entry.type_id.into(),
                        quantity:     entry.quantity
                    }
                });
        }

        let entries = collected
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();

        Ok(entries)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id:         Uuid,
    pub name:       String,

    pub containers: Vec<ProjectContainer>,
    pub products:   Vec<ProjectProduct>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProject {
    pub name: String,

    pub containers: Vec<ProjectContainer>,
    pub products:   Vec<ProjectProduct>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectContainer {
    pub item_id: ItemId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectProduct {
    pub type_id: TypeId,
    pub count:   i32
}

#[derive(Debug, Serialize)]
pub struct ProjectRequiredBlueprint {
    pub name:    String,
    pub type_id: TypeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlueprintTree {
    pub key:      TypeId,
    pub label:    String,
    pub children: Option<Vec<BlueprintTree>>
}

#[derive(Debug, Serialize)]
pub struct ProjectStoredMaterial {
    pub container_id: ItemId,
    pub location_id:  LocationId,
    pub type_id:      TypeId,
    pub quantity:     i32,
}
