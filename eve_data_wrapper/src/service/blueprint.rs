use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BlueprintService {
    blueprints: HashMap<TypeId, BlueprintEntry>,
}

impl BlueprintService {
    const PATH: &'static str = "sde/fsd/blueprints.yaml";

    pub(crate) fn new(mut zip: SdeZipArchive) -> Result<Self, EveConnectError> {
        Ok(Self {
            blueprints: crate::parse_zip_file(Self::PATH, &mut zip)?,
        })
    }

    pub fn blueprints(&self) -> &HashMap<TypeId, BlueprintEntry> {
        &self.blueprints
    }

    pub fn graph<T: Into<TypeId> + Copy>(&self, bp_id: T) -> Option<BlueprintGraph> {
        let product = self.product_manufacturing(bp_id)?;
        let product = product.get(0)?;
        let materials = self.materials_manufacturing(bp_id)?;

        let mut children = Vec::new();
        for material in materials {
            let entry = self.graph(material.type_id)?;
            children.push(entry);
        }

        let root = BlueprintGraph {
            type_id:  product.type_id,
            quantity: product.quantity,
            children,
        };

        Some(root)
    }

    pub fn product_manufacturing<T: Into<TypeId>>(
        &self,
        bp_id: T
    ) -> Option<Vec<BlueprintMaterial>> {
        let bp = self.blueprints.get(&bp_id.into())?;
        bp.activities
            .clone()
            .manufacturing?
            .products
    }

    pub fn materials_manufacturing<T: Into<TypeId>>(
        &self,
        bp_id: T
    ) -> Option<Vec<BlueprintMaterial>> {
        let bp = self.blueprints.get(&bp_id.into())?;
        bp.activities
            .clone()
            .manufacturing?
            .materials
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintEntry {
    #[serde(rename = "activities")]
    pub activities:           BlueprintActivity,
    #[serde(rename = "blueprintTypeID")]
    pub type_id:              TypeId,
    #[serde(rename = "maxProductionLimit")]
    pub max_production_limit: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintActivity {
    #[serde(rename = "copying")]
    pub copying:           Option<BlueprintAdditional>,
    #[serde(rename = "invention")]
    pub invention:         Option<BlueprintAdditional>,
    #[serde(rename = "manufacturing")]
    pub manufacturing:     Option<BlueprintAdditional>,
    #[serde(rename = "reaction")]
    pub reaction:          Option<BlueprintAdditional>,
    #[serde(rename = "research_material")]
    pub research_material: Option<BlueprintAdditional>,
    #[serde(rename = "research_time")]
    pub research_time:     Option<BlueprintAdditional>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintAdditional {
    #[serde(rename = "materials")]
    pub materials: Option<Vec<BlueprintMaterial>>,
    #[serde(rename = "products")]
    pub products:  Option<Vec<BlueprintMaterial>>,
    #[serde(rename = "skills")]
    pub skills:    Option<Vec<BlueprintSkill>>,
    #[serde(rename = "time")]
    pub time:      u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintMaterial {
    #[serde(rename = "quantity")]
    pub quantity:    u32,
    #[serde(rename = "typeID")]
    pub type_id:     TypeId,

    #[serde(rename = "probability")]
    pub probability: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintSkill {
    #[serde(rename = "level")]
    pub level:   u32,
    #[serde(rename = "typeID")]
    pub type_id: TypeId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlueprintGraph {
    #[serde(rename = "typeId")]
    pub type_id:  TypeId,
    pub quantity: u32,
    pub children: Vec<BlueprintGraph>,
}
