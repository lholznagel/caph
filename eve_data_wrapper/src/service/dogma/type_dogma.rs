use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDogmaEntry {
    #[serde(rename = "dogmaAttributes")]
    pub attributes: Vec<DogmaAttribute>,
    #[serde(rename = "dogmaEffects")]
    pub effects: Vec<DogmaEffect>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DogmaAttribute {
    #[serde(rename = "attributeID")]
    pub attribute_id: AttributeId,
    #[serde(rename = "value")]
    pub value:        f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DogmaEffect {
    #[serde(rename = "effectID")]
    pub effect_id: EffectId,
    #[serde(rename = "isDefault")]
    pub is_default:   bool,
}
