use caph_connector::{TypeId, GroupId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{ProjectId, Error};

/// Service for managing project storage
#[derive(Clone)]
pub struct ProjectStorageService {
    pool: PgPool
}

impl ProjectStorageService {
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
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Gets a list of all stored items of a project.
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
    /// List of all items
    /// 
    pub async fn stored(
        &self,
        pid: ProjectId
    ) -> Result<Vec<StorageEntry>, Error> {
        let entries = sqlx::query!("
                SELECT
                    ps.type_id,
                    ps.quantity,
                    i.name,
                    i.group_id
                FROM project_storage ps
                JOIN items i
                  ON i.type_id = ps.type_id
                WHERE ps.project = $1
            ",
                pid
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| StorageEntry {
                type_id:  x.type_id.into(),
                quantity: x.quantity,
                name:     x.name,
                group_id: x.group_id.into()
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Gets a single stored entry.
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
    /// If the item exists, information about that stored item
    /// 
    pub async fn storage_by_id(
        &self,
        pid: ProjectId,
        tid: TypeId,
    ) -> Result<Option<StorageEntry>, Error> {
        let entry = sqlx::query!("
                SELECT
                    ps.type_id,
                    ps.quantity,
                    i.name,
                    i.group_id
                FROM project_storage ps
                JOIN items i
                  ON i.type_id = ps.type_id
                WHERE ps.project = $1
                  AND ps.type_id = $2
            ",
                pid,
                *tid
            )
            .fetch_optional(&self.pool)
            .await?;
        if let Some(x) = entry {
            Ok(
                Some(StorageEntry {
                    type_id:  x.type_id.into(),
                    quantity: x.quantity,
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
    pub async fn modify(
        &self,
        pid:     ProjectId,
        request: ModifyRequest
    ) -> Result<(), Error> {
        let ids = request
            .entries
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let quantities = request
            .entries
            .iter()
            .map(|x| x.quantity)
            .collect::<Vec<_>>();

        if request.mode == ModifyMode::Add {
            sqlx::query!("
                    INSERT INTO project_storage
                    (
                        project,
                        type_id,
                        quantity
                    )
                    SELECT $1, * FROM UNNEST(
                        $2::INTEGER[],
                        $3::BIGINT[]
                    )
                    ON CONFLICT(project, type_id) DO UPDATE
                    SET quantity = project_storage.quantity + EXCLUDED.quantity
                ",
                    pid,
                    &ids,
                    &quantities
                )
                .execute(&self.pool)
                .await
                .map(drop)
                .map_err(Error::DatabaseError)
        } else {
            sqlx::query!("
                INSERT INTO project_storage
                (
                    project,
                    type_id,
                    quantity
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::BIGINT[]
                )
                ON CONFLICT(project, type_id) DO UPDATE
                SET quantity = EXCLUDED.quantity
            ",
                pid,
                &ids,
                &quantities
            )
            .execute(&self.pool)
            .await
            .map(drop)
            .map_err(Error::DatabaseError)
        }
    }

    /// Sets the storage for a item.
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
    pub async fn set_storage(
        &self,
        pid:     ProjectId,
        entries: Vec<Modify>
    ) -> Result<(), Error> {
        let ids = entries
            .iter()
            .map(|x| *x.type_id)
            .collect::<Vec<_>>();
        let quantities = entries
            .iter()
            .map(|x| x.quantity)
            .collect::<Vec<_>>();

        sqlx::query!("
                INSERT INTO project_storage
                (
                    project,
                    type_id,
                    quantity
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::BIGINT[]
                )
                ON CONFLICT(project, type_id) DO UPDATE
                SET quantity = EXCLUDED.quantity
            ",
                pid,
                &ids,
                &quantities
            )
            .execute(&self.pool)
            .await
            .map(drop)
            .map_err(Error::DatabaseError)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ModifyMode {
    /// Adds the items
    Add,
    /// Sets the items
    Set,
}

/// Represents a modification object
#[derive(Clone, Debug, Deserialize)]
pub struct ModifyRequest {
    /// Mode that should be used
    pub mode:    ModifyMode,
    /// Entries that should be modified
    pub entries: Vec<Modify>
}

/// Represents a modification object
#[derive(Clone, Debug, Deserialize)]
pub struct Modify {
    /// TypeId that should be modified
    pub type_id: TypeId,
    /// When positiv the quantity is added, if negativ this quantity is subtracted
    pub quantity: i64,
}

/// Represents and entry of a stored item
#[derive(Clone, Debug, Serialize)]
pub struct StorageEntry {
    /// Id of the item
    pub type_id:  TypeId,
    /// Item category
    pub group_id: GroupId,
    /// Quantity that is stored
    pub quantity: i64,
    /// Name of the item
    pub name:     String,
}
