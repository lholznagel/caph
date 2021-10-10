use crate::error::CollectorError;

use caph_connector::{CharacterId, CorporationId, ConnectCharacterService, EveAuthClient};
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
            character_id:   CharacterId,
            /// Id of the corporation the character is in
            corporation_id: CorporationId,
            /// Refresh token for the EVE-API
            refresh_token: String
        }

        let tokens = sqlx::query!(r#"
                SELECT
                    c.character_id   AS "character_id!",
                    c.corporation_id AS "corporation_id!",
                    l.refresh_token  AS "refresh_token!"
                FROM login l
                JOIN character c
                  ON l.character_id = c.character_id
                WHERE l.character_id IS NOT NULL
                  AND refresh_token IS NOT NULL
            "#)
            .fetch_all(&self.pool)
            .await
            .map_err(CollectorError::SelectCharacterEntries)?
            .into_iter()
            .map(|x| {
                CharacterEntry {
                    character_id:   x.character_id.into(),
                    corporation_id: x.corporation_id.into(),
                    refresh_token:  x.refresh_token
                }
            })
            .collect::<Vec<_>>();

        for token in tokens {
            let client = EveAuthClient::new(token.refresh_token)
                .map_err(CollectorError::CouldNotCreateEveClient)?;

            let _ = self.assets(
                client.clone(),
                token.character_id,
            ).await;
            let _ = self.asset_names(
                client.clone(),
                token.character_id,
            ).await;
            let _ = self.blueprints(
                client.clone(),
                token.character_id,
            ).await;
            let _ = self.industry_jobs(
                client,
                token.character_id,
                token.corporation_id
            ).await;
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
                    .map(|x| x.location_id)
                    .unwrap_or(asset.location_id);
                location_ids.push(*location_id);
                reference_ids.push(Some(*reference_id));
            }
        }

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("
                DELETE FROM asset CASCADE WHERE character_id = $1
            ", *cid)
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingCharacterAssets)?;

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
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingCharacterAssets)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

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

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("
                DELETE FROM asset_name CASCADE WHERE character_id = $1
            ", *cid)
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingCharacterAssets)?;

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
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingCharacterAssetNames)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

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

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("
                DELETE FROM asset_blueprint CASCADE WHERE character_id = $1
            ", *cid)
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingCharacterAssets)?;

        sqlx::query!("
                INSERT INTO asset_blueprint
                (
                    character_id,

                    item_id,

                    quantity,
                    material_efficiency,
                    time_efficiency,
                    runs
                )
                SELECT $1, * FROM UNNEST(
                    $2::BIGINT[],
                    $3::INTEGER[],
                    $4::INTEGER[],
                    $5::INTEGER[],
                    $6::INTEGER[]
                )
            ",
                *cid,
                &item_id,
                &quantity,
                &m_eff,
                &t_eff,
                &runs,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingCharacterBlueprints)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

        Ok(())
    }

    #[instrument]
    async fn industry_jobs(
        &self,
        client:  EveAuthClient,
        char_id: CharacterId,
        corp_id: CorporationId
    ) -> Result<(), CollectorError> {
        let jobs = ConnectCharacterService::new(client, char_id)
            .industry_jobs(corp_id)
            .await
            .map_err(CollectorError::CouldNotGetCharacterIndustryJobs)?;

        let job_ids = jobs.iter().map(|x| *x.job_id).collect::<Vec<_>>();
        let type_ids = jobs.iter().map(|x| *x.type_id).collect::<Vec<_>>();
        let activities = jobs.iter().map(|x| x.activity).collect::<Vec<_>>();
        let station_ids = jobs.iter().map(|x| *x.station_id).collect::<Vec<_>>();
        let end_dates = jobs.iter().map(|x| x.end_date.clone()).collect::<Vec<_>>();
        let start_dates = jobs.iter().map(|x| x.start_date.clone()).collect::<Vec<_>>();

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("
                DELETE FROM industry_job CASCADE WHERE character_id = $1
            ", *char_id)
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingCharacterIndustryJobs)?;

        sqlx::query!("
                INSERT INTO industry_job
                (
                    character_id,
                    job_id,
                    type_id,

                    activity,

                    station_id,

                    end_date,
                    start_date
                )
                SELECT $1, * FROM UNNEST(
                    $2::INTEGER[],
                    $3::INTEGER[],
                    $4::INTEGER[],
                    $5::BIGINT[],
                    $6::VARCHAR[],
                    $7::VARCHAR[]
                )
            ",
                *char_id,
                &job_ids,
                &type_ids,
                &activities,
                &station_ids,
                &end_dates,
                &start_dates
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingCharacterBlueprints)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

        Ok(())
    }
}

impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Character").finish()
    }
}
