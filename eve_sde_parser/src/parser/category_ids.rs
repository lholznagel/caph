use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CategoryIds {
    pub name: HashMap<String, String>,
    pub published: bool,
}
