use crate::error::CollectorError;

use cachem::ConnectionPool;
use caph_db::*;
use caph_eve_data_wrapper::{EveDataWrapper, SolarsystemEntry};
use std::collections::HashMap;

pub struct Sde {
    eve:  EveDataWrapper,
    pool: ConnectionPool,
}

impl Sde {
    pub fn new(eve: EveDataWrapper, pool: ConnectionPool) -> Self {
        Self { eve, pool }
    }

    pub async fn run(&mut self) -> Result<(), CollectorError> {
        self.save_blueprints(&self.eve).await?;
        self.save_schematics(&self.eve).await?;
        self.save_reprocessing_info(&self.eve).await?;
        self.save_items(&self.eve).await?;
        self.save_item_dogma(&self.eve).await?;
        self.save_names(&self.eve).await?;
        self.save_system_region(&self.eve).await?;

        Ok(())
    }

    async fn save_names(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let mut con = self.pool.acquire().await?;

        let stations = sde.stations().await?;
        let types = sde.types().await?;
        let unique_names = sde.names().await?;

        let mut stations = stations.collect_names();
        let types = types.collect_names();
        let unique_names = unique_names.collect_names();

        stations.extend(types);
        stations.extend(unique_names);
        con.mset(CacheName::Name, stations).await.unwrap();

        Ok(())
    }

    /// Extractes all items and inserts them into the database.
    async fn save_items(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let item_service  = sde.types().await?;
        let group_service = sde.groups().await?;

        let mut con = self.pool.acquire().await?;

        // Collect all items together
        let mut entries = HashMap::new();
        for (tid, entry) in item_service.types() {
            let category = group_service.groups()
                .get(&entry.group_id)
                .map(|x| x.category_id)
                .unwrap();
            let name = entry.name().unwrap_or_default();
            let description = entry.description().unwrap_or_default();
            let volume = entry.volume.unwrap_or(0f32);
            entries.insert(
                *tid,
                ItemEntry::new(
                    category,
                    entry.group_id,
                    *tid,
                    volume,
                    name,
                    description,
                )
            );
        }
        con.mset(CacheName::Item, entries).await.unwrap();

        Ok(())
    }

    /// Collects all dogma attributes and effects for all items and stores them.
    async fn save_item_dogma(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let dogma_service = sde.dogma().await?;

        let mut con = self.pool.acquire().await?;
        let mut entries = HashMap::new();

        for (tid, entry) in dogma_service.get_type_dogma() {
            let attributes = entry
                .attributes
                .iter()
                .cloned()
                .map(|x| DogmaAttribute {
                    attr_id: (*x.attribute_id).into(),
                    value:   x.value
                })
                .collect::<Vec<_>>();
            let effects = entry
                .effects
                .iter()
                .cloned()
                .map(|x| DogmaEffect {
                    eff_id:  (*x.effect_id).into(),
                    default: x.is_default
                })
                .collect::<Vec<_>>();

            let entry = ItemDogmaEntry {
                attributes,
                effects
            };
            entries.insert(tid, entry);
        }
        con.mset(CacheName::ItemDogma, entries).await.unwrap();

        Ok(())
    }

    /// Collect all item materials together and save them in the database.
    async fn save_reprocessing_info(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let type_service = sde.types().await?;

        let mut con = self.pool.acquire().await?;

        // Collect all items together
        let mut entries = HashMap::new();
        for (tid, materials) in type_service.materials() {
            let mut material_entries = Vec::new();
            for material in materials.materials.iter() {
                let material_id = material.material_type_id;
                let quantity = material.quantity;

                material_entries.push(ReprocessEntry::new(material_id, quantity));
            }

            entries.insert(*tid, material_entries);
        }
        con.mset(CacheName::Reprocess, entries).await.unwrap();

        Ok(())
    }

    /// Collects all stations an stores a subset of it in the database
    async fn save_system_region(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let system_service = sde.systems().await?;

        let mut con = self.pool.acquire().await?;

        // Collect all entries
        let mut entries = HashMap::new();
        for (cid, entry) in system_service.constellations() {
            let region = system_service.regions()
                .iter()
                .find(|(_, rentry)| rentry.constellations.contains(&cid))
                .map(|(rid, _)| rid)
                .unwrap();

            for system in entry.systems.iter() {
                let security = system_service.eve_systems()
                    .iter()
                    .find(|x: &&SolarsystemEntry| x.solar_system_id == *system)
                    .map(|x| x.security);

                if let Some(x) = security {
                    entries.insert(
                        *system,
                        SystemRegionEntry {
                            region_id: *region,
                            system_id: *system,
                            security:  x,
                        }
                    );
                }
            }
        }
        con.mset(CacheName::SystemRegion, entries).await.unwrap();

        Ok(())
    }

    async fn save_blueprints(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let blueprint_service = sde.blueprints().await?;

        let mut con = self.pool.acquire().await?;

        let entries = blueprint_service
            .blueprints()
            .iter()
            .map(|(bid, entry)| (*bid, BlueprintEntry::from(entry)))
            .collect::<HashMap<_, _>>();
        con.mset(CacheName::Blueprint, entries).await.unwrap();

        Ok(())
    }

    async fn save_schematics(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let schematic_service = sde.planet_schematics().await?;

        let mut con = self.pool.acquire().await?;

        let entries = schematic_service
            .schematics()
            .iter()
            .map(|(bid, entry)| (*bid, SchematicEntry::from(entry)))
            .collect::<HashMap<_, _>>();
        con.mset(CacheName::Schematic, entries).await.unwrap();

        Ok(())
    }
}
