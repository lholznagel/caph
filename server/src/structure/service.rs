use caph_connector::{TypeId, CharacterId};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::convert::Infallible;
use uuid::Uuid;
use warp::Filter;

use super::error::StructureError;
use super::structure::{Structure, StructureRig, StructureType, Security};

#[derive(Clone, Debug)]
pub struct StructureService {
    pool: PgPool,
}

impl StructureService {
    pub fn new(
        pool: PgPool,
    ) -> Self {
        Self {
            pool
        }
    }

    pub async fn get_all(
        &self,
        cid: CharacterId,
    ) -> Result<Vec<Structure>, StructureError> {
        let result = sqlx::query!(r#"
                SELECT
                    id,
                    name,
                    system,
                    security AS "security!: Security",
                    sid,
                    rig0,
                    rig1,
                    rig2
                FROM structures
                WHERE character = $1
            "#,
                *cid
            )
            .fetch_all(&self.pool)
            .await
            .map_err(StructureError::FetchStructures)?;

        let mut structures = Vec::new();
        for structure in result {
            let mut rigs = Vec::new();

            if let Some(x) = structure.rig0 {
                rigs.push(StructureRig::new(&self.pool, TypeId::from(x)).await?);
            }

            if let Some(x) = structure.rig1 {
                rigs.push(StructureRig::new(&self.pool, TypeId::from(x)).await?);
            }

            if let Some(x) = structure.rig2 {
                rigs.push(StructureRig::new(&self.pool, TypeId::from(x)).await?);
            }

            let structure = Structure::new(
                structure.id,
                structure.name,
                structure.system,
                structure.security,
                StructureType::from(structure.sid),
                rigs,
            );
            structures.push(structure);
        }

        Ok(structures)
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
/// Initialized instance of [StructureService]
/// 
pub fn with_structure_service(
    pool: PgPool,
)  -> impl Filter<Extract = (StructureService,), Error = Infallible> + Clone {
    warp::any().map(move || StructureService::new(pool.clone()))
}
