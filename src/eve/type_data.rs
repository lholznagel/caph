use crate::error::*;
use crate::eve::*;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Serialize)]
pub struct AttributeId(pub u32);

#[derive(Copy, Clone, Debug, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct TypeId(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypeData {
    pub description: String,
    pub group_id: GroupId,
    pub name: String,
    pub published: bool,
    pub type_id: TypeId,
    pub capacity: Option<f32>,
    pub dogma_attributes: Option<Vec<DogmaAttributes>>,
    pub dogma_effects: Option<Vec<DogmaEffects>>,
    pub graphic_id: Option<u32>,
    pub icon_id: Option<u32>,
    pub market_group_id: Option<u32>,
    pub mass: Option<f32>,
    pub package_volume: Option<f32>,
    pub portion_size: Option<u32>,
    pub radius: Option<f32>,
    pub volume: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DogmaAttributes {
    pub attribute_id: AttributeId,
    pub value: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DogmaEffects {
    pub effect_id: u32,
    pub is_default: bool,
}

impl TypeData {
    pub fn find_dogma(&self, attribute_id: AttributeId) -> Option<f32> {
        self.dogma_attributes
            .clone()
            .unwrap_or_default()
            .into_iter()
            .find(|x| x.attribute_id == attribute_id)
            .map(|x| x.value)
    }
}

impl Eve {
    pub async fn fetch_types(&self, type_ids: Vec<TypeId>) -> Result<Vec<TypeData>> {
        let mut result = Vec::with_capacity(type_ids.len());

        let mut counter = 0;
        for type_id in &type_ids {
            let response = self
                .fetch(&format!(
                    "universe/types/{}?datasource=tranquility",
                    type_id.0
                ))
                .await?;

            if response.status() == 404 {
                log::warn!("TypeId {} does not exist. Skipping.", type_id.0);
                continue;
            }

            counter += 1;
            log::debug!(
                "Downloaded TypeId {}. Remaining: {}",
                type_id.0,
                type_ids.len() - counter
            );
            let type_data = response.json().await?;
            result.push(type_data);
        }

        log::debug!("Downloaded all given TypeIds");
        Ok(result)
    }

    pub async fn fetch_type_ids(&self) -> Result<Vec<TypeId>> {
        self.fetch_ids("universe/types/?datasource=tranquility&page=")
            .await
    }
}
