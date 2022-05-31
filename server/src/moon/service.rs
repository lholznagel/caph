use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use caph_connector::{CharacterId, TypeId};

use crate::Error;
use uuid::Uuid;

/// Service for handling moon pulls
#[derive(Clone)]
pub struct MoonService {
    pool: PgPool
}

impl MoonService {
    /// Creates a new moon instance.
    /// 
    /// # Params
    /// 
    /// * `pool` -> Postres connection pool
    /// 
    /// # Returns
    /// 
    /// New instance
    /// 
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn pull(
        &self,
        cid: CharacterId,
        id:  Uuid,
    ) -> Result<Pull, Error> {
        unimplemented!()
    }

    pub async fn pulls(
        &self,
        cid: CharacterId
    ) -> Result<Vec<Pull>, Error> {
        unimplemented!()
    }

    pub async fn create(
        &self,
        cid:  CharacterId,
        pull: Pull
    ) -> Result<Vec<Pull>, Error> {
        unimplemented!()
    }

    pub async fn update(
        &self,
        cid:  CharacterId,
        id:   Uuid,
        pull: Pull,
    ) -> Result<Vec<Pull>, Error> {
        unimplemented!()
    }
}

/// Represents a single pull
#[derive(Debug, Deserialize, Serialize)]
pub struct Pull {
    id:                Uuid,
    cid:               CharacterId,

    material_1:        u32,
    material_2:        u32,
    material_3:        Option<u32>,
    material_4:        Option<u32>,

    material_1_amount: u32,
    material_2_amount: u32,
    material_3_amount: Option<u32>,
    material_4_amount: Option<u32>,

    extraction_time:   String,

    appraisal_mined:   String,
    appraisal_waste:   String,
}

/// Represents a mined material
#[derive(Debug, Deserialize, Serialize)]
pub struct Mined {
    moon:    Uuid,
    type_id: TypeId,
    amount:  u32,
}
