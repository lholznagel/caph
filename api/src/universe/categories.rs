use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub category_id: CategoryId,
    pub groups: Vec<GroupId>,
    pub name: String,
    pub published: bool,
}

impl EveClient {
    fetch!(fetch_category, "universe/categories", CategoryId, Category);
}
