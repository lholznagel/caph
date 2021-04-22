use std::collections::HashMap;

use cachem::{ConnectionPool, EmptyMsg, Protocol};
use caph_db::{FetchUserReq, FetchUserRes, InsertUserReq, UserEntry};
use caph_eve_data_wrapper::{CharacterAsset, CharacterBlueprint, EveClient, EveConnectError, EveDataWrapper, EveOAuthUser};

use crate::error::EveServerError;

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
        let mut conn = self.pool.acquire().await?;

        if let Some(x) = self.lookup(character.user_id).await? {
            Protocol::request::<_, EmptyMsg>(
                &mut conn,
                InsertUserReq(UserEntry {
                    access_token: character.access_token,
                    refresh_token: character.refresh_token,
                    ..x
                })
            )
            .await?;
        } else {
            Protocol::request::<_, EmptyMsg>(
                &mut conn,
                InsertUserReq(UserEntry {
                    access_token: character.access_token,
                    refresh_token: character.refresh_token,
                    user_id: character.user_id,
                    name: String::new(),
                    aliase: Vec::new(),
                })
            )
            .await?;
        }

        Ok(())
    }

    pub async fn lookup(
        &self,
        character_id: u32,
    ) -> Result<Option<UserEntry>, EveServerError> {
        let mut conn = self.pool.acquire().await?;

        Protocol::request::<_, FetchUserRes>(
            &mut conn,
            FetchUserReq(character_id)
        )
        .await
        .map(|x| {
            match x {
                FetchUserRes::Ok(x) => Some(x),
                _ => None
            }
        })
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

        let mut result = HashMap::new();
        for asset in assets {
            result
                .entry(asset.type_id)
                .and_modify(|x: &mut CharacterAsset| x.quantity += asset.quantity)
                .or_insert(asset);
        }

        let result = result
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(result)
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
}
