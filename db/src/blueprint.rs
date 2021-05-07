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
    cache: RwLock<HashMap<u32, BlueprintEntry>>,
    metrix: MetrixSender,
}

impl BlueprintCache {
    pub const CAPACITY: usize = 100_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Parse)]
pub struct BlueprintEntry {
    pub activity:  Activity,
    pub item_id:   u32,
    pub time:      u32,
    pub materials: Vec<Material>,
    pub skills:    Vec<Skill>,
}

impl BlueprintEntry {
    pub fn new(
        activity:  Activity,
        item_id:   u32,
        time:      u32,
        materials: Vec<Material>,
        skills:    Vec<Skill>,
    ) -> Self {
        Self {
            activity,
            item_id,
            time,
            materials,
            skills,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Parse)]
pub struct Material {
    pub material_id: u32,
    pub quantity:    u32,
    pub is_product:  bool,
}

impl Material {
    pub fn new(
        material_id: u32,
        quantity: u32,
        is_product: bool,
    ) -> Self {
        Self {
            material_id,
            quantity,
            is_product,
        }
    }
}

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

#[derive(Clone, Debug, PartialEq, Parse)]
pub enum Activity {
    Manufacturing,
    Reaction,
    PlanetInteraction,
}
