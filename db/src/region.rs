mod fetch;
mod insert;

pub use self::fetch::*;
pub use self::insert::*;

use cachem::Parse;
use std::collections::HashSet;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct RegionCache(RwLock<HashSet<u32>>);

impl RegionCache {
    pub const CAPACITY: usize = 50;
}

#[derive(Debug, PartialEq, Eq, Hash, Parse)]
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
