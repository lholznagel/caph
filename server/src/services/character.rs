use crate::error::EveServerError;

use cachem::v2::ConnectionPool;
use caph_db_v2::{CacheName, UserEntry};
use caph_eve_data_wrapper::{CharacterAssetName, CharacterBlueprint, CharacterSkill, CharacterSkillQueue, EveClient, EveConnectError, EveDataWrapper, EveOAuthUser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone)]
pub struct CharacterService {
    eve:  EveDataWrapper,
    pool: ConnectionPool,
}

impl CharacterService {
    pub fn new(eve: EveDataWrapper, pool: ConnectionPool) -> Self {
        Self {
            eve,
            pool,
        }
    }

    pub async fn save_login(
        &self,
        character: EveOAuthUser,
    ) -> Result<(), EveServerError> {
        if let Some(x) = self.lookup(character.user_id).await? {
            let entry = UserEntry {
                access_token: character.access_token,
                refresh_token: character.refresh_token,
                ..x
            };

            self
                .pool
                .acquire()
                .await?
                .set(CacheName::User, x.user_id, entry)
                .await?;
        } else {
            let entry = UserEntry {
                access_token: character.access_token,
                refresh_token: character.refresh_token,
                user_id: character.user_id,
                name: String::new(),
                aliase: Vec::new(),
                token: String::new()
            };

            self
                .pool
                .acquire()
                .await?
                .set(CacheName::User, character.user_id, entry)
                .await?;
        }

        Ok(())
    }

