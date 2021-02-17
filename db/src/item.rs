mod insert;

pub use self::insert::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct ItemCache(RwLock<HashMap<u32, ItemEntry>>);

impl ItemCache {
    pub const CAPACITY: usize = 40_000;
}

#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct ItemEntry {
    pub item_id: u32,
    pub volume:  f32,
}

impl ItemEntry {
    pub fn new(
        item_id: u32,
        volume: f32,
    ) -> Self {
        Self {
            item_id,
            volume,
        }
    }
}
