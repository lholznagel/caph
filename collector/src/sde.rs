use cachem::{ConnectionPool, EmptyResponse, Protocol};
use caph_eve_sde_parser::{
    Blueprint, ParseRequest, ParseResult, Schematic, Station, TypeIds, TypeMaterial, UniqueName,
};
use caph_db::*;
use metrix_exporter::MetrixSender;
use std::collections::{HashMap, HashSet};
use std::io::Cursor;
use std::time::Instant;

use crate::error::{CollectorError, CollectorResult};

pub enum Action {
    Blueprint(Vec<BlueprintEntry>),
    IdName(Vec<IdNameEntry>),
    Item(Vec<ItemEntry>),
    ItemMaterial(Vec<ItemMaterialEntry>),
    Region(HashSet<RegionEntry>),
    Station(Vec<StationEntry>),
}

pub struct Sde {
    metrix: MetrixSender,
    pool: ConnectionPool,
}

impl Sde {
    const METRIC_SDE:                  &'static str = "sde::time::complete";
    const METRIC_FETCH_SDE:            &'static str = "sde::time::fetch";
    const METRIC_PARSE_SDE:            &'static str = "sde::time::parse";
    const METRIC_PROCESSING:           &'static str = "sde::time::processing";
    const METRIC_PROCESSING_ID_NAME:   &'static str = "sde::time::processing::id_name";
    const METRIC_PROCESSING_ITEM:      &'static str = "sde::time::processing::item";
    const METRIC_PROCESSING_MATERIAL:  &'static str = "sde::time::processing::material";
    const METRIC_PROCESSING_STATION:   &'static str = "sde::time::processing::station";
    const METRIC_PROCESSING_BLUEPRINT: &'static str = "sde::time::processing::blueprint";
    const METRIC_PROCESSING_REGION:    &'static str = "sde::time::processing::region";
    const METRIC_ID_NAME_COUNT:        &'static str = "sde::count::id_name";
    const METRIC_ITEM_NAME_COUNT:      &'static str = "sde::count::item_name";
    const METRIC_ITEM_VOLUME_COUNT:    &'static str = "sde::count::item_volume";
    const METRIC_MATERIAL_COUNT:       &'static str = "sde::count::material";
    const METRIC_STATION_COUNT:        &'static str = "sde::count::station";
    const METRIC_BLUEPRINT_COUNT:      &'static str = "sde::count::blueprint";
    const METRIC_SCHEMATIC_COUNT:      &'static str = "sde::count::schematic";
    const METRIC_REGION_COUNT:         &'static str = "sde::count::region";

    pub async fn new(metrix: MetrixSender, pool: ConnectionPool) -> Self {
        Self { metrix, pool }
    }

