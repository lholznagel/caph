use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TypeMaterialService(pub HashMap<TypeId, TypeMaterialEntry>);

impl TypeMaterialService {
    const PATH: &'static str = "sde/fsd/typeMaterials.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeMaterialEntry {
    #[serde(rename = "materials")]
    pub materials: Vec<Material>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    #[serde(rename = "materialTypeID")]
    pub material_type_id: TypeId,
    #[serde(rename = "quantity")]
    pub quantity:         u32
}
