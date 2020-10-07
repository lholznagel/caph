use crate::error::*;

use eve_online_api::{
    Constellation, ConstellationId, EveClient, Region, RegionId, System, SystemId, Type, TypeId,
};

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Database {
    constellations: HashMap<ConstellationId, Constellation>,
    items: HashMap<TypeId, Type>,
    regions: HashMap<RegionId, Region>,
    systems: HashMap<SystemId, System>,
}

impl Database {
    /*pub async fn resolve_item_ids(&mut self, names: Vec<String>) -> Result<Vec<TypeId>> {
        let ids = EveClient::default()
            .resolve_name_to_id(names)
            .await?
            .inventory_types
            .unwrap_or_default()
            .into_iter()
            .map(|x| x.id)
            .collect::<Vec<TypeId>>();

        for id in &ids {
            self.items.insert(
                id.clone(),
                EveClient::default().fetch_type(id.clone()).await?.unwrap(),
            );
        }

        Ok(ids)
    }

    pub async fn resolve_region_ids(&mut self, names: Vec<String>) -> Result<Vec<RegionId>> {
        let ids = EveClient::default()
            .resolve_name_to_id(names)
            .await?
            .regions
            .unwrap_or_default()
            .into_iter()
            .map(|x| x.id)
            .collect::<Vec<RegionId>>();

        for id in &ids {
            self.regions.insert(
                id.clone(),
                EveClient::default()
                    .fetch_region(id.clone())
                    .await?
                    .unwrap(),
            );
        }

        Ok(ids)
    }*/

    pub async fn fetch_constellation(&mut self, id: &ConstellationId) -> Result<Constellation> {
        if !self.constellations.contains_key(&id) {
            let constellation = EveClient::default()
                .fetch_constellation(id.clone())
                .await?
                .unwrap();
            self.constellations
                .insert(id.clone(), constellation.clone());
            Ok(constellation)
        } else {
            Ok(self.constellations.get(&id).unwrap().clone())
        }
    }

    pub async fn fetch_item(&mut self, id: &TypeId) -> Result<Type> {
        if !self.items.contains_key(&id) {
            let item = EveClient::default().fetch_type(id.clone()).await?.unwrap();
            self.items.insert(id.clone(), item.clone());
            Ok(item)
        } else {
            Ok(self.items.get(&id).unwrap().clone())
        }
    }

    pub async fn fetch_region(&mut self, id: &RegionId) -> Result<Region> {
        if !self.regions.contains_key(&id) {
            let item = EveClient::default()
                .fetch_region(id.clone())
                .await?
                .unwrap();
            self.regions.insert(id.clone(), item.clone());
            Ok(item)
        } else {
            Ok(self.regions.get(&id).unwrap().clone())
        }
    }

    pub async fn fetch_system(&mut self, id: &SystemId) -> Result<System> {
        if !self.systems.contains_key(&id) {
            let system = EveClient::default()
                .fetch_system(id.clone())
                .await?
                .unwrap();
            self.systems.insert(id.clone(), system.clone());
            Ok(system)
        } else {
            Ok(self.systems.get(&id).unwrap().clone())
        }
    }

    pub async fn fetch_system_region(&mut self, id: &SystemId) -> Result<RegionId> {
        let system = self.fetch_system(&id).await?;
        let constellation = self.fetch_constellation(&system.constellation_id).await?;
        Ok(constellation.region_id)
    }
}
