use crate::{ConnectError, EveAuthClient, RequestClient, EveClient};
use crate::{AllianceId, CharacterId, CorporationId, LocationId, ItemId, TypeId};
use serde::{Deserialize, Serialize};

/// Wrapper for character
pub struct ConnectCharacterService {
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
        cid: CharacterId,
    ) -> Self {
        Self {
            cid,
        }
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
        client: &EveClient
    ) -> Result<CharacterInfo, ConnectError> {
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
        aid:    AllianceId,
    ) -> Result<String, ConnectError> {
        /// Temporary struct for deserializing
        #[derive(Deserialize)]
        struct Alliance {
            /// Name of the alliance
            name: String
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
    pub async fn corporation_name(
        &self,
        client: &EveClient,
        cid:    CorporationId,
    ) -> Result<String, ConnectError> {
        /// Temporary struct for deserializing
        #[derive(Deserialize)]
        struct Corp {
            /// Name of the corporation
            name: String
        }

        let path = format!("latest/corporations/{}", cid);
        client
            .fetch::<Corp>(&path)
            .await
            .map(|x| x.name)
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
    pub async fn corporation_blueprints(
        &self,
        client: &EveAuthClient,
        cid:    CorporationId
    ) -> Result<Vec<BlueprintEntry>, ConnectError> {
        let path = format!("latest/corporations/{}/blueprints", cid);
        client
            .fetch_page::<BlueprintEntry>(&path)
            .await
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

/// Represents a single character blueprint
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlueprintEntry {
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
