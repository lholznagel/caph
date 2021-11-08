use crate::{ConnectError, EveClient, RequestClient};
use crate::{TypeId, SystemId};
use serde::{Deserialize, Serialize};

/// Wrapper for character
#[deprecated = "User caph_core::MarketService"]
pub struct ConnectMarketService {
    /// Client for communicating with the EVE-API
    client: EveClient,
}

impl ConnectMarketService {

    /// Creates a new instance of the service
    ///
    /// # Params
    ///
    /// * `client` -> Eve client for communicating with the EVE-API
    ///
    /// # Returns
    ///
    /// New instance
    ///
    pub fn new(
        client: EveClient,
    ) -> Self {
        Self {
            client,
        }
    }

    /// Gets a list of all market prices for items
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of all prices
    ///
    pub async fn market_prices(
        &self,
    ) -> Result<Vec<MarketPrice>, ConnectError> {
        let path = format!("latest/markets/prices");
        self
            .client
            .fetch::<Vec<MarketPrice>>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of all indestry indexes
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of industry indexes
    ///
    pub async fn industry_systems(
        &self,
    ) -> Result<Vec<IndustrySystem>, ConnectError> {
        let path = format!("latest/industry/systems");
        self
            .client
            .fetch::<Vec<IndustrySystem>>(&path)
            .await
            .map_err(Into::into)
    }
}

/// Information about a market price for a single item
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketPrice {
    /// Adjusted price of the item
    #[serde(default)]
    pub adjusted_price: f64,
    /// Average price of the item
    #[serde(default)]
    pub average_price:  f64,
    /// TypeID of the item
    pub type_id:        TypeId,
}

/// Industry index for a specific system
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndustrySystem {
    /// Costs broken down to activities
    pub cost_indices:    Vec<CostIndex>,
    /// Id of the solar system the cost index refers to
    pub solar_system_id: SystemId
}

/// Cost for a specific activity
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CostIndex {
    /// Activity name
    pub activity:   String,
    /// Cost index
    pub cost_index: f64,
}
