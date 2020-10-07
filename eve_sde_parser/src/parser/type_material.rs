use eve_online_api::TypeId;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TypeMaterial {
    pub materials: Vec<Material>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Material {
    #[serde(rename = "materialTypeID")]
    pub material_type_id: TypeId,
    pub quantity: usize,
}
