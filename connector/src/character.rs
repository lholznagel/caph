use serde::{Deserialize, Serialize};

use crate::{AllianceId, CharacterId, CorporationId};
use crate::{AssetEntry, BlueprintEntry, ConnectError, EveAuthClient, EveClient, IndustryJobEntry, ItemId, RequestClient};

/// Wrapper for character
#[derive(Debug)]
pub struct EveCharacterService {
    /// Character id this client belongs to
    cid: CharacterId,
}

impl EveCharacterService {
    /// Creates a new instance of the service
    ///
    /// # Params
    ///
    /// * `cid`    -> Character id the client belongs to
    /// * `client` -> Eve auth client for communicating with the EVE-API
    ///
    /// # Returns
    ///
    /// New instance
    ///
    pub fn new(cid: CharacterId) -> Self {
        Self { cid }
    }

    /// Gets general information about the character
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// Character information
    ///
    pub async fn info(&self, client: &EveClient) -> Result<CharacterInfo, ConnectError> {
        let path = format!("latest/characters/{}/", self.cid);
        client
            .fetch::<CharacterInfo>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets the name of an alliance by its id
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// Alliance name
    ///
    pub async fn alliance_name(
        &self,
        client: &EveClient,
        aid: AllianceId,
    ) -> Result<String, ConnectError> {
        /// Temporary struct for deserializing
        #[derive(Deserialize)]
        struct Alliance {
            /// Name of the alliance
            name: String,
        }

        let path = format!("latest/alliances/{}", aid);
        client
            .fetch::<Alliance>(&path)
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    /// Gets the name of an corporation by its id
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// Corporation name
    ///
    #[deprecated(note = "Use CorporationService::name")]
    pub async fn corporation_name(
        &self,
        client: &EveClient,
        cid: CorporationId,
    ) -> Result<String, ConnectError> {
        /// Temporary struct for deserializing
        #[derive(Deserialize)]
        struct Corp {
            /// Name of the corporation
            name: String,
        }

        let path = format!("latest/corporations/{}", cid);
        client
            .fetch::<Corp>(&path)
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    /// Gets all assets the character owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of assets
    ///
    pub async fn assets(
        &self,
        client: &EveAuthClient
    ) -> Result<Vec<AssetEntry>, ConnectError> {
        let path = format!("latest/characters/{}/assets", self.cid);
        client
            .fetch_page::<AssetEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets all asset names of the assets the character owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of assets
    ///
    pub async fn asset_names(
        &self,
        client: &EveAuthClient,
        iids:   Vec<ItemId>,
    ) -> Result<Vec<AssetName>, ConnectError> {
        let path = format!("latest/characters/{}/assets/names", self.cid);
        client
            .post::<Vec<ItemId>, Vec<AssetName>>(iids, &path)
            .await
            .map_err(Into::into)
    }

    /// Gets all blueprints the character owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of Blueprints
    ///
    pub async fn blueprints(
        &self,
        client: &EveAuthClient,
    ) -> Result<Vec<BlueprintEntry>, ConnectError> {
        let path = format!("latest/characters/{}/blueprints", self.cid);
        client
            .fetch_page::<BlueprintEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets all industry jobs the character has running
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of all industry jobs
    ///
    pub async fn industry_jobs(
        &self,
        client: &EveAuthClient,
    ) -> Result<Vec<IndustryJobEntry>, ConnectError> {
        let path = format!("latest/characters/{}/industry/jobs", self.cid);
        client
            .fetch_page::<IndustryJobEntry>(&path)
            .await
            .map_err(Into::into)
    }
}

/// General information about the character
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterInfo {
    /// Optional alliance id the character is in
    pub alliance_id: Option<AllianceId>,
    /// Corporation id of the character
    pub corporation_id: CorporationId,
    /// Name of the character
    pub name: String,
}

/// Information about a location by [LocationId]
#[derive(Debug, Deserialize)]
pub struct AssetName {
    /// Id of the location id that maps to the name
    pub item_id: ItemId,
    /// Name of the location, for example a container or station
    pub name:    String,
}
