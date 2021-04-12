use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(transparend)]
pub struct ContractId(pub u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(transparend)]
pub struct CharacterId(pub u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(transparend)]
pub struct CorporationId(pub u32);

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(transparend)]
pub struct LocationId(pub u32);

pub struct Contract {
    #[serde(rename = "contract_id")]
    pub contract_id:           ContractId,
    /// Expiration date of the contract
    #[serde(rename = "date_expired")]
    pub date_expired:          String,
    /// Сreation date of the contract
    #[serde(rename = "date_issued")]
    pub date_issued:           String,
    /// true if the contract was issued on behalf of the issuer’s corporation
    #[serde(rename = "for_corporation")]
    #[serde(default)]
    pub for_corporation:       bool,
    /// Character’s corporation ID for the issuer
    #[serde(rename = "issue_corporation_id")]
    pub issuer_corporation_id: CorporationId,
    /// Character ID for the issuer
    #[serde(rename = "issuer_id")]
    pub issuer_id:             CharacterId,
    /// Type of the contract
    /// unknown, item_exchange, auction, courier, loan
    #[serde(rename = "type")]
    pub typ:                   String,

    /// Buyout price (for Auctions only)
    #[serde(rename = "buyout")]
    pub buyout:                Option<f32>,
    /// Collateral price (for Couriers only)
    #[serde(rename = "collateral")]
    pub collateral:            Option<f32>,
    /// Number of days to perform the contract
    #[serde(rename = "days_to_complete")]
    pub days_to_complete:      Option<u32>,
    /// Start location ID (for Couriers contract)
    #[serde(rename = "start_location_id")]
    pub start_location_id:     Option<LocationId>,
    /// End location ID (for Couriers contract)
    #[serde(rename = "end_location_id")]
    pub end_location_id:       Option<LocationId>,
    /// Price of contract (for ItemsExchange and Auctions)
    #[serde(rename = "price")]
    pub price:                 Option<f32>,
    /// Remuneration for contract (for Couriers only)
    #[serde(rename = "reward")]
    pub reward:                Option<f32>,
    /// Title of the contract
    #[serde(rename = "title")]
    pub title:                 Option<String>,
    /// Volume of items in the contract
    #[serde(rename = "volume")]
    pub volume:                Option<f32>,
}
