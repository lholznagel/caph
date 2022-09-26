use caph_connector::{CharacterId, EveAuthClient, IndustryJobEntry, EveCharacterService, EveCorporationService, TypeId, ItemId, LocationId, AssetEntry, EveUniverseService};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use sqlx::PgPool;
use std::{convert::Infallible, collections::HashMap};
use warp::Filter;

use super::error::IndustryError;
use crate::{AuthCharacter, AuthCharacterInfo};

#[derive(Clone, Debug)]
pub struct IndustryService {
    pool: PgPool,
}

impl IndustryService {
    pub fn new(
        pool: PgPool,
    ) -> Self {
        Self {
            pool
        }
    }

    /// Gets all character industry jobs for the given array of characters.
    /// 
    /// # Params
    /// 
    /// * cid_client > Array of [CharacterId] and [EveAuthClient]
    /// 
    /// # Errors
    /// 
    /// - If the Eve API is not available
    /// - If the character does not have the permission
    /// 
    /// # Returns
    /// 
    /// List of all industry jobs all character are currently running
    /// 
    pub async fn character_jobs(
        &self,
        cid_client: Vec<(AuthCharacterInfo, EveAuthClient)>,
    ) -> Result<Vec<IndustryJobEntry>, IndustryError> {
        let mut jobs = Vec::new();
        for (c, client) in cid_client {
            // Ignore failed requests
            if let Ok(j) = EveCharacterService::new(c.character_id)
                .industry_jobs(&client)
                .await {

                jobs.extend(j);
            }
        }
        Ok(jobs)
    }

    /// Gets all corporation industry jobs for the given [CharacterId].
    /// 
    /// # Params
    /// 
    /// * cid_client > Array of [CharacterId] and [EveAuthClient]
    /// 
    /// # Errors
    /// 
    /// - If the Eve API is not available
    /// - If the character does not have the permission
    /// 
    /// # Returns
    /// 
    /// List of all jobs that the corporation has currently running
    /// 
    pub async fn corporation_jobs(
        &self,
        cid_client: Vec<(AuthCharacterInfo, EveAuthClient)>,
    ) -> Result<Vec<IndustryJobEntry>, IndustryError> {
        let mut jobs = Vec::new();
        for (c, client) in cid_client {
            // Ignore failed requests
            if let Ok(j) = EveCorporationService::new(c.corporation_id)
                .industry_jobs(&client)
                .await {

                jobs.extend(j);
            }
        }

        jobs.sort_by_key(|x| x.job_id);
        jobs.dedup_by_key(|x| x.job_id);
        Ok(jobs)
    }

    /// Fetches all assets for the given [CharacterId], resolves their name
    /// and saves them in the database.
    /// 
    /// The method will fail silently in the following cases:
    /// 
    /// - If the database is not available
    /// - If the EVE API is not available
    /// - If the user does not have the permission to fetch the assets
    /// 
    pub async fn character_assets(
        &self,
        cid_client: Vec<(AuthCharacterInfo, EveAuthClient)>,
    ) {
        //let mut assets = Vec::new();
        for (c, client) in cid_client {
            // Ignore errors
            if let Err(_) = AssetService::fetch_assets(
                &self.pool,
                &client,
                c.character_id,
            ).await {
                continue;
            }

            if let Err(_) = AssetService::fetch_asset_names(
                &self.pool,
                &client,
                c.character_id,
            ).await {
                continue;
            }

            if let Err(_) = AssetService::resolve_locations(
                &self.pool,
                &client,
                c.character_id,
            ).await {
                continue;
            }
        }
    }

    pub async fn corporation_assets(
        &self,
        cid_client: Vec<(AuthCharacterInfo, EveAuthClient)>,
    ) -> Result<(), IndustryError> {
        Ok(())
    }
}

/// Filter for the API.
/// 
/// # Params
/// 
/// * `pool` > Open connection to postgres
/// 
/// # Returns
/// 
/// Initialized instance of [IndustryService]
/// 
pub fn with_industry_service(
    pool: PgPool,
)  -> impl Filter<Extract = (IndustryService,), Error = Infallible> + Clone {
    warp::any()
        .map(move || IndustryService::new(pool.clone()))
}

pub struct AssetService;

impl AssetService {
    /// Removes all assets that the given [CharacterId] has stored.
    /// 
    /// # Params
    /// 
    /// * `pool` > Connection to postgres
    /// * `cid`  > [CharacterId] of the character
    /// 
    /// # Errors
    /// 
    /// - If the database is not avaialble
    /// 
    pub async fn remove_assets(
        pool: &PgPool,
        cid:  CharacterId,
    ) -> Result<(), IndustryError> {
        // Delete all assets from the character
        sqlx::query!("
                DELETE FROM assets
                WHERE character_id = $1
            ",
                *cid
            )
            .execute(pool)
            .await
            .map_err(IndustryError::DeleteAssetsError)
            .map(drop)?;

        // Delete all names for the characters assets
        sqlx::query!("
                DELETE FROM asset_names
                WHERE character_id = $1
            ",
                *cid
            )
            .execute(pool)
            .await
            .map_err(IndustryError::DeleteAssetsError)
            .map(drop)?;

        // Delete all locations for the characters assets
        sqlx::query!("
                DELETE FROM asset_locations
                WHERE character_id = $1
            ",
                *cid
            )
            .execute(pool)
            .await
            .map_err(IndustryError::DeleteAssetsError)
            .map(drop)
    }

