use crate::error::CollectorError;

use cachem::{ConnectionPool, EmptyMsg, Protocol};
use caph_db::*;
use caph_eve_data_wrapper::{BlueprintAdditional, EveDataWrapper, SolarsystemEntry};
use metrix_exporter::MetrixSender;
use std::time::Instant;

pub struct Sde {
    eve:    EveDataWrapper,
    metrix: MetrixSender,
    pool:   ConnectionPool,
}

impl Sde {
    const METRIC_SDE:                  &'static str = "sde::time::complete";
    const METRIC_FETCH_SDE:            &'static str = "sde::time::fetch";
    const METRIC_PROCESSING_ITEM:      &'static str = "sde::time::processing::item";
    const METRIC_PROCESSING_MATERIAL:  &'static str = "sde::time::processing::material";
    const METRIC_PROCESSING_STATION:   &'static str = "sde::time::processing::station";
    const METRIC_PROCESSING_BLUEPRINT: &'static str = "sde::time::processing::blueprint";
    const METRIC_ITEM_VOLUME_COUNT:    &'static str = "sde::count::item_volume";
    const METRIC_MATERIAL_COUNT:       &'static str = "sde::count::material";
    const METRIC_SYSTEM_REGION_COUNT:  &'static str = "sde::count::system_region";
    const METRIC_BLUEPRINT_COUNT:      &'static str = "sde::count::blueprint";

    pub fn new(eve: EveDataWrapper, metrix: MetrixSender, pool: ConnectionPool) -> Self {
        Self { eve, metrix, pool }
    }

    pub async fn run(&mut self) -> Result<(), CollectorError> {
        let start = Instant::now();
        let timer = Instant::now();

        // Download the current zip
        self.metrix.send_time(Self::METRIC_FETCH_SDE, timer).await;

        self.save_blueprints(&self.eve).await?;
        self.save_item_materials(&self.eve).await?;
        self.save_items(&self.eve).await?;
        self.save_names(&self.eve).await?;
        self.save_system_region(&self.eve).await?;

        self.metrix.send_time(Self::METRIC_SDE, start).await;

        Ok(())
    }

    async fn save_names(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let mut conn = self.pool.acquire().await?;

        let stations = sde.stations().await?;
        let types = sde.types().await?;
        let unique_names = sde.names().await?;

        let mut stations = stations.collect_names();
        let types = types.collect_names();
        let unique_names = unique_names.collect_names();

        stations.extend(types);
        stations.extend(unique_names);

        let x = stations
            .into_iter()
            .map(|(id, entry)| IdNameEntry {
                item_id: *id,
                name:    entry,
            })
            .collect::<Vec<_>>();

        Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertIdNameReq(x)
        )
        .await?;

