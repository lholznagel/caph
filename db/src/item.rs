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

pub struct ItemCache {
    cache: RwLock<HashMap<u32, ItemEntry>>,
    metrix: MetrixSender,
}

impl ItemCache {
    pub const CAPACITY: usize = 40_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct ItemEntry {
    pub category_id: u32,
    pub group_id:    u32,
    pub item_id:     u32,
    pub volume:      f32,
    pub name:        String,
    pub description: String,
}

impl ItemEntry {
    pub fn new(
        category_id: u32,
        group_id:    u32,
        item_id:     u32,
        volume:      f32,
        name:        String,
        description: String,
    ) -> Self {
        Self {
            category_id,
            group_id,
            item_id,
            volume,
            name,
            description,
        }
    }
}
