use crate::{CorporationId, ConnectError, BlueprintEntry, EveAuthClient, RequestClient, TypeId, LocationId, ItemId};
use serde::Deserialize;

/// Wrapper for corporations
pub struct CorporationService {
    /// Corporation id this client belongs to
    cid: CorporationId,
}

impl CorporationService {

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
        cid: CorporationId,
    ) -> Self {
        Self {
            cid,
        }
    }

    /// Gets all assets the corporation owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of Blueprints
    ///
    pub async fn assets(
        &self,
        client: &EveAuthClient,
    ) -> Result<Vec<AssetEntry>, ConnectError> {
        let path = format!("latest/corporations/{}/assets", self.cid);
        client
            .fetch_page::<AssetEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets all blueprints the corporation owns
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
        let path = format!("latest/corporations/{}/blueprints", self.cid);
        client
            .fetch_page::<BlueprintEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of last transactions of the master wallet.
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of transactions
    ///
    pub async fn wallet_journal(
        &self,
        client: &EveAuthClient,
    ) -> Result<Vec<JournalEntry>, ConnectError> {
        let path = format!("latest/corporations/{}/wallets/1/journal", self.cid);
        client
            .fetch_page::<JournalEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of all wallets and their current balance
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of balance
    ///
    pub async fn wallets(
        &self,
        client: &EveAuthClient,
    ) -> Result<Vec<WalletEntry>, ConnectError> {
        let path = format!("latest/corporations/{}/wallets", self.cid);
        client
            .fetch_page::<WalletEntry>(&path)
            .await
            .map_err(Into::into)
    }
}

/// Represents an asset
#[derive(Debug, Deserialize)]
pub struct AssetEntry {
    /// Unique Id of the item
    pub item_id:           ItemId,
    /// Flag of the location, eg. MedSlot6, Deliveries, Wallet
    pub location_flag:     String,
    /// Either a id of a structurte, container or ship
    pub location_id:       LocationId,
    /// Stored quantity
    pub quantity:          u32,
    /// [TypeId] of the item
    pub type_id:           TypeId,

    /// True if the item is a copy
    #[serde(default)]
    pub is_blueprint_copy: bool,
}

/// Represents a transaction entry
#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    ///
    pub amount: f32,
    ///
    pub date: String,
    ///
    pub description: String,
    ///
    pub id: u64,
    ///
    pub ref_type: String,
}

/// Represents a wallet entry
#[derive(Debug, Deserialize)]
pub struct WalletEntry {
    /// Current balance of the division
    pub balance:  f32,
    /// Devision number, eg: 1 is the master wallet
    pub division: u8,
}
