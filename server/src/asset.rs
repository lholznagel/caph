use crate::ServerError;

use caph_connector::TypeId;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AssetService {
    pool: PgPool
}

impl AssetService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn general_assets_buildable(
        &self,
    ) -> Result<Vec<AssetBuildable>, ServerError> {
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
                AssetBuildable {
                    type_id: x.ptype_id.into(),
                    name:    x.name
                }
            })
            .collect::<Vec<_>>();

        Ok(blueprints)
    }
}

#[derive(Debug, Serialize)]
pub struct AssetBuildable {
    pub type_id: TypeId,
    pub name:    String
}
