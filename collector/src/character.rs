use crate::error::CollectorError;

use caph_connector::{CharacterId, ConnectCharacterService, EveAuthClient};
use sqlx::PgPool;
use std::collections::HashMap;
use std::fmt;
use tracing::instrument;

/// Responsible for loading all needed character information
pub struct Character {
    /// Connection pool to a postgres database
    pool: PgPool,
}

impl Character {
    /// Creates a new instance
    ///
    /// # Params
    ///
    /// * `pool` -> Connection pool to a postgres
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

    /// Runs a task in the background that periodically collects all market
    /// entries from all markets and writes them into the database.
    ///
    /// # Errors
    ///
    /// If there is failure during the process. This causes everything to stop
    /// and it will retry on the next interval
    ///
    /// # Panics
    ///
    /// If the character id or refresh token is not set. This should never
    /// happen because we check in the SQL-Query that those fields are set.
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    pub async fn task(&mut self) -> Result<(), CollectorError> {
        /// Represents a character entry with its character id and refresh token
        struct CharacterEntry {
            /// Character ID of the character
            character_id:  CharacterId,
            /// Refresh token for the EVE-API
            refresh_token: String
        }

        let tokens = sqlx::query!("
                SELECT character_id, refresh_token
                FROM login
                WHERE character_id IS NOT NULL AND refresh_token IS NOT NULL
            ")
            .fetch_all(&self.pool)
            .await
            .map_err(CollectorError::SelectCharacterEntries)?
            .into_iter()
            .map(|x| {
                CharacterEntry {
                    character_id:  x.character_id
                        .expect("The character id should be set.")
                        .into(),
                    refresh_token: x.refresh_token
                        .expect("The refresh token should be set.")
                }
            })
            .collect::<Vec<_>>();

        let cids = tokens
            .iter()
            .map(|x| *x.character_id)
            .collect::<Vec<_>>();
        sqlx::query!("
                DELETE FROM asset CASCADE WHERE character_id = ANY($1)
            ", &cids)
            .execute(&self.pool)
            .await
            .map_err(CollectorError::DeletingCharacterAssets)?;

        for token in tokens {
            let client = EveAuthClient::new(token.refresh_token)
                .map_err(CollectorError::CouldNotCreateEveClient)?;

            self.assets(
                client.clone(),
                token.character_id,
            ).await?;
            self.asset_names(
                client.clone(),
                token.character_id,
            ).await?;
            self.blueprints(
                client,
                token.character_id,
            ).await?;
        }

        Ok(())
    }

    #[instrument]
    async fn assets(
        &self,
        client: EveAuthClient,
        cid:    CharacterId,
    ) -> Result<(), CollectorError> {
        let assets = ConnectCharacterService::new(client, cid)
            .assets()
            .await
            .map_err(CollectorError::CouldNotGetCharacterAssets)?;

        let asset_location = assets
            .iter()
            .map(|x| (x.item_id, x))
            .collect::<HashMap<_, _>>();

        let mut item_ids = Vec::new();
        let mut type_ids = Vec::new();
        let mut quantities = Vec::new();
        let mut location_flags = Vec::new();

        let mut location_ids = Vec::new();
        let mut reference_ids = Vec::new();

        for asset in assets.iter() {
            item_ids.push(*asset.item_id);
            type_ids.push(*asset.type_id);
            quantities.push(asset.quantity);
            location_flags.push(asset.location_flag.clone());

            if asset.location_flag == "Hangar" {
                location_ids.push(*asset.location_id);
                reference_ids.push(None);
            } else {
                let reference_id = asset.location_id;
                let location_id = asset_location
                    .get(&(*asset.location_id).into())
                    .unwrap()
                    .location_id;
                location_ids.push(*location_id);
                reference_ids.push(Some(*reference_id));
            }
        }

        sqlx::query!("
                INSERT INTO asset
                (
                    character_id,

                    type_id,
                    item_id,
                    location_id,
                    reference_id,
                    quantity,
                    location_flag
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::BIGINT[],
                    $4::BIGINT[],
                    $5::BIGINT[],
                    $6::INTEGER[],
                    $7::VARCHAR[]
                )
            ",
                *cid,
                &type_ids,
                &item_ids,
                &location_ids,
                &reference_ids as _,
                &quantities,
                &location_flags,
            )
            .execute(&self.pool)
            .await
            .map_err(CollectorError::InsertingCharacterAssets)?;

        Ok(())
    }

    #[instrument]
    async fn asset_names(
        &self,
        client: EveAuthClient,
        cid:    CharacterId,
    ) -> Result<(), CollectorError> {
        let iids = sqlx::query!("
                SELECT item_id
                FROM asset
                WHERE character_id = $1
            ",
                *cid
            )
            .fetch_all(&self.pool)
            .await
            .map_err(CollectorError::CouldNotGetCharacterAssetItemIds)?
            .into_iter()
            .map(|x| x.item_id.into())
            .collect::<Vec<_>>();

        let assets = ConnectCharacterService::new(client, cid)
            .asset_name(iids)
            .await
            .map_err(CollectorError::CouldNotGetCharacterAssetNames)?;


        let mut item_ids = Vec::new();
        let mut names = Vec::new();

        for asset in assets {
            if asset.name.is_empty() || asset.name == "None" {
                continue;
            }

            item_ids.push(*asset.item_id);
            names.push(asset.name.clone());
        }

        sqlx::query!("
                INSERT INTO asset_name
                (
                    character_id,

                    item_id,
                    name
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[],
                    $3::VARCHAR[]
                )
            ",
                *cid,
                &item_ids,
                &names,
            )
            .execute(&self.pool)
            .await
            .map_err(CollectorError::InsertingCharacterAssetNames)?;

        Ok(())
    }

    #[instrument]
    async fn blueprints(
        &self,
        client: EveAuthClient,
        cid:    CharacterId,
    ) -> Result<(), CollectorError> {
        let bps = ConnectCharacterService::new(client, cid)
            .blueprints()
            .await
            .map_err(CollectorError::CouldNotGetCharacterBlueprints)?;

        let item_id = bps.iter().map(|x| *x.item_id).collect::<Vec<_>>();
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
            .await
            .map_err(CollectorError::InsertingCharacterBlueprints)?;

        Ok(())
    }
}

impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Character").finish()
    }
}
