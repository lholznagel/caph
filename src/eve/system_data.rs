use crate::error::*;
use crate::eve::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Serialize)]
pub struct SystemId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemData {
    pub constellation_id: u32,
    pub name: String,
    pub position: Position,
    pub security_status: f32,
    pub system_id: SystemId,

    pub planets: Option<Vec<Planet>>,
    pub security_class: Option<String>,
    pub star_id: Option<u32>,
    pub startgates: Option<u32>,
    pub stations: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Planet {
    pub asteroid_belts: Option<Vec<u32>>,
    pub moons: Option<Vec<u32>>,
    pub planet_id: u32,
}

impl Eve {
    pub async fn fetch_systems(&self, system_ids: Vec<SystemId>) -> Result<Vec<SystemData>> {
        let mut result = Vec::with_capacity(system_ids.len());

        let mut counter = 0;
        for type_id in &system_ids {
            let response = self
                .fetch(&format!(
                    "universe/systems/{}?datasource=tranquility",
                    type_id.0
                ))
                .await?;

            if response.status() == 404 {
                log::warn!("SystemId {} does not exist. Skipping.", type_id.0);
                continue;
            }

            counter += 1;
            log::debug!(
                "Downloaded SystemId {}. Remaining: {}",
                type_id.0,
                system_ids.len() - counter
            );
            let type_data = response.json().await?;
            result.push(type_data);
        }

        log::debug!("Downloaded all given SystemIds");
        Ok(result)
    }

    pub async fn fetch_system_ids(&self) -> Result<Vec<SystemId>> {
        self.fetch_ids("universe/systems/?datasource=tranquility&page=")
            .await
    }
}
