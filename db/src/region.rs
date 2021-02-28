mod fetch;
mod insert;
mod storage;

pub use self::fetch::*;
pub use self::insert::*;
pub use self::storage::*;

use cachem::Parse;
use metrix_exporter::MetrixSender;
use std::collections::HashSet;
use tokio::sync::RwLock;

pub struct RegionCache {
    cache: RwLock<HashSet<u32>>,
    metrix: MetrixSender,
}

impl RegionCache {
    pub const CAPACITY: usize = 50;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashSet::new()),
            metrix,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Parse)]
pub struct RegionEntry {
    pub region_id: u32,
}

impl RegionEntry {
    pub fn new(
        region_id: u32,
    ) -> Self {

        Self {
            region_id
        }
    }
}

#[derive(Debug, Parse)]
pub struct RegionEntries(pub HashSet<u32>);
