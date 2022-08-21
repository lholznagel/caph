use caph_connector::{CategoryId, GroupId, TypeId};
use serde::Deserialize;

use crate::DependencyType;

/// FIXME: Purpose?
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseDependency {
    pub btype_id:       TypeId,
    pub blueprint_name: String,
    pub ptype_id:       TypeId,
    pub category_id:    CategoryId,
    pub group_id:       GroupId,
    pub product_name:   String,
    pub time:           u32,
    pub quantity:       u32,
    pub produces:       u32,
    pub typ:            DependencyType,
    pub components:     Vec<DatabaseDependency>,
}
