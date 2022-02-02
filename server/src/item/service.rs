use crate::Error;

use caph_connector::TypeId;
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ItemService {
    pool: PgPool
}

impl ItemService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Gets all item names that are used in manufacturing and inventions.
    /// 
    /// # Errors
    /// 
    /// If the database access failes.
    /// 
    /// # Returns
    /// 
    /// List of all asset names and their [TypeId] that are use in any
    /// manufacture or invention jobs.
    /// 
    pub async fn components(
        &self,
    ) -> Result<Vec<Item>, Error> {
        let blueprints = sqlx::query!(r#"
                SELECT
                    i.type_id AS "type_id!",
                    i.name    AS "name!"
                FROM items i
                WHERE i.type_id = ANY(
                    SELECT DISTINCT(btype_id) FROM blueprint_manufacture
                    UNION
                    SELECT DISTINCT(ptype_id) FROM blueprint_manufacture
                    UNION
                    SELECT DISTINCT(ptype_id) FROM blueprint_inventions
                    UNION
                    SELECT DISTINCT(mtype_id) FROM blueprint_materials
                )
                ORDER BY i.name
            "#)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                Item {
                    type_id: x.type_id.into(),
                    name:    x.name
                }
            })
            .collect::<Vec<_>>();

        Ok(blueprints)
    }

    /// Gets a list of all item names that can be constructed.
    /// 
    /// # Errors
    /// 
    /// If the database access failes.
    /// 
    /// # Returns
    /// 
    /// List of all items that have a blueprint associated with them.
    /// 
    pub async fn buildable(
        &self,
    ) -> Result<Vec<Item>, Error> {
        let blueprints = sqlx::query!("
                SELECT
                    bman.ptype_id,
                    i.name
                FROM blueprint_manufacture bman
                JOIN items i
                  ON i.type_id = bman.ptype_id
                ORDER BY i.name
            ")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                Item {
                    type_id: x.ptype_id.into(),
                    name:    x.name
                }
            })
            .collect::<Vec<_>>();

        Ok(blueprints)
    }

    /// Takes a name and resolves the name to a [TypeId].
    /// 
    /// # Params
    /// 
    /// * `names`  -> List of names that should be resolved
    /// * `filter` -> Pre filters the resolved items
    /// 
    /// # Errors
    /// 
    /// If the database access failes.
    /// 
    /// # Returns
    /// 
    /// List of name and [TypeId] of the requested items.
    /// If the given name
    /// is not found or does not match the filter, the returning array may
    /// be smaller than the given array.
    /// 
    pub async fn resolve_id_from_name_bulk(
        &self,
        names:  Vec<String>,
        filter: ResolveIdNameFilter
    ) -> Result<Vec<Item>, Error> {
        let entries = if let Some(true) = filter.is_buildable {
            sqlx::query!(r#"
                    SELECT
                        bman.ptype_id AS "type_id!",
                        i.name        AS "name!"
                    FROM blueprint_manufacture bman
                    JOIN items i
                    ON i.type_id = bman.ptype_id
                    WHERE name = ANY($1)
                "#,
                    &names
                )
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|x| Item {
                    name:    x.name,
                    type_id: x.type_id.into()
                })
                .collect::<Vec<_>>()
        } else {
            sqlx::query!(r#"
                SELECT
                    i.type_id AS "type_id!",
                    i.name    AS "name!"
                FROM items i
                WHERE i.name = ANY($1)
            "#,
                &names
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| Item {
                name:    x.name,
                type_id: x.type_id.into()
            })
            .collect::<Vec<_>>()
        };
        Ok(entries)
    }
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub type_id: TypeId,
    pub name:    String
}

#[derive(Debug, Deserialize)]
pub struct ResolveIdNameFilter {
    pub is_buildable: Option<bool>
}
