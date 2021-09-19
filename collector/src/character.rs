use crate::error::CollectorError;

use caph_connector::{CharacterId, ConnectCharacterService, EveAuthClient};
use sqlx::PgPool;
use std::fmt;
use tracing::instrument;

pub struct Character {
    pool: PgPool,
}

impl Character {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Runs a task in the background that periodically collects all market
    /// entries from all markets and writes them into the database.
    pub async fn task(&mut self) -> Result<(), CollectorError> {
        struct CharacterEntry {
            character_id:  CharacterId,
            refresh_token: String
        }

        let tokens = sqlx::query!("
                SELECT character_id, refresh_token
                FROM login
                WHERE character_id IS NOT NULL AND refresh_token IS NOT NULL
            ")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                CharacterEntry {
                    character_id:  x.character_id.unwrap().into(),
                    refresh_token: x.refresh_token.unwrap()
                }
            })
            .collect::<Vec<_>>();

        let cids = tokens.iter().map(|x| *x.character_id).collect::<Vec<_>>();
        sqlx::query!("
                DELETE FROM asset WHERE character_id = ANY($1)
            ", &cids)
            .execute(&self.pool)
            .await?;

        for token in tokens {
            let client = EveAuthClient::new(token.refresh_token).unwrap();

            self.assets(
                client.clone(),
                &token.character_id,
            ).await?;
            self.blueprints(
                client,
                &token.character_id,
            ).await?;
        }

        Ok(())
    }

    #[instrument]
    async fn assets(
        &self,
        client: EveAuthClient,
        cid:    &CharacterId,
    ) -> Result<(), CollectorError> {
        let character_service = ConnectCharacterService::new(client, *cid);
        let assets = character_service.assets().await?;

        let item_id = assets.iter().map(|x| x.item_id).map(|x| *x).collect::<Vec<_>>();
        let location_id = assets.iter().map(|x| x.location_id).map(|x| *x).collect::<Vec<_>>();
        let quantity = assets.iter().map(|x| x.quantity).map(|x| x).collect::<Vec<_>>();
        let type_id = assets.iter().map(|x| x.type_id).map(|x| *x).collect::<Vec<_>>();

        sqlx::query!("
                INSERT INTO asset
                (
                    character_id,

                    item_id,
                    location_id,
                    quantity,
                    type_id
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[],
                    $3::BIGINT[],
                    $4::INTEGER[],
                    $5::INTEGER[]
                )
            ",
                **cid,
                &item_id,
                &location_id,
                &quantity,
                &type_id,
            )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn blueprints(
        &self,
        client: EveAuthClient,
        cid:    &CharacterId,
    ) -> Result<(), CollectorError> {
        let character_service = ConnectCharacterService::new(client, *cid);
        let bps = character_service.blueprints().await.unwrap();

        let item_id = bps.iter().map(|x| x.item_id).map(|x| *x).collect::<Vec<_>>();
        let quantity = bps.iter().map(|x| x.quantity).collect::<Vec<_>>();
        let m_eff = bps.iter().map(|x| x.material_efficiency).collect::<Vec<_>>();
        let t_eff = bps.iter().map(|x| x.time_efficiency).collect::<Vec<_>>();
        let runs = bps.iter().map(|x| x.runs).collect::<Vec<_>>();

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
}

impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Character").finish()
    }
}
