use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypeMaterial {
    pub materials: Vec<Material>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
    #[serde(rename = "materialTypeID")]
    pub material_type_id: u32,
    pub quantity: usize,
}
