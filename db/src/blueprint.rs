mod insert;
mod storage;

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
    pub item_id:   u32,
    pub time:      u32,
    pub materials: Vec<Material>,
}

impl BlueprintEntry {
    pub fn new(
        item_id: u32,
        time: u32,
        materials: Vec<Material>,
    ) -> Self {
        Self {
            item_id,
            time,
            materials,
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
