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

pub struct StationCache {
    cache: RwLock<HashMap<u32, StationEntry>>,
    metrix: MetrixSender,
}

impl StationCache {
    pub const CAPACITY: usize = 6_000;

    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Parse)]
pub struct StationEntry {
    pub station_id: u32,
    pub region_id:  u32,
    pub system_id:  u32,
    pub security:   f32,
}

impl StationEntry {
    pub fn new(
        station_id: u32,
        region_id:  u32,
        system_id:  u32,
        security:   f32,
    ) -> Self {

        Self {
            station_id,
            region_id,
            system_id,
            security,
        }
    }
}
