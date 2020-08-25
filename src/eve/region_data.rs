use crate::error::*;
use crate::eve::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Serialize)]
pub struct RegionId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegionData {
    pub constellations: Vec<u32>,
    pub name: String,
    pub region_id: RegionId,
    pub description: Option<String>,
}

impl Eve {
    pub async fn fetch_regions(&self, region_ids: Vec<RegionId>) -> Result<Vec<RegionData>> {
        let mut result = Vec::with_capacity(region_ids.len());

        for type_id in region_ids {
            let response = self
                .fetch(&format!(
                    "universe/regions/{}?datasource=tranquility",
                    type_id.0
                ))
                .await?;

            if response.status() == 404 {
                log::warn!("RegionId {} does not exist. Skipping.", type_id.0);
                continue;
            }

            log::debug!("Downloaded RegionId {}", type_id.0);
            let type_data = response.json().await?;
            result.push(type_data);
        }

        log::debug!("Downloaded all given RegionIds");
        Ok(result)
    }

    pub async fn fetch_region_ids(&self) -> Result<Vec<TypeId>> {
        self.fetch_ids("universe/regions/?datasource=tranquility&page=")
            .await
    }
}
