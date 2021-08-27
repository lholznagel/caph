use std::collections::HashMap;

use crate::{character::CharacterService, error::EveServerError, eve::EveAuthService, item::ItemService};

use cachem::ConnectionPool;
use caph_db::{AllianceFittingEntry, CacheName};
use caph_eve_data_wrapper::TypeId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Service for all character related interfaces
#[derive(Clone)]
pub struct AllianceService {
    pool:      ConnectionPool,
    eve_auth:  EveAuthService,
    character: CharacterService,
    item:      ItemService,
}

impl AllianceService {
    /// Creates a new instance
    pub fn new(
        pool: ConnectionPool,

        eve_auth:  EveAuthService,
        character: CharacterService,
        item:      ItemService,
    ) -> Self {
        Self {
            pool,
            eve_auth,
            character,
            item,
        }
    }

    pub async fn get_fittings(
        &self,
        token: Uuid,
    ) -> Result<Vec<AllianceFitting>, EveServerError> {
        let mut con = self
            .pool
            .acquire()
            .await?;

        let ids = con
            .keys::<_, Uuid>(CacheName::AllianceFitting)
            .await?;
        let fittings = con
            .mget::<_, Uuid, AllianceFittingEntry>(CacheName::AllianceFitting, ids)
            .await?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let mut type_ids = Vec::new();
        fittings
            .iter()
            .for_each(|x| {
                x
                    .clone()
                    .fittings
                    .into_iter()
                    .for_each(|y| {
                        type_ids.extend(y.type_ids);
                    });
            });
        type_ids.sort();
        type_ids.dedup();

        let mut dogma_skills = HashMap::new();
        for type_id in type_ids {
            let entry = self.item.dogma_skill(type_id).await?;
            dogma_skills.insert(type_id, entry);
        }

        let skills = self
            .character
            .skills(token)
            .await?
            .skills;
        let mut fitting_type_ids = HashMap::new();
        for (type_id, dg_skills) in dogma_skills {
            let mut can_use = true;
            for skill in dg_skills {
                if !can_use {
                    break;
                }

                can_use = skills
                    .iter()
                    .find(|x| x.skill_id == *skill.skill_id)
                    .map(|x| x.active_skill_level as u8 >= skill.level)
                    .unwrap_or(false);
            }
            fitting_type_ids.insert(type_id, can_use);
        }

        let mut result = Vec::new();
        for fitting in fittings {
            let mut f = Vec::new();
            for x in fitting.fittings {
                let mut type_ids = Vec::new();
                for y in x.type_ids {
                    let can_use = fitting_type_ids
                        .get(&y)
                        .cloned()
                        .unwrap_or(false);
                    type_ids.push(FittingTypeId {
                        can_use,
                        type_id: y
                    });
                }

                f.push(Fitting {
                    type_ids,
                    name: x.name
                });
            }
            result.push(AllianceFitting {
                fittings: f,
                how_to_fit: fitting.how_to_fit,
                how_to_fly: fitting.how_to_fly,
                id: Some(fitting.id),
                name: fitting.name,
                url: fitting.url
            });
        }

        Ok(result)
    }

    pub async fn get_fitting(
        &self,
        _token: Uuid,
        id:     Uuid,
    ) -> Result<Option<AllianceFittingEntry>, EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .get::<_, Uuid, AllianceFittingEntry>(CacheName::AllianceFitting, id)
            .await
            .map_err(Into::into)
    }

    pub async fn set_fitting(
        &self,
        _token: Uuid,
        entry:  NewAllianceFitting,
    ) -> Result<(), EveServerError> {
        let id = Uuid::new_v4();
        let entry = AllianceFittingEntry {
            id,
            url: entry.url,
            name: entry.name,
            how_to_fly: entry.how_to_fly,
            how_to_fit: entry.how_to_fit,
            fittings: entry.fittings
        };

        self
            .pool
            .acquire()
            .await?
            .set(CacheName::AllianceFitting, id, entry)
            .await
            .map(drop)
            .map_err(Into::into)
    }

    pub async fn del_fitting(
        &self,
        _token: Uuid,
        id:     Uuid,
    ) -> Result<(), EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .del(CacheName::AllianceFitting, id)
            .await
            .map_err(Into::into)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AllianceFitting {
    pub fittings:   Vec<Fitting>,
    pub name:       String,
    pub url:        String,

    pub id:         Option<Uuid>,
    pub how_to_fit: Option<String>,
    pub how_to_fly: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fitting {
    pub name:     String,
    pub type_ids: Vec<FittingTypeId>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FittingTypeId {
    pub type_id: TypeId,
    pub can_use: bool
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewAllianceFitting {
    pub fittings:   Vec<caph_db::Fitting>,
    pub name:       String,
    pub url:        String,

    pub how_to_fit: Option<String>,
    pub how_to_fly: Option<String>,
}