    pub async fn run(&mut self) -> CollectorResult<()> {
        let start = Instant::now();
        let timer = Instant::now();

        let zip = caph_eve_sde_parser::fetch_zip()
            .await
            .map_err(|_| CollectorError::DownloadSdeZip)?;

        self.metrix.send_time(Self::METRIC_FETCH_SDE, timer).await;
        let timer = Instant::now();

        let parse_results = caph_eve_sde_parser::from_reader(
            &mut Cursor::new(zip),
            vec![
                ParseRequest::TypeIds,
                ParseRequest::TypeMaterials,
                ParseRequest::UniqueNames,
                ParseRequest::Stations,
                ParseRequest::Blueprints,
                ParseRequest::Schematics,
            ],
        )
        .map_err(CollectorError::SdeParserError)?;

        self.metrix.send_time(Self::METRIC_PARSE_SDE, timer).await;
        let timer = Instant::now();

        // collects all actions that need to be perfomed
        let mut actions: Vec<Action> = Vec::new();
        //let mut conn = self.pool.acquire().await?;
        for parse_result in parse_results {
            let x = match parse_result {
                ParseResult::TypeIds(x) => self.items(x).await?,
                ParseResult::TypeMaterials(x) => self.item_materials(x).await?,
                ParseResult::UniqueNames(x) => self.unique_names(x).await?,
                ParseResult::Stations(x) => self.stations(x).await?,
                ParseResult::Blueprints(x) => self.blueprints(x).await?,
                ParseResult::Schematic(x) => self.schematics(x).await?,
            };
            actions.extend(x);
        }
        self.metrix.send_time(Self::METRIC_PROCESSING, timer).await;

        for action in actions {
            let mut conn = self.pool.acquire().await?;
            match action {
                Action::IdName(x) => {
                    let timer = Instant::now();
                    Protocol::request::<_, EmptyResponse>(
                        &mut conn,
                        InsertIdNameReq(x)
                    )
                    .await?;
                    self.metrix.send_time(Self::METRIC_PROCESSING_ID_NAME, timer).await;
                },
                Action::Item(x) => {
                    let timer = Instant::now();
                    Protocol::request::<_, EmptyResponse>(
                        &mut conn,
                        InsertItemReq(x)
                    )
                    .await?;
                    self.metrix.send_time(Self::METRIC_PROCESSING_ITEM, timer).await;
                },
                Action::ItemMaterial(x) => {
                    let timer = Instant::now();
                    Protocol::request::<_, EmptyResponse>(
                        &mut conn,
                        InsertItemMaterialReq(x)
                    )
                    .await?;
                    self.metrix.send_time(Self::METRIC_PROCESSING_MATERIAL, timer).await;
                },
                Action::Station(x) => {
                    let timer = Instant::now();
                    Protocol::request::<_, EmptyResponse>(
                        &mut conn,
                        InsertStationReq(x)
                    )
                    .await?;
                    self.metrix.send_time(Self::METRIC_PROCESSING_STATION, timer).await;
                },
                Action::Blueprint(x) => {
                    let timer = Instant::now();
                    Protocol::request::<_, EmptyResponse>(
                        &mut conn,
                        InsertBlueprintReq(x)
                    )
                    .await?;
                    self.metrix.send_time(Self::METRIC_PROCESSING_BLUEPRINT, timer).await;
                }
                Action::Region(x) => {
                    let timer = Instant::now();
                    Protocol::request::<_, EmptyResponse>(
                        &mut conn,
                        InsertRegionReq(x)
                    )
                    .await?;
                    self.metrix.send_time(Self::METRIC_PROCESSING_REGION, timer).await;
                }
            }
        }

        self.metrix.send_time(Self::METRIC_SDE, start).await;

        Ok(())
    }

    async fn items(
        &mut self,
        items: HashMap<u32, TypeIds>,
    ) -> Result<Vec<Action>, CollectorError> {
        // We know the roughly how many items there are, so we allocate accordingly
        let mut item_name_actions = Vec::with_capacity(ItemCache::CAPACITY);
        let mut item_volume_actions = Vec::with_capacity(ItemCache::CAPACITY);

        for (id, type_id) in items {
            let name = type_id
                .name
                .get("en")
                .map(|x| x.clone())
                .unwrap_or_default();
            item_name_actions.push(IdNameEntry::new(id, name));

            let volume = type_id.volume.unwrap_or(0f32);
            item_volume_actions.push(ItemEntry::new(id, volume));
        }
        self.metrix.send_len(Self::METRIC_ITEM_NAME_COUNT, item_name_actions.len()).await;
        self.metrix.send_len(Self::METRIC_ITEM_VOLUME_COUNT, item_volume_actions.len()).await;

        let item_name_actions = Action::IdName(item_name_actions);
        let item_volume_actions = Action::Item(item_volume_actions);
        Ok(vec![
            item_volume_actions,
            item_name_actions,
        ])
    }

    async fn item_materials(
        &mut self,
        materials: HashMap<u32, TypeMaterial>,
    ) -> Result<Vec<Action>, CollectorError> {
        // We know the roughly how many items there are, so we allocate accordingly
        let mut item_material_actions = Vec::with_capacity(ItemMaterialCache::CAPACITY);

        for (id, materials) in materials {
            for material in materials.materials.iter() {
                let material_id = material.material_type_id;
                let quantity = material.quantity;

                item_material_actions.push(ItemMaterialEntry::new(id, material_id, quantity));
            }
        }
        self.metrix.send_len(Self::METRIC_MATERIAL_COUNT, item_material_actions.len()).await;

        let item_material_actions = Action::ItemMaterial(item_material_actions);
        Ok(vec![
            item_material_actions,
        ])
    }

