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

pub struct ItemMaterialCache {
    cache: RwLock<HashMap<u32, Vec<ItemMaterialEntry>>>,
    metrix: MetrixSender,
}

impl ItemMaterialCache {
    pub const CAPACITY: usize = 45_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Parse)]
pub struct ItemMaterialEntry {
    pub item_id:     u32,
    pub material_id: u32,
    pub quantity:    u32,
}

impl ItemMaterialEntry {
    pub fn new(
        item_id: u32,
        material_id: u32,
        quantity: u32,
    ) -> Self {
        Self {
            item_id,
            material_id,
            quantity,
        }
    }
}
