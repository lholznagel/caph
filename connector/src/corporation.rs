use crate::{
    AllianceId, AssetEntry, BlueprintEntry, ConnectError, CorporationId, EveAuthClient, EveClient,
    IndustryJobEntry, LocationId, RequestClient,
};
use serde::{Deserialize, Serialize};

/// Wrapper for corporations
pub struct EveCorporationService {
    /// Corporation id this client belongs to
    cid: CorporationId,
}

impl EveCorporationService {
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
    pub fn new(cid: CorporationId) -> Self {
        Self { cid }
    }

    /// Gets general information about the corporation
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// Corporation information
    ///
    pub async fn info(&self, client: &EveClient) -> Result<CorporationInfo, ConnectError> {
        let path = format!("latest/corporations/{}/", self.cid);
        client
            .fetch::<CorporationInfo>(&path)
            .await
            .map_err(Into::into)
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
    pub async fn assets(&self, client: &EveAuthClient) -> Result<Vec<AssetEntry>, ConnectError> {
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

    /// Gets all industry jobs the corporation has running
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of industry jobs from the corp
    ///
    pub async fn industry_jobs(
        &self,
        client: &EveAuthClient,
    ) -> Result<Vec<IndustryJobEntry>, ConnectError> {
        let path = format!("latest/corporations/{}/industry/jobs", self.cid);
        let mut data = client
            .fetch_page::<IndustryJobEntry>(&path)
            .await
            .map_err(Into::into)?;

        for x in data.iter_mut() {
            x.corporation_id = Some(self.cid);
        }
        Ok(data)
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
    pub async fn wallets(&self, client: &EveAuthClient) -> Result<Vec<WalletEntry>, ConnectError> {
        let path = format!("latest/corporations/{}/wallets", self.cid);
        client
            .fetch_page::<WalletEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of names for the given [LocationId].
    ///
    /// # Limits
    ///
    /// This is only for the current corporation as determined by the
    /// [EveAuthClient].
    ///
    /// # Params
    ///
    /// * `client` > Authenticated ESI client
    /// * `lid`    > List of [LocationId]s to resolve
    ///
    /// # Errors
    ///
    /// - If the endpoint is not available
    /// - If the response cannot be parsed
    ///
    /// # Returns
    ///
    /// List of [Location]s that match the given [LocationId].
    ///
    pub async fn location_name(
        &self,
        client: &EveAuthClient,
        lid: Vec<LocationId>,
    ) -> Result<Vec<ItemLocation>, ConnectError> {
        let path = format!("latest/corporations/{}/assets/names", self.cid);
        client
            .post::<Vec<LocationId>, Vec<ItemLocation>>(lid, &path)
            .await
            .map_err(Into::into)
    }
}

/// Represents a transaction entry
#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    /// ISK amount
    pub amount: f32,
    /// Date the transaction was performed
    pub date: String,
    /// Information about the transaction
    pub description: String,
    /// Unique ID
    pub id: u64,
}

/// Represents a wallet entry
#[derive(Debug, Deserialize)]
pub struct WalletEntry {
    /// Current balance of the division
    pub balance: f32,
    /// Devision number, eg: 1 is the master wallet
    pub division: u8,
}

/// General information about the character
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CorporationInfo {
    /// Optional alliance id the character is in
    pub alliance_id: Option<AllianceId>,
    /// Name of the character
    pub name: String,
}

/// Information about a location by [LocationId]
#[derive(Debug, Deserialize)]
pub struct ItemLocation {
    /// Id of the location id that maps to the name
    pub item_id: LocationId,
    /// Name of the location, for example a container or station
    pub name: String,
}
