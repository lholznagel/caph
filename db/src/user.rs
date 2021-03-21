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

type UserId = u32;

pub struct UserCache {
    cache: RwLock<HashMap<UserId, UserEntry>>,
    metrix: MetrixSender,
}

impl UserCache {
    pub fn new(metrix: MetrixSender) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            metrix,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Parse)]
pub struct UserEntry {
    pub user_id:       u32,
    pub name:          String,
    pub access_token:  String,
    pub refresh_token: String,
    pub aliase:        Vec<u32>,
}
