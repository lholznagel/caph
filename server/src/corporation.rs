use std::collections::HashMap;

use cachem::v2::ConnectionPool;
use caph_db_v2::{CacheName, CharacterAssetEntry, CharacterBlueprintEntry, CorporationBlueprintEntry};
use caph_eve_data_wrapper::EveDataWrapper;
use caph_eve_data_wrapper::ItemLocation;
use caph_eve_data_wrapper::{CharacterId, CorporationId, ItemId};
use serde::Serialize;
use uuid::Uuid;

use crate::error::EveServerError;
use crate::eve::EveAuthService;

/// Service for all character related interfaces
#[derive(Clone)]
pub struct CorporationService {
    pool:     ConnectionPool,
    eve_auth: EveAuthService,
}

impl CorporationService {
    /// Creates a new instance
    pub fn new(
        pool:     ConnectionPool,
        eve_auth: EveAuthService,
    ) -> Self {
        Self {
            pool,
            eve_auth,
        }
    }

    pub async fn blueprints(
        &self,
        cid:   CorporationId,
        token: String
    ) -> Result<Vec<CorporationBlueprintEntry>, EveServerError> {
        let mut pool = self
            .pool
            .acquire()
            .await?;

        let char_id = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?
            .user_id;

        let blueprint_ids = pool
            .keys::<_, Uuid>(CacheName::CorporationBlueprint)
            .await?;
        let blueprints = pool
            .mget::<_, _, CorporationBlueprintEntry>(CacheName::CorporationBlueprint, blueprint_ids)
            .await?
            .into_iter()
            .flatten()
            .filter(|x| x.char_id == char_id && x.corp_id == cid)
            .collect::<Vec<_>>();
        Ok(blueprints)
    }

    pub async fn set_blueprints(
        &self,
        _:          CorporationId,
        blueprints: Vec<CorporationBlueprintEntry>,
        token:      String,
    ) -> Result<(), EveServerError> {
        let user = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;

        let mut blueprints = blueprints;
        let blueprints = blueprints
            .iter_mut()
            .map(|x| {
                x.id      = Uuid::new_v4();
                x.char_id = user.user_id;
                x.corp_id = user.corp_id;
                x
            })
            .map(|x| x.clone())
            .map(|x| (x.id, x))
            .collect::<HashMap<Uuid, CorporationBlueprintEntry>>();

        self
            .pool
            .acquire()
            .await?
            .mset(CacheName::CorporationBlueprint, blueprints)
            .await
            .map(drop)
            .map_err(Into::into)
    }

    pub async fn delete_blueprints(
        &self,
        cid:   CorporationId,
        token: String
    ) -> Result<(), EveServerError> {
        let mut pool = self
            .pool
            .acquire()
            .await?;

        let char_id = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?
            .user_id;

        let blueprint_ids = pool
            .keys::<_, Uuid>(CacheName::CorporationBlueprint)
            .await?;

        if blueprint_ids.is_empty() {
            return Ok(());
        }

        let blueprint_ids = pool
            .get::<_, _, CorporationBlueprintEntry>(CacheName::CorporationBlueprint, blueprint_ids)
            .await?
            .into_iter()
            .filter(|x| x.char_id == char_id && x.corp_id == cid)
            .map(|x| x.id)
            .collect::<Vec<_>>();
        pool
            .mdel(CacheName::CorporationBlueprint, blueprint_ids)
            .await
            .map(drop)
            .map_err(Into::into)
    }
}
