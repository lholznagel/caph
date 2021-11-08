use crate::error::CollectorError;

use caph_connector::{CharacterId, CorporationId, ConnectCharacterService, EveAuthClient};
use futures::stream::*;
use sqlx::{PgPool, Postgres, Transaction};
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
        #[derive(Clone)]
        struct CharacterEntry {
            /// [CharacterId] of the character
            character_id:   CharacterId,
            /// [CharacterId] of the main character if any
            character_main: Option<CharacterId>,
            /// Id of the corporation the character is in
            corporation_id: CorporationId,
            /// Refresh token for the EVE-API
            refresh_token: String
        }

        let tokens = sqlx::query!(r#"
                SELECT
                    c.character_main,
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
                    character_main: x.character_main.map(|x| x.into()),
                    corporation_id: x.corporation_id.into(),
                    refresh_token:  x.refresh_token
                }
            })
            .collect::<Vec<_>>();

        // TODO: group all toons from a character together
        //       create an eve auth client for each character
        // CharacterId -> If of the main character
        // Vec<CharacterReq> -> list contianing the main and all toons
        let mut characters: HashMap<CharacterId, Vec<CharacterReq>> = HashMap::new();

        for token in tokens.clone() {
            let character_main = token.character_main.unwrap_or(token.character_id);
            let entry = CharacterReq::new(token.refresh_token, token.character_id, token.corporation_id).await?;
            characters
                .entry(character_main)
                .and_modify(|x: &mut Vec<CharacterReq>| {
                    x.push(entry.clone());
                })
                .or_insert(vec![entry]);
        }

        let mut char_proc = FuturesUnordered::new();
        let characters = Characters::new(characters);
        for character in characters {
            let e = self.fetch_characters(character);
            char_proc.push(e);
        }

        while let Some(x) = char_proc.next().await {
            // nothing to do, we just want to wait until all characters are
            // fetched
        }

        Ok(())
    }

    async fn fetch_characters(
        &self,
        characters: Characters
    ) -> Result<(), CollectorError> {
        let cids = characters.character_ids();

        // We can only execute one command at a time
        let x = sqlx::query!("
                DELETE FROM asset CASCADE WHERE character_id = ANY($1)
            ",
                &cids
            )
            .execute(&self.pool)
            .await
            .map_err(CollectorError::DeletingCharacterAssets)?;
        sqlx::query!("
                DELETE FROM asset_name CASCADE WHERE character_id = ANY($1)
            ",
                &cids
            )
            .execute(&self.pool)
            .await
            .map_err(CollectorError::DeletingCharacterAssetNames)?;
        sqlx::query!("
                DELETE FROM asset_blueprint CASCADE WHERE character_id = ANY($1)
            ",
                &cids
            )
            .execute(&self.pool)
            .await
            .map_err(CollectorError::DeletingCharacterBlueprints)?;
        sqlx::query!("
                DELETE FROM industry_job CASCADE WHERE character_id = ANY($1)
            ",
                &cids
            )
            .execute(&self.pool)
            .await
            .map_err(CollectorError::DeletingCharacterIndustryJobs)?;

        for character in characters.0 {
            let x = self.assets(
                &character.client,
                character.character_id,
            )
            .await;
            if let Err(e) = x {
                log::error!("Failed task asset_names for [{}]. {:?}", character.character_id, e)
            }

            let x = self.blueprints(
                &character.client,
                character.character_id,
            )
            .await;
            if let Err(e) = x {
                log::error!("Failed task asset_names for [{}]. {:?}", character.character_id, e)
            }

            let x = self.industry_jobs(
                &character.client,
                character.character_id,
                character.corporation_id,
            )
            .await;
            if let Err(e) = x {
                log::error!("Failed task asset_names for [{}]. {:?}", character.character_id, e)
            }

            let x = self.asset_names(
                &character.client,
                character.character_id,
            )
            .await;
            if let Err(e) = x {
                log::error!("Failed task asset_names for [{}]. {:?}", character.character_id, e)
            }
        }

        Ok(())
    }

    #[instrument]
    async fn assets(
        &self,
        client:    &EveAuthClient,
        cid:       CharacterId,
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
        client:    &EveAuthClient,
        cid:       CharacterId,
    ) -> Result<(), CollectorError> {
        let iids = sqlx::query!("
                SELECT item_id
                FROM asset a
                JOIN item i
                  ON a.type_id = i.type_id
                WHERE a.character_id = $1
                  AND (i.category_id = 2 OR i.category_id = 6)
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
        .unwrap();
        //.map_err(CollectorError::InsertingCharacterAssetNames)?;

        Ok(())
    }

    #[instrument]
    async fn blueprints(
        &self,
        client:    &EveAuthClient,
        cid:       CharacterId,
    ) -> Result<(), CollectorError> {
        let bps = ConnectCharacterService::new(client, cid)
            .blueprints()
            .await
            .map_err(CollectorError::CouldNotGetCharacterBlueprints)?;

        let item_ids = bps.iter().map(|x| *x.item_id).collect::<Vec<_>>();
        let quantities = bps.iter().map(|x| x.quantity).collect::<Vec<_>>();
        let m_eff = bps.iter().map(|x| x.material_efficiency).collect::<Vec<_>>();
        let t_eff = bps.iter().map(|x| x.time_efficiency).collect::<Vec<_>>();
        let runs = bps.iter().map(|x| x.runs).collect::<Vec<_>>();
        let type_ids = bps.iter().map(|x| *x.type_id).collect::<Vec<_>>();

        // FIXME: the data may not exist yet
        let type_ids_pre = sqlx::query!(r#"
                SELECT
                    b.type_id   AS "type_id!",
                    bm.ptype_id AS "ptype_id!"
                FROM blueprint b
                JOIN blueprint_material bm
                  ON b.id = bm.blueprint
                WHERE b.type_id = ANY($1)
                  AND bm.is_product = TRUE
                  AND bm.activity != 1
            "#,
                &type_ids
            )
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| (x.type_id, x.ptype_id))
            .collect::<HashMap<_, _>>();

        let mut ptype_ids = Vec::new();
        for type_id in type_ids.iter() {
            ptype_ids.push(*type_ids_pre.get(type_id).unwrap());
        }

        sqlx::query!("
            INSERT INTO asset_blueprint
            (
                character_id,

                item_id,

                quantity,
                material_efficiency,
                time_efficiency,
                runs,
                type_id,
                ptype_id
            )
            SELECT $1, * FROM UNNEST(
                $2::BIGINT[],
                $3::INTEGER[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::INTEGER[],
                $7::INTEGER[],
                $8::INTEGER[]
            )
        ",
            *cid,
            &item_ids,
            &quantities,
            &m_eff,
            &t_eff,
            &runs,
            &type_ids,
            &ptype_ids
        )
        .execute(&self.pool)
        .await
        .map_err(CollectorError::InsertingCharacterBlueprints)?;

        Ok(())
    }

    #[instrument]
    async fn industry_jobs(
        &self,
        client:    &EveAuthClient,
        char_id:   CharacterId,
        corp_id:   CorporationId,
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

/// List of all alt accounts of a main account
struct Characters(Vec<CharacterReq>);

impl Characters {
    pub fn new(characters: HashMap<CharacterId, Vec<CharacterReq>>) -> Vec<Characters> {
        let mut entries = Vec::new();
        for (_, e) in characters {
            entries.push(Characters(e));
        }
        entries
    }

    pub fn character_ids(&self) -> Vec<i32> {
        self.0
            .iter()
            .map(|x| *x.character_id)
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
struct CharacterReq {
    client:         EveAuthClient,
    character_id:   CharacterId,
    corporation_id: CorporationId,
}

impl CharacterReq {
    pub async fn new(refresh_token: String, character_id: CharacterId, corporation_id: CorporationId) -> Result<Self, CollectorError> {
        let client = EveAuthClient::new(refresh_token)
            .map_err(CollectorError::CouldNotCreateEveClient)?;
        Ok(Self {
            client,
            character_id,
            corporation_id,
        })
    }
}
