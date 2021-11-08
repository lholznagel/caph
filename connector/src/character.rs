use crate::{ConnectError, EveAuthClient, RequestClient};
use crate::{AllianceId, CharacterId, CorporationId, LocationId, ItemId, TypeId, StationId, JobId};
use serde::{Deserialize, Serialize};

/// Wrapper for character
pub struct ConnectCharacterService<'a> {
    /// Client for communicating with the EVE-API
    client: &'a EveAuthClient,
    /// Character id this client belongs to
    cid:    CharacterId,
}

impl<'a> ConnectCharacterService<'a> {

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
    pub fn new(
        client: &'a EveAuthClient,
        cid:    CharacterId,
    ) -> Self {
        Self {
            client,
            cid,
        }
    }

    /// Gets a list of all player owned assets
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of all assets
    ///
    pub async fn assets(
        &self,
    ) -> Result<Vec<CharacterAssetEntry>, ConnectError> {
        let path = format!("latest/characters/{}/assets", self.cid);
        self
            .client
            .fetch_page::<CharacterAssetEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Fetches all item names of an character
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of all assets
    ///
    pub async fn asset_name(
        &self,
        iids: Vec<ItemId>
    ) -> Result<Vec<CharacterAssetName>, ConnectError> {
        let path = format!("latest/characters/{}/assets/names", self.cid);
        self
            .client
            .post::<_, Vec<CharacterAssetName>>(iids, &path)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of all player owned bluepritns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of all blueprints
    ///
    pub async fn blueprints(
        &self,
    ) -> Result<Vec<CharacterBlueprintEntry>, ConnectError> {
        let path = format!("latest/characters/{}/blueprints", self.cid);
        self
            .client
            .fetch_page::<CharacterBlueprintEntry>(&path)
            .await
            .map_err(Into::into)
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
    pub async fn info(
        &self,
    ) -> Result<CharacterInfo, ConnectError> {
        let path = format!("latest/characters/{}/", self.cid);
        self
            .client
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
        aid: AllianceId,
    ) -> Result<String, ConnectError> {
        /// Temporary struct for deserializing
        #[derive(Deserialize)]
        struct Alliance {
            /// Name of the alliance
            name: String
        }

        let path = format!("latest/alliances/{}", aid);
        self
            .client
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
    pub async fn corporation_name(
        &self,
        cid: CorporationId,
    ) -> Result<String, ConnectError> {
        /// Temporary struct for deserializing
        #[derive(Deserialize)]
        struct Corp {
            /// Name of the corporation
            name: String
        }

        let path = format!("latest/corporations/{}", cid);
        self
            .client
            .fetch::<Corp>(&path)
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    /// Gets a list of all jobs that an character started
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of all industry jobs including those made with corporation
    /// blueprints.
    ///
    pub async fn industry_jobs(
        &self,
        cid: CorporationId
    ) -> Result<Vec<IndustryJob>, ConnectError> {
        let path = format!("latest/characters/{}/industry/jobs", self.cid);
        let mut character = self
            .client
            .fetch::<Vec<IndustryJob>>(&path)
            .await
            .map_err(Into::into)?;

        let path = format!("latest/corporations/{}/industry/jobs", cid);
        let corporation = self
            .client
            .fetch::<Vec<IndustryJob>>(&path)
            .await;
        // The character may not have the permission
        let corporation = if let Ok(x) = corporation {
            x
                .into_iter()
                .filter(|x| x.character_id == self.cid)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        character.extend(corporation);
        Ok(character)
    }
}

/// General information about the character
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterInfo {
    /// Optional alliance id the character is in
    pub alliance_id:    Option<AllianceId>,
    /// Corporation id of the character
    pub corporation_id: CorporationId,
    /// Name of the character
    pub name:           String,
}

/// Represents a single character asset
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterAssetEntry {
    /// Unique ID of the asset
    pub item_id:       ItemId,
    /// Id of the location the asset is located in
    pub location_id:   LocationId,
    /// Location of the item
    pub location_flag: String,
    /// Number of assets
    pub quantity:      i32,
    /// Type id of the asset
    pub type_id:       TypeId,
}

/// Name of an asset that belongs to an character
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterAssetName {
    /// Id of the asset
    pub item_id: ItemId,
    /// Name of the asset
    pub name:    String,
}

/// Represents a single character blueprint
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CharacterBlueprintEntry {
    /// Unique ID of the asset
    pub item_id:             ItemId,
    /// Id of the location the asset is located in
    pub location_id:         LocationId,
    /// Material efficiency of the blueprint, max 10
    pub material_efficiency: i32,
    /// Time efficiency of the blueprint, max 20
    pub time_efficiency:     i32,
    /// A range of numbers with a minimum of -2 and no maximum value where -1
    /// is an original and -2 is a copy. It can be a positive integer if it is
    /// a stack of blueprint originals fresh from the market (e.g. no 
    /// activities performed on them yet).
    pub quantity:            i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original
    pub runs:                i32,
    /// Type id of the asset
    pub type_id:             TypeId,
}

/// Represents a single industry job
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndustryJob {
    /// Activity of the job
    #[serde(rename = "activity_id")]
    pub activity:    i32,
    /// TypeId of the blueprint that is used
    #[serde(rename = "blueprint_type_id")]
    pub type_id:      TypeId,
    /// Date the character started the job
    #[serde(rename = "start_date")]
    pub start_date:   String,
    /// Date when the job is done
    #[serde(rename = "end_date")]
    pub end_date:     String,
    /// Character id of the character that installed the job
    #[serde(rename = "installer_id")]
    pub character_id: CharacterId,
    /// Id of the station the job was started
    #[serde(alias = "location_id")]
    #[serde(rename = "station_id")]
    pub station_id:   StationId,
    /// Unique id of the job
    #[serde(rename = "job_id")]
    pub job_id:       JobId,
}