    async fn unique_names(
        &mut self,
        names: Vec<UniqueName>,
    ) -> Result<Vec<Action>, CollectorError> {
        // We know the roughly how many items there are, so we allocate accordingly
        let mut id_name_actions = Vec::with_capacity(IdNameCache::CAPACITY);

        for name in names {
            let id = name.item_id;
            let name = name.item_name;
            id_name_actions.push(IdNameEntry::new(id, name));
        }
        self.metrix.send_len(Self::METRIC_ID_NAME_COUNT, id_name_actions.len()).await;

        let id_name_actions = Action::IdName(id_name_actions);
        Ok(vec![
            id_name_actions
        ])
    }

    async fn stations(
        &mut self,
        stations: Vec<Station>,
    ) -> Result<Vec<Action>, CollectorError> {
        // We know the roughly how many items there are, so we allocate accordingly
        let mut region_actions = HashSet::with_capacity(RegionCache::CAPACITY);
        let mut station_actions = Vec::with_capacity(StationCache::CAPACITY);

        for station in stations {
            let id = station.station_id;
            let region_id = station.region_id;
            let system_id = station.solar_system_id;
            let security = station.security;
            station_actions.push(
                StationEntry::new(id, region_id, system_id, security)
            );
            region_actions.insert(
                RegionEntry::new(station.region_id)
            );
        }
        self.metrix.send_len(Self::METRIC_REGION_COUNT, region_actions.len()).await;
        self.metrix.send_len(Self::METRIC_STATION_COUNT, station_actions.len()).await;

        let region_actions = Action::Region(region_actions);
        let station_actions = Action::Station(station_actions);
        Ok(vec![
            region_actions,
            station_actions
        ])
    }

    async fn blueprints(
        &mut self,
        blueprints: HashMap<u32, Blueprint>,
    ) -> Result<Vec<Action>, CollectorError> {
        // We know the roughly how many items there are, so we allocate accordingly
        let mut blueprint_actions = Vec::with_capacity(BlueprintCache::CAPACITY);

        for (id, blueprint) in blueprints {
            let mut time = 0;
            let mut materials = Vec::new();

            for x in blueprint.activities.manufacturing {
                time = x.time;

                for material in x.materials.unwrap_or_default() {
                    let material_id = material.type_id;
                    let quantity = material.quantity;
                    let material = Material::new(material_id, quantity, false);
                    materials.push(material);
                }

                for product in x.products.unwrap_or_default() {
                    let material_id = product.type_id;
                    let quantity = product.quantity;
                    let material = Material::new(material_id, quantity, true);
                    materials.push(material);
                }
            }

            blueprint_actions.push(
                BlueprintEntry::new(id, time, materials)
            );
        }
        self.metrix.send_len(Self::METRIC_BLUEPRINT_COUNT, blueprint_actions.len()).await;

        let blueprint_actions = Action::Blueprint(blueprint_actions);
        Ok(vec![
            blueprint_actions
        ])
    }

    async fn schematics(
        &mut self,
        schematics: HashMap<u32, Schematic>,
    ) -> Result<Vec<Action>, CollectorError> {
        // We know the roughly how many items there are, so we allocate accordingly
        let mut schematic_actions = Vec::with_capacity(BlueprintCache::CAPACITY);

        for (id, schematic) in schematics {
            let time = schematic.cycle_time;
            let mut materials = Vec::new();

            for (material_id, schematic) in schematic.types {
                let quantity = schematic.quantity;
                let is_product = !schematic.is_input;
                let material = Material::new(material_id, quantity, is_product);
                materials.push(material);
            }

            schematic_actions.push(
                BlueprintEntry::new(id, time, materials)
            );
        }
        self.metrix.send_len(Self::METRIC_SCHEMATIC_COUNT, schematic_actions.len()).await;

        let schematic_actions = Action::Blueprint(schematic_actions);
        Ok(vec![
            schematic_actions
        ])
    }
}