    pub async fn lookup(
        &self,
        cid: u32,
    ) -> Result<Option<UserEntry>, EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .get::<_, _, UserEntry>(CacheName::User, cid)
            .await
            .map_err(Into::into)
    }

    pub async fn name(
        &self,
        character_id: u32,
    ) -> Result<String, EveServerError> {
        let oauth = self.lookup(character_id).await?.ok_or(EveServerError::UserNotFound)?;
        let charater_service = self.eve.character().await?;

        let whoami = charater_service.whoami(&oauth.access_token, character_id).await;
        let name = if let Err(EveConnectError::Unauthorized) = whoami {
            let user = EveClient::retrieve_refresh_token(&oauth.refresh_token)
                .await
                .map_err(EveServerError::from)?;

            self.save_login(user.clone()).await?;

            charater_service.whoami(&user.access_token, character_id)
                .await
                .map_err(EveServerError::from)?
        } else if let Ok(x) = whoami {
            x
        } else {
            return Err(EveServerError::EveConnectError(EveConnectError::Unauthorized).into());
        };

        Ok(name)
    }

    pub async fn portrait(
        &self,
        character_id: u32,
    ) -> Result<String, EveServerError> {
        let oauth = self.lookup(character_id).await?.ok_or(EveServerError::UserNotFound)?;
        let character_service = self.eve.character().await?;

        let portrait = character_service.portrait(&oauth.access_token, character_id).await;
        let name = if let Err(EveConnectError::Unauthorized) = portrait {
            let user = EveClient::retrieve_refresh_token(&oauth.refresh_token)
                .await
                .map_err(EveServerError::from)?;

            self.save_login(user.clone()).await?;

            character_service.portrait(&user.access_token, character_id)
                .await
                .map_err(EveServerError::from)?
        } else if let Ok(x) = portrait {
            x
        } else {
            return Err(EveServerError::EveConnectError(EveConnectError::Unauthorized).into());
        };

        Ok(name)
    }

    pub async fn assets(
        &self,
        character_id: u32,
    ) -> Result<Vec<CharacterAsset>, EveServerError> {
        let oauth = self.lookup(character_id).await?.ok_or(EveServerError::UserNotFound)?;
        let character_service = self.eve.character().await?;

        let assets = character_service.assets(&oauth.access_token, character_id).await;
        let assets = if let Err(EveConnectError::Unauthorized) = assets {
            let user = EveClient::retrieve_refresh_token(&oauth.refresh_token)
                .await
                .map_err(EveServerError::from)?;

            self.save_login(user.clone()).await?;

            character_service.assets(&user.access_token, character_id)
                .await
                .map_err(EveServerError::from)?
        } else if let Ok(x) = assets {
            x
        } else {
            return Err(EveServerError::EveConnectError(EveConnectError::Unauthorized).into());
        };

        let asset_location = |
            asset: &caph_eve_data_wrapper::CharacterAsset,
            x:     &mut Vec<AssetLocation>
        | -> Vec<AssetLocation> {
            // Try to find an already existing location
            let location = x
                .iter_mut()
                .find(|x| x.item_id == asset.location_id);

            // If it exists, add the quantity
            if let Some(mut location) = location {
                location.quantity += asset.quantity;
            } else {
                // otherwise add the location
                x.push(AssetLocation {
                    item_id:  asset.location_id,
                    quantity: asset.quantity,
                    typ:      asset.location_flag.clone(),
                });
            }
            x.to_vec()
        };

        let mut result = HashMap::new();
        for asset in assets {
            result
                .entry(asset.type_id)
                .and_modify(|x: &mut CharacterAsset| {
                    x.item_ids.push(asset.item_id);
                    x.locations = asset_location(&asset, &mut x.locations);
                    x.quantity += asset.quantity;
                })
                .or_insert(CharacterAsset::from(asset));
        }

        let result = result
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(result)
    }

    pub async fn asset_names(
        &self,
        character_id: u32,
        ids: Vec<u64>
    ) -> Result<Vec<CharacterAssetName>, EveServerError> {
        let oauth = self.lookup(character_id).await?.ok_or(EveServerError::UserNotFound)?;
        let character_service = self.eve.character().await?;

        let names = character_service.asset_names(&oauth.access_token, character_id, ids.clone()).await;
        let names = if let Err(EveConnectError::Unauthorized) = names {
            let user = EveClient::retrieve_refresh_token(&oauth.refresh_token)
                .await
                .map_err(EveServerError::from)?;

            self.save_login(user.clone()).await?;

            character_service.asset_names(&user.access_token, character_id, ids)
                .await
                .map_err(EveServerError::from)?
        } else if let Ok(x) = names {
            x
        } else {
            return Err(EveServerError::EveConnectError(EveConnectError::Unauthorized).into());
        };

        Ok(names)
    }

    pub async fn blueprints(
        &self,
        character_id: u32,
    ) -> Result<Vec<CharacterBlueprint>, EveServerError> {
        let oauth = self.lookup(character_id).await?.ok_or(EveServerError::UserNotFound)?;
        let character_service = self.eve.character().await?;

        let blueprints = character_service.blueprints(&oauth.access_token, character_id).await;
        let blueprints = if let Err(EveConnectError::Unauthorized) = blueprints {
            let user = EveClient::retrieve_refresh_token(&oauth.refresh_token)
                .await
                .map_err(EveServerError::from)?;

            self.save_login(user.clone()).await?;

            character_service.blueprints(&user.access_token, character_id)
                .await
                .map_err(EveServerError::from)?
        } else if let Ok(x) = blueprints {
            x
        } else {
            return Err(EveServerError::EveConnectError(EveConnectError::Unauthorized).into());
        };

        Ok(blueprints)
    }

    pub async fn skills(
        &self,
        character_id: u32,
    ) -> Result<Vec<CharacterSkill>, EveServerError> {
        let oauth = self.lookup(character_id).await?.ok_or(EveServerError::UserNotFound)?;
        let character_service = self.eve.character().await?;

        let skills = character_service.skills(&oauth.access_token, character_id).await;
        let skills = if let Err(EveConnectError::Unauthorized) = skills {
            let user = EveClient::retrieve_refresh_token(&oauth.refresh_token)
                .await
                .map_err(EveServerError::from)?;

            self.save_login(user.clone()).await?;

            character_service.skills(&user.access_token, character_id)
                .await
                .map(|x| x.skills)
                .map_err(EveServerError::from)?
        } else if let Ok(x) = skills {
            x.skills
        } else {
            return Err(EveServerError::EveConnectError(EveConnectError::Unauthorized).into());
        };

        Ok(skills)
    }

    pub async fn skillqueue(
        &self,
        character_id: u32,
    ) -> Result<Vec<CharacterSkillQueue>, EveServerError> {
        let oauth = self.lookup(character_id).await?.ok_or(EveServerError::UserNotFound)?;
        let character_service = self.eve.character().await?;

        let skills = character_service.skillqueue(&oauth.access_token, character_id).await;
        let skills = if let Err(EveConnectError::Unauthorized) = skills {
            let user = EveClient::retrieve_refresh_token(&oauth.refresh_token)
                .await
                .map_err(EveServerError::from)?;

            self.save_login(user.clone()).await?;

            character_service.skillqueue(&user.access_token, character_id)
                .await
                .map_err(EveServerError::from)?
        } else if let Ok(x) = skills {
            x
        } else {
            return Err(EveServerError::EveConnectError(EveConnectError::Unauthorized).into());
        };

        Ok(skills)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterAsset {
    pub item_ids:  Vec<u64>,
    pub quantity:  u32,
    pub type_id:   u32,
    pub locations: Vec<AssetLocation>
}

impl From<caph_eve_data_wrapper::CharacterAsset> for CharacterAsset {
    fn from(x: caph_eve_data_wrapper::CharacterAsset) -> Self {
        Self {
            item_ids:  vec![x.item_id],
            quantity:  x.quantity,
            type_id:   x.type_id,
            locations: vec![
                AssetLocation {
                    item_id:  x.location_id,
                    quantity: x.quantity,
                    typ:      x.location_flag,
                }
            ],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetLocation {
    pub item_id:  u64,
    pub quantity: u32,
    pub typ:      String,
}
