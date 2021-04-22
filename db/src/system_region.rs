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

pub struct SystemRegionCache {
    cache: RwLock<HashMap<u32, SystemRegionEntry>>,
    metrix: MetrixSender,
}

impl SystemRegionCache {
    pub const CAPACITY: usize = 6_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct SystemRegionEntry {
    pub region_id:  u32,
    pub system_id:  u32,
    pub security:   f32,
}

impl SystemRegionEntry {
    pub fn new(
        region_id:  u32,
        system_id:  u32,
        security:   f32,
    ) -> Self {

        Self {
            region_id,
            system_id,
            security,
        }
    }
}
