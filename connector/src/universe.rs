use serde::Deserialize;

use crate::{SystemId, LocationId};
use crate::{ConnectError, EveAuthClient, RequestClient};

/// Wrapper for character
#[derive(Debug, Default)]
pub struct EveUniverseService;

impl EveUniverseService {
    /// Fetches information about the given location id.
    /// The location id must be larget than 1_000_000_000_000.
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveAuthClient] is not valid
    /// - If the character does not have access to the structure
    /// - If the structure does not exist
    /// - If the [LocationId] is not a valid id
    /// 
    /// # Returns
    /// 
    /// Information about the structure
    /// 
    pub async fn structure(
        &self,
        client: &EveAuthClient,
        lid:    LocationId,
    ) -> Result<(LocationId, Structure), ConnectError> {
        let path = format!("latest/universe/structures/{}", lid);
        client
            .fetch::<Structure>(&path)
            .await
            .map_err(Into::into)
            .map(|x| (lid, x))
    }

    /// Gets the name of the given system.
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [SystemId] is not a valid id
    /// 
    /// # Returns
    /// 
    /// Information about the structure
    /// 
    pub async fn system_name(
        &self,
        client: &impl RequestClient,
        sid:    &SystemId,
    ) -> Result<System, ConnectError> {
        let path = format!("latest/universe/systems/{}", sid);
        client
            .fetch::<System>(&path)
            .await
            .map_err(Into::into)
    }
}

/// Represents a strucutre
#[derive(Clone, Debug, Deserialize)]
pub struct Structure {
    /// Name of the structure
    pub name:            String,
    /// Id of the system the structure is located in
    pub solar_system_id: SystemId
}

/// Represents a System
#[derive(Clone, Debug, Deserialize)]
pub struct System {
    /// Name of the system
    pub name:            String,
    /// Securtiy status of the system
    pub security_status: f32,
    /// Id of the system
    pub system_id:       SystemId
}
