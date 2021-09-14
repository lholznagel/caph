use crate::error::CollectorError;

use caph_eve_data_wrapper::{CharacterId, CharacterService, EveClient, EveDataWrapper, EveOAuthUser};
use sqlx::PgPool;


pub struct Character {
    eve:  EveDataWrapper,
    pool: PgPool,
}

impl Character {
    pub fn new(eve: EveDataWrapper, pool: PgPool) -> Self {
        Self {
            eve,
            pool
        }
    }

    /// Runs a task in the background that periodically collects all market
    /// entries from all markets and writes them into the database.
    pub async fn task(&mut self) -> Result<(), CollectorError> {
        tracing::info!("Loading eve services");
        let character_service = self.eve.character().await?;
        tracing::info!("Services loaded");

        let refresh_tokens = sqlx::query!("
                SELECT refresh_token
                FROM login
                WHERE character_id IS NOT NULL AND refresh_token IS NOT NULL
            ")
            .fetch_all(&self.pool)
            .await?;
        let mut tokens = Vec::new();
        for refresh_token in refresh_tokens {
            let token = self.refresh_token(&refresh_token.refresh_token.unwrap()).await?;
            tokens.push(token);
        }

        let cids = tokens.iter().map(|x| *x.user_id as i32).collect::<Vec<_>>();
        sqlx::query!("
                DELETE FROM asset WHERE character_id = ANY($1)
            ", &cids)
            .execute(&self.pool)
            .await?;

        for token in tokens {
            self.assets(
                token.access_token.clone(),
                token.user_id,
                character_service.clone()
            ).await?;
            self.blueprints(
                token.access_token.clone(),
                token.user_id,
                character_service.clone()
            ).await?;
        }

        Ok(())
    }

    async fn assets(
        &self,
        token:             String,
        cid:               CharacterId,
        character_service: CharacterService,
    ) -> Result<(), CollectorError> {
        let assets = character_service
            .assets(&token, cid)
            .await?;

        let item_id = assets.iter().map(|x| x.item_id).map(|x| *x as i64).collect::<Vec<_>>();
        let location_flag = assets.iter().map(|x| x.clone().location_flag).collect::<Vec<_>>();
        let location_id = assets.iter().map(|x| x.location_id).map(|x| *x as i64).collect::<Vec<_>>();
        let quantity = assets.iter().map(|x| x.quantity).map(|x| x as i32).collect::<Vec<_>>();
        let type_id = assets.iter().map(|x| x.type_id).map(|x| *x as i32).collect::<Vec<_>>();

        sqlx::query!("
                INSERT INTO asset
                (
                    character_id,

                    item_id,
                    location_id,
                    quantity,
                    type_id,
                    location_flag
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[],
                    $3::BIGINT[],
                    $4::INTEGER[],
                    $5::INTEGER[],
                    $6::VARCHAR[]
                )
            ",
                *cid as i32,
                &item_id,
                &location_id,
                &quantity,
                &type_id,
                &location_flag,
            )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn blueprints(
        &self,
        token:             String,
        cid:               CharacterId,
        character_service: CharacterService,
    ) -> Result<(), CollectorError> {
        let bps = character_service
            .blueprints(&token, cid)
            .await?;

        let item_id = bps.iter().map(|x| x.item_id).map(|x| *x as i64).collect::<Vec<_>>();
        let quantity = bps.iter().map(|x| x.quantity).map(|x| x as i32).collect::<Vec<_>>();
        let m_eff = bps.iter().map(|x| x.material_efficiency).map(|x| x as i32).collect::<Vec<_>>();
        let t_eff = bps.iter().map(|x| x.time_efficiency).map(|x| x as i32).collect::<Vec<_>>();
        let runs = bps.iter().map(|x| x.runs).map(|x| x as i32).collect::<Vec<_>>();

        sqlx::query!("
                INSERT INTO asset_blueprint
                (
                    item_id,

                    quantity,
                    material_efficiency,
                    time_efficiency,
                    runs
                )
                SELECT * FROM UNNEST(
                    $1::BIGINT[],
                    $2::INTEGER[],
                    $3::INTEGER[],
                    $4::INTEGER[],
                    $5::INTEGER[]
                )
            ",
                &item_id,
                &quantity,
                &m_eff,
                &t_eff,
                &runs,
            )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn refresh_token(&self, token: &str) -> Result<EveOAuthUser, CollectorError> {
        let oauth = EveClient::retrieve_refresh_token(&token)
            .await
            .unwrap();

        Ok(oauth)
    }
}

