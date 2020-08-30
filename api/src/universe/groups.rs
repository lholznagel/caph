use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub category_id: u32,
    pub group_id: GroupId,
    pub name: String,
    pub published: bool,
    pub types: Vec<TypeId>,
}

impl EveClient {
    fetch!(fetch_group, "universe/groups", GroupId, Group);
}
