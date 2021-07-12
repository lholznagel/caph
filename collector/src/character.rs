use crate::error::CollectorError;

use cachem::v2::ConnectionPool;
use caph_db_v2::{CacheName, CharacterAssetEntry, CharacterBlueprintEntry, CharacterFittingEntry, UserEntry};
use caph_eve_data_wrapper::{CharacterId, CharacterService, EveClient, EveDataWrapper, EveOAuthUser, FittingId, ItemId};
use std::collections::HashMap;


pub struct Character {
    eve:  EveDataWrapper,
    pool: ConnectionPool,
}

impl Character {
    pub fn new(eve: EveDataWrapper, pool: ConnectionPool) -> Self {
        Self {
            eve,
            pool
        }
    }

    /// Runs a task in the background that periodically collects all market
    /// entries from all markets and writes them into the database.
    pub async fn task(&mut self) -> Result<(), CollectorError> {
        log::info!("Loading eve services");
        let character_service = self.eve.character().await?;
        log::info!("Services loaded");

        let mut con = self.pool.acquire().await?;
        let characters = con
            .keys::<_, CharacterId>(CacheName::User)
            .await
            .unwrap_or_default();
        let characters = con
            .mget::<_, _, UserEntry>(CacheName::User, characters)
            .await
            .unwrap();
        let mut tokens = Vec::new();
        for character in characters {
            let character = character.unwrap();
            let token = self.refresh_token(&character.refresh_token).await?;
            tokens.push(token);

            for alt in character.aliase {
                let token = self.refresh_token(&alt.refresh_token).await?;
                tokens.push(token);
            }
        }

        for token in tokens {
            let _ = tokio::join! {
                self.assets(
                    token.access_token.clone(),
                    token.user_id,
                    character_service.clone()
                ),
                self.blueprints(
                    token.access_token.clone(),
                    token.user_id,
                    character_service.clone()
                ),
                self.fittings(
                    token.access_token,
                    token.user_id,
                    character_service.clone()
                )
            };
        }

        Ok(())
    }

    async fn assets(
        &self,
        token: String,
        user_id: CharacterId,
        character_service: CharacterService,
    ) -> Result<(), CollectorError> {
        let mut con = self.pool.acquire().await?;
        let prices = character_service
            .assets(&token, user_id)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|x| CharacterAssetEntry::from(x, user_id))
            .map(|x| (x.item_id, x))
            .collect::<HashMap<ItemId, CharacterAssetEntry>>();
        con.mset(CacheName::CharacterAsset, prices).await.unwrap();
        Ok(())
    }

    async fn blueprints(
        &self,
        token: String,
        user_id: CharacterId,
        character_service: CharacterService
    ) -> Result<(), CollectorError> {
        let mut con = self.pool.acquire().await?;
        let prices = character_service
            .blueprints(&token, user_id)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|x| CharacterBlueprintEntry::from(x, user_id))
            .map(|x| (x.item_id, x))
            .collect::<HashMap<ItemId, CharacterBlueprintEntry>>();
        con.mset(CacheName::CharacterBlueprint, prices).await.unwrap();
        Ok(())
    }

    async fn fittings(
        &self,
        token: String,
        user_id: CharacterId,
        character_service: CharacterService
    ) -> Result<(), CollectorError> {
        let mut con = self.pool.acquire().await?;
        let prices = character_service
            .fitting(&token, user_id)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|x| CharacterFittingEntry::from(x, user_id))
            .map(|x| (x.fitting_id, x))
            .collect::<HashMap<FittingId, CharacterFittingEntry>>();
        con.mset(CacheName::CharacterFitting, prices).await.unwrap();
        Ok(())
    }

    async fn refresh_token(&self, token: &str) -> Result<EveOAuthUser, CollectorError> {
        let oauth = EveClient::retrieve_refresh_token(&token)
            .await
            .unwrap();

        Ok(oauth)
    }
}

