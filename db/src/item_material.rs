mod insert;

pub use self::insert::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct ItemMaterialCache(RwLock<HashMap<u32, ItemMaterialEntry>>);

impl ItemMaterialCache {
    pub const CAPACITY: usize = 45_000;
}

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
