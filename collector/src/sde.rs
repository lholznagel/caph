use crate::error::CollectorError;

use caph_eve_data_wrapper::{BlueprintMaterial, BlueprintSkill, EveDataWrapper};
use sqlx::{PgPool, Type};
use std::fmt;
use uuid::Uuid;

pub struct Sde {
    eve:  EveDataWrapper,
    pool: PgPool
}

impl Sde {
    pub fn new(eve: EveDataWrapper, pool: PgPool) -> Self {
        Self { eve, pool}
    }

    pub async fn run(&mut self) -> Result<(), CollectorError> {
        self.save_blueprints(&self.eve).await?;
        self.save_items(&self.eve).await?;
        self.save_reprocessing_info(&self.eve).await?;
        self.save_schematics(&self.eve).await?;

        Ok(())
    }

    /// Extractes all items and inserts them into the database.
    async fn save_items(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let item_service  = sde.types().await?;
        let group_service = sde.groups().await?;

        let mut type_ids = Vec::new();
        let mut categories = Vec::new();
        let mut group_ids = Vec::new();
        let mut volumes = Vec::new();
        let mut names = Vec::new();

        // Collect all items together
        for (tid, entry) in item_service.types() {
            let category = *group_service.groups()
                .get(&entry.group_id)
                .map(|x| x.category_id)
                .unwrap() as i32;
            let group_id = *entry.group_id as i32;
            let name = entry.name().unwrap_or_default();
            let volume = entry.volume.unwrap_or(0f32);

            type_ids.push(**tid as i32);
            categories.push(category);
            group_ids.push(group_id);
            names.push(name);
            volumes.push(volume);
        }

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

        Ok(())
    }

    /// Collect all item materials together and save them in the database.
    async fn save_reprocessing_info(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let type_service = sde.types().await?;

        let mut type_ids = Vec::new();
        let mut material_ids = Vec::new();
        let mut quantities = Vec::new();

        // Collect all items together
        for (tid, materials) in type_service.materials() {
            for material in materials.materials.iter() {
                let material_id = material.material_type_id;
                let quantity = material.quantity;

                type_ids.push(**tid as i32);
                material_ids.push(*material_id as i32);
                quantities.push(quantity as i32);
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

    async fn save_blueprints(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let blueprint_service = sde.blueprints().await?;
        let count = blueprint_service.blueprints().len();

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
            id: Uuid,
            materials: Vec<BlueprintMaterial>,
            product: bool,
            activity: BlueprintActivity| {
            for material in materials {
                bm_blueprint_id.push(id.clone());
                bm_type_id.push(*material.type_id as i32);
                bm_quantity.push(material.quantity as i32);
                bm_is_product.push(product);
                bm_probability.push(material.probability);
                bm_activity.push(activity.into());
            }
        };
        let mut insert_skill = |
            id: Uuid,
            skills: Vec<BlueprintSkill>,
            activity: BlueprintActivity| {

            for skill in skills {
                bs_blueprint_id.push(id.clone());
                bs_type_id.push(*skill.type_id as i32);
                bs_level.push(skill.level as i32);
                bs_activity.push(activity.into());
            }
        };

        for (bid, entry) in blueprint_service.blueprints() {
            let id = Uuid::new_v4();

            ids.push(id);
            type_id.push(**bid as i32);
            limit.push(entry.max_production_limit as i32);

            if let Some(x) = entry.activities.copying.clone() {
                copy.push(Some(x.time as i32));
                insert_mat(id, x.materials.unwrap_or_default(), false, BlueprintActivity::Copy);
                insert_mat(id, x.products.unwrap_or_default(), true, BlueprintActivity::Copy);
                insert_skill(id, x.skills.unwrap_or_default(), BlueprintActivity::Copy);
            } else {
                copy.push(None);
            }
            if let Some(x) = entry.activities.invention.clone() {
                invention.push(Some(x.time as i32));
                insert_mat(id, x.materials.unwrap_or_default(), false, BlueprintActivity::Invention);
                insert_mat(id, x.products.unwrap_or_default(), true, BlueprintActivity::Invention);
                insert_skill(id, x.skills.unwrap_or_default(), BlueprintActivity::Invention);
            } else {
                invention.push(None);
            }
            if let Some(x) = entry.activities.manufacturing.clone() {
                let materials = x.materials.unwrap_or_default();
                let products = x.products.unwrap_or_default();

                if materials.len() == 1 && products.len() == 1 && materials[0].type_id == products[0].type_id {
                    continue
                }

                manufacture.push(Some(x.time as i32));
                insert_mat(id, materials, false, BlueprintActivity::Manufacture);
                insert_mat(id, products, true, BlueprintActivity::Manufacture);
                insert_skill(id, x.skills.unwrap_or_default(), BlueprintActivity::Manufacture);
            } else {
                manufacture.push(None);
            }
            if let Some(x) = entry.activities.reaction.clone() {
                reaction.push(Some(x.time as i32));
                insert_mat(id, x.materials.unwrap_or_default(), false, BlueprintActivity::Reaction);
                insert_mat(id, x.products.unwrap_or_default(), true, BlueprintActivity::Reaction);
                insert_skill(id, x.skills.unwrap_or_default(), BlueprintActivity::Reaction);
            } else {
                reaction.push(None);
            }
            if let Some(x) = entry.activities.research_material.clone() {
                research_mat.push(Some(x.time as i32));
                insert_mat(id, x.materials.unwrap_or_default(), false, BlueprintActivity::ResearchMaterial);
                insert_mat(id, x.products.unwrap_or_default(), true, BlueprintActivity::ResearchMaterial);
                insert_skill(id, x.skills.unwrap_or_default(), BlueprintActivity::ResearchMaterial);
            } else {
                research_mat.push(None);
            }
            if let Some(x) = entry.activities.research_time.clone() {
                research_time.push(Some(x.time as i32));
                insert_mat(
                    id,
                    x.materials.unwrap_or_default(),
                    false,
                    BlueprintActivity::ResearchTime);
                insert_mat(
                    id,
                    x.products.unwrap_or_default(),
                    true,
                    BlueprintActivity::ResearchTime
                );
                insert_skill(id, x.skills.unwrap_or_default(), BlueprintActivity::ResearchTime);
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

    async fn save_schematics(&self, sde: &EveDataWrapper) -> Result<(), CollectorError> {
        let schematic_service = sde.planet_schematics().await?;

        let mut s_ids = Vec::new();
        let mut s_type_ids = Vec::new();
        let mut s_cycle_times = Vec::new();

        let mut sm_schematic = Vec::new();
        let mut sm_type_id = Vec::new();
        let mut sm_is_input = Vec::new();
        let mut sm_quantity = Vec::new();

        for (type_id, entry) in schematic_service.schematics() {
            let id = Uuid::new_v4();

            s_ids.push(id);
            s_type_ids.push(**type_id as i32);
            s_cycle_times.push(entry.cycle_time as i32);

            for (type_id, entry) in entry.types.clone() {
                sm_schematic.push(id);
                sm_type_id.push(*type_id as i32);
                sm_is_input.push(entry.is_input);
                sm_quantity.push(entry.quantity as i32);
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
