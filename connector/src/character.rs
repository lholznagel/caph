use crate::{ConnectError, EveAuthClient, RequestClient};
use crate::{AllianceId, CharacterId, CorporationId, LocationId, ItemId, TypeId};
use serde::{Deserialize, Serialize};

/// Wrapper for character
pub struct ConnectCharacterService {
    /// Client for communicating with the EVE-API
    client: EveAuthClient,
    /// Character id this client belongs to
    cid:    CharacterId,
}

impl ConnectCharacterService {

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
        client: EveAuthClient,
        cid:    CharacterId,
    ) -> Self {
        ConnectCharacterService {
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
        let path = format!("characters/{}/assets", self.cid);
        self
            .client
            .fetch_page::<CharacterAssetEntry>(&path)
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
        let path = format!("characters/{}/blueprints", self.cid);
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
        let path = format!("characters/{}/", self.cid);
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

        let path = format!("alliances/{}", aid);
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

        let path = format!("corporations/{}", cid);
        self
            .client
            .fetch::<Corp>(&path)
            .await
            .map(|x| x.name)
            .map_err(Into::into)
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
    pub item_id:     ItemId,
    /// Id of the location the asset is located in
    pub location_id: LocationId,
    /// Number of assets
    pub quantity:    i32,
    /// Type id of the asset
    pub type_id:     TypeId,
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
