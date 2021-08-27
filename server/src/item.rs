use crate::error::EveServerError;

use cachem::ConnectionPool;
use caph_db::{CacheName, ItemDogmaEntry, ItemEntry};
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

    pub async fn dogma_skill(
        &self,
        type_id: TypeId
    ) -> Result<Vec<ItemDogmaSkill>, EveServerError> {
        let mut con = self
            .pool
            .acquire()
            .await?;

        let dogma = con
            .get::<_, TypeId, ItemDogmaEntry>(CacheName::ItemDogma, type_id)
            .await?;
        let dogma = if let Some(x) = dogma {
            x
        } else {
            return Ok(Vec::new())
        };

        let mut skills = Vec::new();
        let mut add_skill = |skill: u32, level: u32| {
            let primary = dogma
                .attributes
                .iter()
                .find(|x| x.attr_id == skill.into())
                .cloned();
            if let Some(x) = primary {
                let skill_id = x.value as u32;
                let level = dogma
                    .attributes
                    .iter()
                    .find(|x| x.attr_id == level.into())
                    .cloned()
                    .map(|x| x.value as u8)
                    .unwrap_or(1u8);
                skills.push(ItemDogmaSkill { skill_id: skill_id.into(), level });
            }
        };

        add_skill(182, 277);
        add_skill(183, 278);
        add_skill(184, 279);

        Ok(skills)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemDogmaSkill {
    pub skill_id: TypeId,
    pub level:    u8,
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

