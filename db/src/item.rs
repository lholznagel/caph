mod fetch;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct ItemCache(RwLock<HashMap<u32, ItemEntry>>);

impl ItemCache {
    pub const CAPACITY: usize = 40_000;
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
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
