use crate::error::CollectorError;

use cachem::{ConnectionPool, EmptyMsg, Protocol};
use caph_db::*;
use caph_eve_sde_parser2::SdeServiceLoader;
use metrix_exporter::MetrixSender;
use std::time::Instant;

pub struct Sde {
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
    const METRIC_STATION_COUNT:        &'static str = "sde::count::station";
    const METRIC_BLUEPRINT_COUNT:      &'static str = "sde::count::blueprint";

    pub fn new(metrix: MetrixSender, pool: ConnectionPool) -> Self {
        Self { metrix, pool }
    }

    pub async fn run(&mut self) -> Result<(), CollectorError> {
        let start = Instant::now();
        let timer = Instant::now();

        // Download the current zip
        let sde_zip = SdeServiceLoader::new().await?;
        self.metrix.send_time(Self::METRIC_FETCH_SDE, timer).await;

        self.save_blueprints(&sde_zip).await?;
        self.save_item_materials(&sde_zip).await?;
        self.save_items(&sde_zip).await?;
        self.save_stations(&sde_zip).await?;

        self.metrix.send_time(Self::METRIC_SDE, start).await;

        Ok(())
    }

    /// Extractes all items and inserts them into the database.
    async fn save_items(&self, sde: &SdeServiceLoader) -> Result<(), CollectorError> {
        let item_service  = sde.type_ids().await?;
        let group_service = sde.groups().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all items together
        let mut entries = Vec::new();
        for (id, entry) in item_service.0 {
            let category = group_service.0
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
    async fn save_item_materials(&self, sde: &SdeServiceLoader) -> Result<(), CollectorError> {
        let material_service = sde.type_materials().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all items together
        let mut entries = Vec::new();
        for (id, materials) in material_service.0 {
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
    async fn save_stations(&self, sde: &SdeServiceLoader) -> Result<(), CollectorError> {
        let station_service = sde.stations().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all entries
        let mut entries = Vec::new();
        for station in station_service.stations {
            let id = station.station_id;
            let region_id = station.region_id;
            let system_id = station.solar_system_id;
            let security = station.security;
            entries.push(
                StationEntry::new(
                    id.0,
                    region_id.0,
                    system_id.0,
                    security
                )
            );
        }
        self.metrix.send_len(Self::METRIC_STATION_COUNT, entries.len()).await;

        // Save all entries
        let timer = Instant::now();
        Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertStationReq(entries)
        )
        .await?;
        self.metrix.send_time(Self::METRIC_PROCESSING_STATION, timer).await;

        Ok(())
    }

    async fn save_blueprints(&self, sde: &SdeServiceLoader) -> Result<(), CollectorError> {
        let blueprint_service = sde.blueprints().await?;
        let schematic_service = sde.planet_schematics().await?;

        let mut conn = self.pool.acquire().await?;

        // Collect all blueprints
        let mut entries = Vec::new();
        for (_, blueprint) in blueprint_service.0 {
            let mut materials = Vec::new();

            for x in blueprint.activities.manufacturing {
                for material in x.materials.unwrap_or_default() {
                    let material_id = material.type_id;
                    let quantity = material.quantity;
                    let material = Material::new(material_id.0, quantity, false);
                    materials.push(material);
                }

                for product in x.products.unwrap_or_default() {
                    let material_id = product.type_id;
                    let quantity = product.quantity;
                    materials.push(Material::new(material_id.0, quantity, true));
                }
            }
        }

        // Collect all schematics
        for (id, schematic) in schematic_service.0 {
            let time = schematic.cycle_time;
            let mut materials = Vec::new();

            for (material_id, schematic) in schematic.types {
                let quantity = schematic.quantity;
                let is_product = !schematic.is_input;
                let material = Material::new(material_id.0, quantity, is_product);
                materials.push(material);
            }

            entries.push(BlueprintEntry::new(id.0, time, materials));
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
