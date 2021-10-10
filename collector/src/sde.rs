use crate::error::CollectorError;

use caph_connector::{BlueprintMaterial, BlueprintSkill, SdeService, TypeId};
use caph_connector::services::*;
use serde::Serialize;
use sqlx::{PgPool, Type};
use std::{collections::{HashMap, VecDeque}, fmt};
use uuid::Uuid;

/// Responsible for processing EVE-SDE files
pub struct Sde {
    /// Connection pool to a postgres database
    pool: PgPool,
}

impl Sde {
    /// Creates a new instance
    ///
    /// # Params
    ///
    /// * `pool` -> Open connction pool to a postgres
    ///
    /// # Returns
    ///
    /// New instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool}
    }

    /// Starts the processing of a sde.zip file
    ///
    /// # Errors
    ///
    /// Fails when there is an error while processing the file
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    pub async fn run(&mut self) -> Result<(), CollectorError> {
        let mut zip = SdeService::new()
            .await
            .map_err(CollectorError::LoadingSdeZip)?;

        let asset_service = ConnectAssetService::new(&mut zip)
            .map_err(CollectorError::LoadSdeFile)?;
        let blueprint_service = ConnectBlueprintService::new(&mut zip)
            .map_err(CollectorError::LoadSdeFile)?;
        let reprocess_service = ConnectReprocessService::new(&mut zip)
            .map_err(CollectorError::LoadSdeFile)?;
        let schematic_service = ConnectSchematicService::new(&mut zip)
            .map_err(CollectorError::LoadSdeFile)?;
        let station_service = ConnectStationService::new(&mut zip)
            .map_err(CollectorError::LoadSdeFile)?;
        let system_service = ConnectSystemService::new(&mut zip)
            .map_err(CollectorError::LoadSdeFile)?;
        let unique_name_service = ConnectUniqueNameService::new(&mut zip)
            .map_err(CollectorError::LoadSdeFile)?;

        self.save_assets(&asset_service).await?;
        self.save_blueprints(&blueprint_service).await?;
        self.save_reprocessing_info(&reprocess_service).await?;
        self.save_schematics(&schematic_service).await?;
        self.save_stations(&station_service).await?;
        self.save_systems(&system_service, &unique_name_service).await?;

        self.blueprint_tree(
            &asset_service,
            &blueprint_service,
            &schematic_service
        ).await?;

        Ok(())
    }

    /// Extractes all items and inserts them into the database.
    ///
    /// # Params
    ///
    /// * `asset_service` -> Service that holds SDE information about assets
    ///
    /// # Errors
    ///
    /// Failes when a database operation fails
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    async fn save_assets(&self, asset_service: &ConnectAssetService) -> Result<(), CollectorError> {
        let mut type_ids = Vec::new();
        let mut categories = Vec::new();
        let mut group_ids = Vec::new();
        let mut volumes = Vec::new();
        let mut names = Vec::new();

        tracing::debug!(task = "asset", "Loading asset information");
        let asset_type_ids = asset_service.type_ids();
        let asset_group_ids = asset_service.group_ids();
        tracing::debug!(task = "asset", "Loaded asset information");

        tracing::debug!(task = "asset", "Start preparing assets");
        // Collect all items together
        for (tid, entry) in asset_type_ids {
            let category = *asset_group_ids
                .get(&entry.group_id)
                .map(|x| x.category_id)
                .expect("Every entry should have a category id");
            let group_id = *entry.group_id;
            let name = entry.name().unwrap_or_default();
            let volume = entry.volume.unwrap_or(0f32);

            type_ids.push(**tid);
            categories.push(category);
            group_ids.push(group_id);
            names.push(name);
            volumes.push(volume);
        }
        tracing::debug!(task = "asset", "Finsihed preparing assets");

        tracing::debug!(task = "asset", "Start inserting assets in DB");
        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("
                INSERT INTO item
                (
                    type_id,
                    category_id,
                    group_id,
                    volume,
                    name
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::INTEGER[],
                    $3::INTEGER[],
                    $4::REAL[],
                    $5::VARCHAR[]
                )
                ON CONFLICT(type_id) DO UPDATE
                SET category_id = excluded.category_id,
                    group_id    = excluded.group_id,
                    volume      = excluded.volume,
                    name        = excluded.name
            ",
                &type_ids,
                &categories,
                &group_ids,
                &volumes,
                &names
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeItem)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;
        tracing::debug!(task = "asset", "Finished inserting assets in DB");

        Ok(())
    }

    /// Collect all item materials together and save them in the database.
    ///
    /// # Params
    ///
    /// * `reprocess_service` -> Service that holds SDE information about
    ///                          reprocessing
    ///
    /// # Errors
    ///
    /// Failes when a database operation fails
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    async fn save_reprocessing_info(
        &self,
        reprocess_service: &ConnectReprocessService
    ) -> Result<(), CollectorError> {
        let mut type_ids = Vec::new();
        let mut material_ids = Vec::new();
        let mut quantities = Vec::new();

        // Collect all items together
        for (tid, materials) in reprocess_service.entries() {
            for material in materials.materials.iter() {
                let material_id = material.type_id;
                let quantity = material.quantity;

                type_ids.push(**tid);
                material_ids.push(*material_id);
                quantities.push(quantity);
            }
        }

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("DELETE FROM reprocess")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeReprocess)?;
        sqlx::query!("
                INSERT INTO reprocess
                (
                    type_id,
                    material_id,
                    quantity
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
            ",
                &type_ids,
                &material_ids,
                &quantities,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeReprocess)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

        Ok(())
    }

    /// Collect all blueprints together and save them in the database.
    ///
    /// # Params
    ///
    /// * `blueprint_service` -> Service that holds SDE information about
    ///                          blueprints
    ///
    /// # Errors
    ///
    /// Failes when a database operation fails
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    async fn save_blueprints(
        &self,
        blueprint_service: &ConnectBlueprintService
    ) -> Result<(), CollectorError> {
        let count = blueprint_service.entries().len();

        // blueprint
        let mut ids = Vec::with_capacity(count);
        let mut type_id = Vec::with_capacity(count);
        let mut limit = Vec::with_capacity(count);
        let mut copy = Vec::with_capacity(count);
        let mut invention = Vec::with_capacity(count);
        let mut manufacture = Vec::with_capacity(count);
        let mut reaction = Vec::with_capacity(count);
        let mut research_mat = Vec::with_capacity(count);
        let mut research_time = Vec::with_capacity(count);

        // blueprint_material
        let mut bm_blueprint_id = Vec::new();
        let mut bm_type_id = Vec::new();
        let mut bm_quantity = Vec::new();
        let mut bm_is_product = Vec::new();
        let mut bm_probability = Vec::new();
        let mut bm_activity = Vec::new();

        // blueprint_skill
        let mut bs_blueprint_id = Vec::new();
        let mut bs_type_id = Vec::new();
        let mut bs_level = Vec::new();
        let mut bs_activity = Vec::new();

        let mut insert_mat = |
            id:        Uuid,
            materials: &Vec<BlueprintMaterial>,
            products:  &Vec<BlueprintMaterial>,
            activity:  BlueprintActivity| {
            for material in materials {
                bm_blueprint_id.push(id);
                bm_type_id.push(*material.type_id);
                bm_quantity.push(material.quantity);
                bm_is_product.push(false);
                bm_probability.push(material.probability);
                bm_activity.push(activity.into());
            }

            for product in products {
                bm_blueprint_id.push(id);
                bm_type_id.push(*product.type_id);
                bm_quantity.push(product.quantity);
                bm_is_product.push(true);
                bm_probability.push(product.probability);
                bm_activity.push(activity.into());
            }
        };
        let mut insert_skill = |
            id:       Uuid,
            skills:   &Vec<BlueprintSkill>,
            activity: BlueprintActivity| {

            for skill in skills {
                bs_blueprint_id.push(id);
                bs_type_id.push(*skill.type_id);
                bs_level.push(skill.level);
                bs_activity.push(activity.into());
            }
        };

        for (bid, entry) in blueprint_service.entries() {
            let id = Uuid::new_v4();

            ids.push(id);
            type_id.push(**bid);
            limit.push(entry.max_production_limit);

            if let Some(x) = &entry.activities.copying {
                copy.push(Some(x.time));
                insert_mat(id, &x.materials, &x.products, BlueprintActivity::Copy);
                insert_skill(id, &x.skills, BlueprintActivity::Copy);
            } else {
                copy.push(None);
            }
            if let Some(x) = &entry.activities.invention {
                invention.push(Some(x.time));
                insert_mat(id, &x.materials, &x.products, BlueprintActivity::Invention);
                insert_skill(id, &x.skills, BlueprintActivity::Invention);
            } else {
                invention.push(None);
            }
            if let Some(x) = &entry.activities.manufacturing {
                let materials = &x.materials;
                let products = &x.products;

                if materials.len() == 1 && products.len() == 1 &&
                   materials[0].type_id == products[0].type_id {
                    manufacture.push(None)
                } else {
                    manufacture.push(Some(x.time));
                    insert_mat(id, materials, &x.products, BlueprintActivity::Manufacture);
                    insert_skill(id, &x.skills, BlueprintActivity::Manufacture);
                }
            } else {
                manufacture.push(None);
            }
            if let Some(x) = &entry.activities.reaction {
                reaction.push(Some(x.time));
                insert_mat(id, &x.materials, &x.products, BlueprintActivity::Reaction);
                insert_skill(id, &x.skills, BlueprintActivity::Reaction);
            } else {
                reaction.push(None);
            }
            if let Some(x) = &entry.activities.research_material {
                research_mat.push(Some(x.time));
                insert_mat(id, &x.materials, &x.products, BlueprintActivity::ResearchMaterial);
                insert_skill(id, &x.skills, BlueprintActivity::ResearchMaterial);
            } else {
                research_mat.push(None);
            }
            if let Some(x) = &entry.activities.research_time {
                research_time.push(Some(x.time));
                insert_mat(
                    id,
                    &x.materials,
                    &x.products,
                    BlueprintActivity::ResearchTime);
                insert_skill(id, &x.skills, BlueprintActivity::ResearchTime);
            } else {
                research_time.push(None);
            }
        }

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("DELETE FROM blueprint CASCADE")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeBlueprint)?;
        sqlx::query!("
                INSERT INTO blueprint
                (
                    id,
                    type_id,
                    limit_,

                    copy,
                    invention,
                    manufacture,
                    reaction,
                    research_material,
                    research_time
                )
                SELECT * FROM UNNEST(
                    $1::UUID[],
                    $2::INTEGER[],
                    $3::INTEGER[],
                    $4::INTEGER[],
                    $5::INTEGER[],
                    $6::INTEGER[],
                    $7::INTEGER[],
                    $8::INTEGER[],
                    $9::INTEGER[]
                )
            ",
                &ids,
                &type_id,
                &limit,
                &copy as _,
                &invention as _,
                &manufacture as _,
                &reaction as _,
                &research_mat as _,
                &research_time as _
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeBlueprint)?;

        sqlx::query!("
                INSERT INTO blueprint_material
                (
                    blueprint,
                    activity,
                    type_id,

                    quantity,
                    is_product,
                    probability
                )
                SELECT * FROM UNNEST(
                    $1::UUID[],
                    $2::SMALLINT[],
                    $3::INTEGER[],
                    $4::INTEGER[],
                    $5::BOOLEAN[],
                    $6::REAL[]
                )
            ",
                &bm_blueprint_id,
                &bm_activity,
                &bm_type_id,
                &bm_quantity,
                &bm_is_product,
                &bm_probability as _,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeBlueprintMaterial)?;

        sqlx::query!("
                INSERT INTO blueprint_skill
                (
                    blueprint,

                    activity,

                    type_id,
                    level
                )
                SELECT * FROM UNNEST(
                    $1::UUID[],
                    $2::SMALLINT[],
                    $3::INTEGER[],
                    $4::INTEGER[]
                )
                ON CONFLICT DO NOTHING
            ",
                &bs_blueprint_id,
                &bs_activity,
                &bs_type_id,
                &bs_level,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeBlueprintSkill)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

        Ok(())
    }

    /// Collect all schematic together and save them in the database.
    ///
    /// # Params
    ///
    /// * `schematic_service` -> Service that holds SDE information about
    ///                          schematics
    ///
    /// # Errors
    ///
    /// Failes when a database operation fails
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    async fn save_schematics(
        &self,
        schematic_service: &ConnectSchematicService
    ) -> Result<(), CollectorError> {
        let mut s_ids = Vec::new();
        let mut s_type_ids = Vec::new();
        let mut s_cycle_times = Vec::new();

        let mut sm_schematic = Vec::new();
        let mut sm_type_id = Vec::new();
        let mut sm_is_input = Vec::new();
        let mut sm_quantity = Vec::new();

        for (type_id, entry) in schematic_service.entries() {
            let id = Uuid::new_v4();

            s_ids.push(id);
            s_type_ids.push(**type_id);
            s_cycle_times.push(entry.cycle_time);

            for (type_id, entry) in entry.types.clone() {
                sm_schematic.push(id);
                sm_type_id.push(*type_id);
                sm_is_input.push(entry.is_input);
                sm_quantity.push(entry.quantity);
            }
        }

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("DELETE FROM schematic CASCADE")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeSchematic)?;
        sqlx::query!("
                INSERT INTO schematic
                (
                    id,

                    type_id,
                    cycle_time
                )
                SELECT * FROM UNNEST(
                    $1::UUID[],
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
            ",
                &s_ids,
                &s_type_ids,
                &s_cycle_times,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeSchematic)?;

        sqlx::query!("
                INSERT INTO schematic_material
                (
                    schematic,

                    type_id,
                    is_input,
                    quantity
                )
                SELECT * FROM UNNEST(
                    $1::UUID[],
                    $2::INTEGER[],
                    $3::BOOLEAN[],
                    $4::INTEGER[]
                )
            ",
                &sm_schematic,
                &sm_type_id,
                &sm_is_input,
                &sm_quantity
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeSchematicMaterial)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

        Ok(())
    }

    /// Collect all stations together and save them in the database.
    ///
    /// # Params
    ///
    /// * `station_service` -> Service that holds SDE information about
    ///                        stations
    ///
    /// # Errors
    ///
    /// Failes when a database operation fails
    ///
    /// # Returns
    ///
    /// Nothing
    ///
    async fn save_stations(
        &self,
        station_service:     &ConnectStationService,
    ) -> Result<(), CollectorError> {
        let mut ids = Vec::new();
        let mut systems = Vec::new();
        let mut names = Vec::new();

        for entry in station_service.entries() {
            ids.push(*entry.id);
            systems.push(*entry.solar_system_id);
            names.push(entry.name.clone());
        }

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("
                DELETE FROM station CASCADE
                WHERE pos IS FALSE
            ")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeStation)?;
        sqlx::query!("
                INSERT INTO station
                (
                    id,
                    system_id,
                    name
                )
                SELECT * FROM UNNEST(
                    $1::BIGINT[],
                    $2::BIGINT[],
                    $3::VARCHAR[]
                )
            ",
                &ids,
                &systems,
                &names,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeStation)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

        Ok(())
    }

    async fn save_systems(
        &self,
        system_service:      &ConnectSystemService,
        unique_name_service: &ConnectUniqueNameService,
    ) -> Result<(), CollectorError> {
        let mut ids = Vec::new();
        let mut names = Vec::new();

        let unique_names = unique_name_service.entries();
        for entry in system_service.entries() {
            ids.push(*entry.id);
            names
                .push(
                    unique_names
                        .get(&(*entry.id).into())
                        .unwrap_or(&String::new())
                        .clone()
                );
        }

        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("DELETE FROM system CASCADE")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeSystem)?;
        sqlx::query!("
                INSERT INTO system
                (
                    id,
                    name
                )
                SELECT * FROM UNNEST(
                    $1::BIGINT[],
                    $2::VARCHAR[]
                )
            ",
                &ids,
                &names,
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeSystem)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;

        Ok(())
    }

    async fn blueprint_tree(
        &self,
        asset_service:     &ConnectAssetService,
        blueprint_service: &ConnectBlueprintService,
        schematic_service: &ConnectSchematicService
    ) -> Result<(), CollectorError> {
        let assets = asset_service.type_ids().clone();

        let productions = blueprint_service
            .entries()
            .clone()
            .iter()
            .filter(|(_, x)| x.activities.manufacturing.is_some())
            .filter(|(_, x)| x.activities.manufacturing.clone().unwrap().products.len() > 0)
            .map(|(_, e)| {
                let activity = e.activities.clone().manufacturing.unwrap();
                let product = activity.products[0].clone();
                let materials = activity
                    .materials
                    .into_iter()
                    .map(|x| {
                        ProductMaterial {
                            type_id:  x.type_id,
                            quantity: x.quantity
                        }
                    })
                    .collect::<Vec<_>>();
                BlueprintProduct {
                    type_id:  product.type_id,
                    quantity: product.quantity,
                    materials
                }
            })
            .map(|x| (x.type_id, x))
            .collect::<HashMap<_, _>>();
        let reactions = blueprint_service
            .entries()
            .clone()
            .iter()
            .filter(|(_, x)| x.activities.reaction.is_some())
            .map(|(_, e)| {
                let activity = e.activities.clone().reaction.unwrap();
                let product = activity.products[0].clone();
                let materials = activity
                    .materials
                    .into_iter()
                    .map(|x| {
                        ProductMaterial {
                            type_id:  x.type_id,
                            quantity: x.quantity
                        }
                    })
                    .collect::<Vec<_>>();
                BlueprintProduct {
                    type_id:  product.type_id,
                    quantity: product.quantity,
                    materials
                }
            })
            .map(|x| (x.type_id, x))
            .collect::<HashMap<_, _>>();
        let schematics = schematic_service
            .entries()
            .clone()
            .iter()
            .map(|(_, e)| {
                let materials = e.materials()
                    .into_iter()
                    .map(|(type_id, x)| {
                        ProductMaterial {
                            type_id:  type_id,
                            quantity: x.quantity
                        }
                    })
                    .collect::<Vec<_>>();
                BlueprintProduct {
                    type_id:  e.product().0,
                    quantity: e.product().1.quantity,
                    materials
                }
            })
            .map(|x| (x.type_id, x))
            .collect::<HashMap<_, _>>();

        let mut resolved: HashMap<TypeId, BlueprintTree> = HashMap::new();

        let mut queue_bps = productions
            .iter()
            .map(|(_, x)| x)
            .cloned()
            .collect::<VecDeque<_>>();
        let queue_reactions = reactions
            .iter()
            .map(|(_, x)| x)
            .cloned()
            .collect::<VecDeque<_>>();
        let queue_schematics = schematics
            .iter()
            .map(|(_, x)| x)
            .cloned()
            .collect::<VecDeque<_>>();
        queue_bps.extend(queue_reactions);
        queue_bps.extend(queue_schematics);

        while let Some(x) = queue_bps.pop_front() {
            let mut all_resolved = true;
            let mut ignore = false;
            for pmaterial in x.materials.iter() {
                // Some blueprints require the same item they produce
                if x.type_id == pmaterial.type_id {
                    ignore = true;
                    continue;
                }

                if resolved.contains_key(&pmaterial.type_id) {
                    continue;
                } else if productions.contains_key(&pmaterial.type_id) {
                    all_resolved = false;
                } else if reactions.contains_key(&pmaterial.type_id) {
                    all_resolved = false;
                } else if schematics.contains_key(&pmaterial.type_id) {
                    all_resolved = false;
                } else {
                    continue;
                }
            }

            if all_resolved && !ignore {
                let mut children = Vec::new();
                for pmaterial in x.materials {
                    if let Some(x) = resolved.get(&pmaterial.type_id) {
                        let x: BlueprintTree = x.clone();
                        children.push(x);
                    } else {
                        let name = if let Some(x) = assets.get(&pmaterial.type_id) {
                            x.name().unwrap_or_default()
                        } else {
                            format!("Unknown {}", *x.type_id)
                        };

                        children.push(BlueprintTree {
                            key:      pmaterial.type_id,
                            label:    name,
                            quantity: pmaterial.quantity,
                            children: None
                        });
                    }
                }

                let name = if let Some(x) = assets.get(&x.type_id) {
                    x.name().unwrap_or_default()
                } else {
                    format!("Unknown {}", *x.type_id)
                };
                resolved.insert(x.type_id, BlueprintTree {
                    key:      x.type_id,
                    label:    name,
                    quantity: x.quantity,
                    children: Some(children)
                });
            } else {
                // At least one material is unknown, so we put this entry back
                // and try it later again
                if !ignore {
                    queue_bps.push_back(x);
                }
            }
        }

        let mut flat_map = Vec::new();
        let mut resolved_raw_ressources = HashMap::new();
        for (type_id, entry) in resolved.iter() {
            let mut raw_resources = HashMap::new();
            let mut resources = VecDeque::new();
            resources.extend(entry.clone().children.unwrap_or_default());

            while let Some(x) = resources.pop_front() {
                let children = x.children.clone().unwrap_or_default();
                if children.is_empty() {
                    raw_resources
                        .entry(x.key)
                        .and_modify(|e| *e += x.quantity)
                        .or_insert(x.quantity);
                } else {
                    resources.extend(children);

                    flat_map.push(BlueprintFlat {
                        type_id:  type_id.clone(),
                        mtype_id: x.key,
                        quantity: x.quantity,
                    });
                }
            }

            resolved_raw_ressources.insert(type_id, raw_resources);
        }

        let type_ids = resolved.iter().map(|(x, _)| **x).collect::<Vec<_>>();
        let trees = resolved
            .iter()
            .map(|(_, x)| serde_json::to_value(x).unwrap())
            .collect::<Vec<_>>();

        tracing::debug!(task = "blueprint_tree", "Start inserting tree in DB");
        let mut trans = self.pool
            .begin()
            .await
            .map_err(CollectorError::TransactionBeginNotSuccessfull)?;

        sqlx::query!("DELETE FROM blueprint_tree CASCADE")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeBlueprintTree)?;
        sqlx::query!("
                INSERT INTO blueprint_tree
                (
                    type_id,
                    tree
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::JSON[]
                )
            ",
                &type_ids,
                &trees
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeBlueprintTree)?;

        let mut item_type_ids = Vec::new();
        let mut material_type_ids = Vec::new();
        let mut material_quantities = Vec::new();
        for (type_id, entries) in resolved_raw_ressources {
            for (mtype_id, quantity) in entries {
                item_type_ids.push(**type_id);
                material_type_ids.push(*mtype_id);
                material_quantities.push(quantity);
            }
        }
        sqlx::query!("DELETE FROM blueprint_raw CASCADE")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeBlueprintRaw)?;
        sqlx::query!("
                INSERT INTO blueprint_raw
                (
                    type_id,
                    mtype_id,
                    quantity
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
            ",
                &item_type_ids,
                &material_type_ids,
                &material_quantities
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeBlueprintRaw)?;

        let mut item_type_ids = Vec::new();
        let mut material_type_ids = Vec::new();
        let mut material_quantities = Vec::new();
        for entry in flat_map {
            item_type_ids.push(*entry.type_id);
            material_type_ids.push(*entry.mtype_id);
            material_quantities.push(entry.quantity);
        }
        sqlx::query!("DELETE FROM blueprint_flat CASCADE")
            .execute(&mut trans)
            .await
            .map_err(CollectorError::DeletingSdeBlueprintFlat)?;
        sqlx::query!("
                INSERT INTO blueprint_flat
                (
                    type_id,
                    mtype_id,
                    quantity
                )
                SELECT * FROM UNNEST(
                    $1::INTEGER[],
                    $2::INTEGER[],
                    $3::INTEGER[]
                )
                ON CONFLICT DO NOTHING
            ",
                &item_type_ids,
                &material_type_ids,
                &material_quantities
            )
            .execute(&mut trans)
            .await
            .map_err(CollectorError::InsertingSdeBlueprintFlat)?;

        trans.commit()
            .await
            .map_err(CollectorError::TransactionCommitNotSuccessfull)?;
        tracing::debug!(task = "asset", "Finished inserting assets in DB");

        Ok(())
    }
}


