use crate::conversion_model;
use crate::error::*;
use crate::eve_client::*;

use reqwest::Client;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct IdToName {
    pub category: String,
    pub id: u32,
    pub name: String,
}

conversion_model!(ConstellationNameToId, crate::eve_client::ConstellationId);
conversion_model!(CorporationNameToId, crate::universe::CorporationId);
conversion_model!(FactionNameToId, crate::universe::FactionId);
conversion_model!(InventoryTypeNameToId, crate::eve_client::TypeId);
conversion_model!(RegionNameToId, crate::eve_client::RegionId);
conversion_model!(StationNameToId, crate::universe::StationId);
conversion_model!(SystemNameToId, crate::eve_client::SystemId);

#[derive(Clone, Debug, Deserialize)]
pub struct NameToId {
    pub constellations: Option<Vec<ConstellationNameToId>>,
    pub corporations: Option<Vec<CorporationNameToId>>,
    pub factions: Option<Vec<FactionNameToId>>,
    pub inventory_types: Option<Vec<InventoryTypeNameToId>>,
    pub regions: Option<Vec<RegionNameToId>>,
    pub stations: Option<Vec<StationNameToId>>,
    pub systems: Option<Vec<SystemNameToId>>,
}

impl EveClient {
    pub async fn resolve_id_to_name(&self, ids: Vec<Box<dyn Id>>) -> Result<Vec<IdToName>> {
        let mut ids = ids.into_iter().map(|x| x.id()).collect::<Vec<u32>>();
        ids.sort();
        ids.dedup();

        let url = format!("{}/universe/names", EveClient::BASE_ADDR);
        Client::new()
            .post(&url)
            .json(&ids)
            .send()
            .await?
            .json()
            .await
            .map_err(EveApiError::ReqwestError)
    }

    pub async fn resolve_name_to_id(&self, names: Vec<String>) -> Result<NameToId> {
        let url = format!("{}/universe/ids", EveClient::BASE_ADDR);
        Client::new()
            .post(&url)
            .json(&names)
            .send()
            .await?
            .json()
            .await
            .map_err(EveApiError::ReqwestError)
    }
}
