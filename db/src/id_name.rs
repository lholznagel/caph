mod fetch;
mod fetch_bulk;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::fetch_bulk::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use metrix_exporter::MetrixSender;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct IdNameCache {
    cache: RwLock<HashMap<u32, IdNameEntry>>,
    metrix: MetrixSender
}

impl IdNameCache {
    pub const CAPACITY: usize = 425_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[cfg_attr(feature = "with_serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Parse)]
pub struct IdNameEntry {
    pub item_id: u32,
    pub name:    String,
}

impl IdNameEntry {
    pub fn new(
        item_id: u32,
        name: String
    ) -> Self {
        Self {
            item_id,
            name,
        }
    }
}