    /// Fetches all assets that the give [CharacterId] owns and stores them in
    /// the database.
    /// 
    /// # Params
    /// 
    /// * `pool`   > Connection to postgres
    /// * `client` > Authenticated eve client
    /// * `cid`    > [CharacterId] of the character
    /// 
    /// # Errors
    /// 
    /// - If the database is not avaialble
    /// - If the EVE API request fails
    /// 
    pub async fn fetch_assets(
        pool:   &PgPool,
        client: &EveAuthClient,
        cid:    CharacterId,
    ) -> Result<(), IndustryError> {
        Self::remove_assets(pool, cid).await?;

        // Ignore failed requests
        let assets = if let Ok(assets) = EveCharacterService::new(cid)
            .assets(&client)
            .await {

            assets
        } else {
            return Ok(())
        };

        let asset_location = assets
            .iter()
            .map(|x| (x.item_id, x))
            .collect::<HashMap<_, _>>();

        let mut item_ids       = Vec::new();
        let mut type_ids       = Vec::new();
        let mut quantities     = Vec::new();
        let mut location_flags = Vec::new();

        let mut location_ids  = Vec::new();
        let mut reference_ids = Vec::new();

        for asset in assets.iter() {
            item_ids.push(*asset.item_id as i64);
            type_ids.push(*asset.type_id as i32);
            quantities.push(asset.quantity);
            location_flags.push(asset.location_flag.clone());

            if asset.location_flag == "Hangar" ||
               asset.location_flag == "Deliveries" {

                location_ids.push(*asset.location_id as i64);
                reference_ids.push(None);
            } else {
                let reference_id = asset.location_id;
                let location_id = asset_location
                    .get(&(*asset.location_id).into())
                    .unwrap()
                    .location_id;
                location_ids.push(*location_id as i64);
                reference_ids.push(Some(*reference_id as i64));
            }
        }

        sqlx::query!("
                INSERT INTO assets
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
                &*type_ids,
                &*item_ids,
                &location_ids,
                &reference_ids as _,
                &quantities,
                &location_flags,
            )
            .execute(pool)
            .await
            .map_err(IndustryError::SaveCharacterAssets)
            .map(drop)
    }

    /// Gets all asset names of the character.
    /// 
    /// # Params
    /// 
    /// * `pool`   > Connection to postgres
    /// * `client` > Authenticated eve client
    /// * `cid`    > [CharacterId] of the character
    /// 
    /// # Errors
    /// 
    /// - If the database is not avaialble
    /// - If the EVE API request fails
    /// 
    pub async fn fetch_asset_names(
        pool:   &PgPool,
        client: &EveAuthClient,
        cid:    CharacterId,
    ) -> Result<(), IndustryError> {
        let ids = sqlx::query!("
                    SELECT item_id
                    FROM assets
                    WHERE (
                        location_flag = 'Hangar' OR
                        location_flag = 'Deliveries'
                    )
                    AND character_id = $1
                    GROUP BY item_id
            ",
                *cid
            )
            .fetch_all(pool)
            .await
            .map_err(IndustryError::FetchCharacterItemIds)?
            .into_iter()
            .map(|x| x.item_id.into())
            .collect::<Vec<ItemId>>();

        let character_service = EveCharacterService::new(cid);
        let items = character_service.asset_names(
                client,
                ids,
            )
            .await
            .map_err(IndustryError::FetchCharacterAssetName)?;

        let mut item_ids  = Vec::new();
        let mut names     = Vec::new();

        for item in items {
            item_ids.push(*item.item_id);
            names.push(item.name);
        }

        sqlx::query!("
                    INSERT INTO asset_names
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
            .execute(pool)
            .await
            .map_err(IndustryError::InsertCharacterAssetNames)?;

        Ok(())
    }

    /// Gets all locations that the character has assets in.
    /// 
    /// # Params
    /// 
    /// * `pool`   > Connection to postgres
    /// * `client` > Authenticated eve client
    /// * `cid`    > [CharacterId] of the character
    /// 
    /// # Errors
    /// 
    /// - If the database is not avaialble
    /// - If the EVE API request fails
    /// 
    pub async fn resolve_locations(
        pool:   &PgPool,
        client: &EveAuthClient,
        cid:    CharacterId,
    ) -> Result<(), IndustryError> {
        let ids = sqlx::query!("
                    SELECT location_id
                    FROM assets
                    WHERE (
                        location_flag = 'Hangar' OR
                        location_flag = 'Deliveries'
                    )
                    -- Only filter on citadels
                    AND location_id > 1000000000000
                    AND character_id = $1
                    GROUP BY location_id
            ",
                *cid
            )
            .fetch_all(pool)
            .await
            .map_err(IndustryError::FetchCharacterLocationIds)?
            .into_iter()
            .map(|x| x.location_id.into())
            .collect::<Vec<LocationId>>();

        let universe_service = EveUniverseService::default();
        let mut structures = FuturesUnordered::new();
        for id in ids {
            structures.push(universe_service.structure(client, id));
        }

        let mut location_ids  = Vec::new();
        let mut system_ids    = Vec::new();
        let mut names         = Vec::new();
        while let Some(Ok((location_id, structure))) = structures.next().await {
            location_ids.push(*location_id);
            system_ids.push(*structure.solar_system_id);
            names.push(structure.name);
        }

        sqlx::query!("
                    INSERT INTO asset_locations
                    (
                        character_id,
                        location_id,
                        system_id,
                        name
                    )
                    SELECT $1, * FROM UNNEST(
                        $2::BIGINT[],
                        $3::BIGINT[],
                        $4::VARCHAR[]
                    )
                ",
                *cid,
                &location_ids,
                &system_ids,
                &names,
            )
            .execute(pool)
            .await
            .map_err(IndustryError::InsertCharacterAssetLocations)?;

        Ok(())
    }
}
