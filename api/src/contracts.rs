use crate::eve_client::*;
use crate::fetch;
use crate::universe::*;

use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ContractId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contract {
    pub contract_id: ContractId,
    pub date_expired: String,
    pub date_issued: String,
    pub issuer_corporation_id: CorporationId,
    pub issuer_id: u32,
    #[serde(rename = "type")]
    pub type_: String,

    pub buyout: Option<f32>,
    pub collateral: Option<f32>,
    pub days_to_complete: Option<u32>,
    pub end_location_id: Option<u32>,
    pub for_corporation: Option<bool>,
    pub price: Option<f32>,
    pub reward: Option<f32>,
    pub start_location_id: Option<u32>,
    pub title: Option<String>,
    pub volume: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractBid {
    pub amount: f32,
    pub bid_id: u32,
    pub date_bid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContractItem {
    pub is_included: bool,
    pub quantity: u32,
    pub record_id: u32,
    pub type_id: TypeId,

    pub is_blueprint_copy: Option<bool>,
    pub item_id: Option<u32>,
    pub material_efficiency: Option<u32>,
    pub runs: Option<u32>,
    pub time_efficiency: Option<u32>,
}

impl EveClient {
    fetch!(
        fetch_public_contracts,
        "contracts/public",
        RegionId,
        Vec<Contract>
    );

    fetch!(
        fetch_public_contract_bids,
        "contracts/public/bids",
        ContractId,
        Vec<ContractBid>
    );
    fetch!(
        fetch_public_contract_items,
        "contracts/public/items",
        ContractId,
        Vec<ContractItem>
    );
}
