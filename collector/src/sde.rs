use crate::error::CollectorError;

use caph_connector::{BlueprintMaterial, BlueprintSkill, ConnectAssetService, ConnectBlueprintService, ConnectReprocessService, ConnectSchematicService, SdeService};
use sqlx::{PgPool, Type};
use std::fmt;
use uuid::Uuid;

pub struct Sde {
    pool: PgPool,
}

impl Sde {
    pub fn new(pool: PgPool) -> Self {
        Self { pool}
    }

    pub async fn run(&mut self) -> Result<(), CollectorError> {
        let mut zip = SdeService::new()
            .await
            .map_err(CollectorError::LoadingZipError)?;

        let asset_service = ConnectAssetService::new(&mut zip)
            .map_err(CollectorError::GeneralConnectError)?;
        let blueprint_service = ConnectBlueprintService::new(&mut zip)
            .map_err(CollectorError::GeneralConnectError)?;
        let reprocess_service = ConnectReprocessService::new(&mut zip)
            .map_err(CollectorError::GeneralConnectError)?;
        let schematic_service = ConnectSchematicService::new(&mut zip)
            .map_err(CollectorError::GeneralConnectError)?;

        self.save_blueprints(&blueprint_service).await?;
        self.save_assets(&asset_service).await?;
        self.save_reprocessing_info(&reprocess_service).await?;
        self.save_schematics(&schematic_service).await?;

        Ok(())
    }

    /// Extractes all items and inserts them into the database.
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
                .unwrap();
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
        let mut trans = self.pool.begin().await?;
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
            .await?;
        trans.commit().await?;
        tracing::debug!(task = "asset", "Finished inserting assets in DB");

        Ok(())
    }

    /// Collect all item materials together and save them in the database.
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

        let mut trans = self.pool.begin().await?;
        sqlx::query!("DELETE FROM reprocess").execute(&mut trans).await?;
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
            .await?;
        trans.commit().await?;

        Ok(())
    }

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

        let mut trans = self.pool.begin().await?;
        sqlx::query!("DELETE FROM blueprint CASCADE").execute(&mut trans).await?;
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
            .await?;

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
            .await?;
        trans.commit().await?;

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
            .execute(&self.pool)
            .await?;

        Ok(())
    }

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

        let mut trans = self.pool.begin().await?;
        sqlx::query!("DELETE FROM schematic CASCADE").execute(&mut trans).await?;
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
            .await?;

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
            .await?;
        trans.commit().await?;

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Type)]
pub enum BlueprintActivity {
    Copy,
    Invention,
    Manufacture,
    Reaction,
    ResearchMaterial,
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
            Self::Copy => "COPY",
            Self::Invention => "INVENTION",
            Self::Manufacture => "MANUFACTURE",
            Self::Reaction => "REACTION",
            Self::ResearchMaterial => "RESEARCH_MATERIAL",
            Self::ResearchTime => "RESEARCH_TIME"
        }
        .to_string();

        write!(f, "{}", x)
    }
}
