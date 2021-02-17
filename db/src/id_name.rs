mod insert;

pub use self::insert::*;

use cachem::Parse;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct IdNameCache(RwLock<HashMap<u32, IdNameEntry>>);

impl IdNameCache {
    pub const CAPACITY: usize = 425_000;
}

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