#[derive(Clone, Debug, Serialize)]
struct BlueprintTree {
    key:      TypeId,
    label:    String,
    quantity: i32,
    children: Option<Vec<BlueprintTree>>
}

#[derive(Clone, Debug)]
struct BlueprintProduct {
    type_id:   TypeId,
    quantity:  i32,
    materials: Vec<ProductMaterial>
}

#[derive(Clone, Debug)]
struct BlueprintFlat {
    type_id:  TypeId,
    mtype_id: TypeId,
    quantity: i32
}

#[derive(Clone, Debug)]
struct ProductMaterial {
    type_id:  TypeId,
    quantity: i32,
}

/// All valid activities that a blueprint can have
#[derive(Copy, Clone, Debug, Type)]
pub enum BlueprintActivity {
    /// Activity of copying a blueprint, to create a bpc
    Copy,
    /// Activity of inventing a blueprint, to create an improved one
    Invention,
    /// Activity of manufacturing an item from a blueprint
    Manufacture,
    /// Activity of creating an item using reactions
    Reaction,
    /// Activity of researching material to reduce the amount needed
    ResearchMaterial,
    /// Activity of researching time to reduce the time needed to manufacture
    ResearchTime
}

impl From<BlueprintActivity> for i16 {
    fn from(x: BlueprintActivity) -> Self {
        match x {
            BlueprintActivity::Copy             => 0i16,
            BlueprintActivity::Invention        => 1i16,
            BlueprintActivity::Manufacture      => 2i16,
            BlueprintActivity::Reaction         => 3i16,
            BlueprintActivity::ResearchMaterial => 4i16,
            BlueprintActivity::ResearchTime     => 5i16,
        }
    }
}

impl fmt::Display for BlueprintActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Self::Copy             => "COPY",
            Self::Invention        => "INVENTION",
            Self::Manufacture      => "MANUFACTURE",
            Self::Reaction         => "REACTION",
            Self::ResearchMaterial => "RESEARCH_MATERIAL",
            Self::ResearchTime     => "RESEARCH_TIME"
        }
        .to_string();

        write!(f, "{}", x)
    }
}
