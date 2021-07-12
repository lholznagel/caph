use crate::error::EveServerError;

use cachem::v2::ConnectionPool;
use caph_db_v2::{CacheName, ItemEntry};
use caph_eve_data_wrapper::TypeId;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ItemService {
    pool: ConnectionPool
}

impl ItemService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            pool,
        }
    }

    pub async fn all(
        &self
    ) -> Result<Vec<Option<ItemEntry>>, EveServerError> {
        let mut con = self
            .pool
            .acquire()
            .await?;

        let keys = con
            .keys::<_, TypeId>(CacheName::Item)
            .await?;
        con
            .mget::<_, _, ItemEntry>(CacheName::Item, keys)
            .await
            .map_err(Into::into)
    }

    pub async fn keys(
        &self
    ) -> Result<Vec<TypeId>, EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .keys::<_, TypeId>(CacheName::Item)
            .await
            .map_err(Into::into)
    }

    pub async fn meta(
        &self,
        tid: TypeId
    ) -> Result<Option<ItemMeta>, EveServerError> {
        Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemMeta {
    type_id: TypeId,
    mtype:   MaterialType,
    sources: Vec<MaterialSource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaterialSource {
    type_id:  TypeId,
    quantity: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MaterialType {
    Asteroid,
    Ice,
    Moon,
    PI,
    Reaction,
    Salvage,
}

