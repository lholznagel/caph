mod insert;

pub use self::insert::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct StationCache(RwLock<HashMap<u32, StationEntry>>);

impl StationCache {
    pub const CAPACITY: usize = 6_000;
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