        Ok(())
    }

    /// Extractes all items and inserts them into the database.
    async fn save_items(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let item_service  = sde.types().await?;
        let group_service = sde.groups().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all items together
        let mut entries = Vec::new();
        for (id, entry) in item_service.types() {
            let category = group_service.groups()
                .get(&entry.group_id)
                .map(|x| x.category_id)
                .unwrap();
            let name = entry.name().unwrap_or_default();
            let description = entry.description().unwrap_or_default();
            let volume = entry.volume.unwrap_or(0f32);
            entries.push(
                ItemEntry::new(
                    category.0,
                    entry.group_id.0,
                    id.0,
                    volume,
                    name,
                    description,
                )
            );
        }
        self.metrix
            .send_len(
                Self::METRIC_ITEM_VOLUME_COUNT,
                entries.len()
            )
            .await;

        // Save all entries in the database
        let timer = Instant::now();
        Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertItemReq(entries)
        )
        .await?;
        self.metrix.send_time(Self::METRIC_PROCESSING_ITEM, timer).await;

        Ok(())
    }

    /// Collect all item materials together and save them in the database.
    async fn save_item_materials(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let type_service = sde.types().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all items together
        let mut entries = Vec::new();
        for (id, materials) in type_service.materials() {
            for material in materials.materials.iter() {
                let material_id = material.material_type_id;
                let quantity = material.quantity;

                entries.push(
                    ItemMaterialEntry::new(
                        id.0,
                        material_id.0,
                        quantity,
                    )
                );
            }
        }
        self.metrix
            .send_len(Self::METRIC_MATERIAL_COUNT, entries.len())
            .await;

        // Save all materials in the database
        let timer = Instant::now();
        Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertItemMaterialReq(entries)
        )
        .await?;
        self.metrix.send_time(Self::METRIC_PROCESSING_MATERIAL, timer).await;

        Ok(())
    }

    /// Collects all stations an stores a subset of it in the database
    async fn save_system_region(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let system_service = sde.systems().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all entries
        let mut entries = Vec::new();
        for (cid, centry) in system_service.constellations() {
            let region = system_service.regions()
                .iter()
                .find(|(_, rentry)| rentry.constellations.contains(&cid))
                .map(|(rid, _)| rid)
                .unwrap();

            for system in centry.systems.iter() {
                let security = system_service.eve_systems()
                    .iter()
                    .find(|x: &&SolarsystemEntry| x.solar_system_id == *system)
                    .map(|x| x.security);

                if let Some(x) = security {
                    entries.push(SystemRegionEntry {
                        region_id: **region,
                        system_id: **system,
                        security:  x,
                    });
                }
            }
        }
        self.metrix.send_len(Self::METRIC_SYSTEM_REGION_COUNT, entries.len()).await;

        // Save all entries
        let timer = Instant::now();
        Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertSystemRegionReq(entries)
        )
        .await?;
        self.metrix.send_time(Self::METRIC_PROCESSING_STATION, timer).await;

        Ok(())
    }

    async fn save_blueprints(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let blueprint_service = sde.blueprints().await?;
        let schematic_service = sde.planet_schematics().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all blueprints
        let mut entries = Vec::new();
        for (id, blueprint) in blueprint_service.blueprints() {
            let mut activity  = Activity::Manufacturing;

            let activity_info = if let Some(x) = blueprint.activities.manufacturing.clone() {
                x
            } else if let Some(x) = blueprint.activities.reaction.clone() {
                activity = Activity::Reaction;
                x
            } else {
                log::error!("Unknown blueprint activity {:?}", blueprint.activities);
                BlueprintAdditional {
                    materials: None,
                    products:  None,
                    skills:    None,
                    time:      0,
                }
            };

            let time      = activity_info.time;
            let mut materials = Vec::new();
            let mut skills    = Vec::new();

            for material in activity_info.materials.unwrap_or_default() {
                let material_id = material.type_id;
                let quantity = material.quantity;
                let material = Material::new(material_id.0, quantity, false);
                materials.push(material);
            }

            for product in activity_info.products.unwrap_or_default() {
                let material_id = product.type_id;
                let quantity = product.quantity;
                materials.push(Material::new(material_id.0, quantity, true));
            }

            for skill in activity_info.skills.unwrap_or_default() {
                let level = skill.level as u8;
                let type_id = skill.type_id;
                skills.push(Skill::new(level, *type_id));
            }

            entries.push(BlueprintEntry::new(activity, id.0, time, materials, skills));
        }

        // Collect all schematics
        for (id, schematic) in schematic_service.schematics() {
            let time = schematic.cycle_time;
            let mut materials = Vec::new();

            for (material_id, schematic) in schematic.types.clone() {
                let quantity = schematic.quantity;
                let is_product = !schematic.is_input;
                let material = Material::new(material_id.0, quantity, is_product);
                materials.push(material);
            }

            entries.push(
                BlueprintEntry::new(
                    Activity::PlanetInteraction,
                    id.0,
                    time,
                    materials,
                    Vec::new()
                )
            );
        }
        self.metrix.send_len(Self::METRIC_BLUEPRINT_COUNT, entries.len()).await;

        // Save all entries
        let timer = Instant::now();
        Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertBlueprintReq(entries)
        )
        .await?;
        self.metrix.send_time(Self::METRIC_PROCESSING_BLUEPRINT, timer).await;

        Ok(())
    }
}
