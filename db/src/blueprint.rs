mod fetch;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use metrix_exporter::MetrixSender;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct BlueprintCache{
    blueprints: RwLock<HashMap<u32, BlueprintEntry>>,
    schematics: RwLock<HashMap<u32, PlanetSchematicEntry>>,
    metrix: MetrixSender,
}

impl BlueprintCache {
    pub const CAPACITY: usize = 100_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            blueprints: RwLock::new(HashMap::new()),
            schematics: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

// TODO: Add copy, research_time, research_material, invention, manufacturing, reaction
#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct BlueprintEntry {
    pub activity:  Activity,
    // blueprint id
    pub bid:       u32,
    pub time:      u32,
    pub product:   Material,
    pub materials: Vec<Material>,
    pub skills:    Vec<Skill>,
}

impl BlueprintEntry {
    pub fn new(
        activity:  Activity,
        bid:       u32,
        time:      u32,
        product:   Material,
        materials: Vec<Material>,
        skills:    Vec<Skill>,
    ) -> Self {
        Self {
            activity,
            bid,
            time,
            product,
            materials,
            skills,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct Material {
    pub material_id: u32,
    pub quantity:    u32,
}

impl Material {
    pub fn new(
        material_id: u32,
        quantity: u32,
    ) -> Self {
        Self {
            material_id,
            quantity,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct PlanetSchematicEntry {
    // planet schemtic id
    pub psid:      u32,
    pub time:      u32,
    pub output:    Material,
    pub inputs:    Vec<Material>,
}

impl PlanetSchematicEntry {
    pub fn new(
        psid:   u32,
        time:   u32,
        output: Material,
        inputs: Vec<Material>,
    ) -> Self {
        Self {
            psid,
            time,
            output,
            inputs,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct Skill {
    pub level:   u8,
    pub type_id: u32,
}

impl Skill {
    pub fn new(
        level:   u8,
        type_id: u32,
    ) -> Self {
        Self {
            level,
            type_id,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub enum Activity {
    Manufacturing,
    Reaction,
    PlanetInteraction,
}
