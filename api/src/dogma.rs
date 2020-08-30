use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attribute {
    pub attribute_id: AttributeId,

    pub default_value: Option<f32>,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub high_is_good: Option<bool>,
    pub icon_id: Option<u32>,
    pub name: Option<String>,
    pub published: Option<bool>,
    pub stackable: Option<bool>,
    pub unit_id: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Effect {
    pub effect_id: EffectId,

    pub description: Option<String>,
    pub disallow_auto_repeat: Option<bool>,
    pub discharage_attribute_id: Option<AttributeId>,
    pub display_name: Option<String>,
    pub duration_attribute_id: Option<AttributeId>,
    pub effect_category: Option<u32>,
    pub electronic_chance: Option<bool>,
    pub falloff_attribute_id: Option<AttributeId>,
    pub icon_id: Option<u32>,
    pub is_assistance: Option<bool>,
    pub is_offensive: Option<bool>,
    pub is_warp_save: Option<bool>,
    pub modifiers: Option<Vec<EffectModifier>>,
    pub name: Option<String>,
    pub post_expression: Option<u32>,
    pub pre_expression: Option<u32>,
    pub published: Option<bool>,
    pub range_attribute_id: Option<AttributeId>,
    pub range_chance: Option<bool>,
    pub tracking_speed_attribute_id: Option<AttributeId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EffectModifier {
    pub func: String,

    pub domain: Option<String>,
    pub effect_id: Option<EffectId>,
    pub modifier_attribute_id: Option<AttributeId>,
    pub modifying_attribute_id: Option<AttributeId>,
    pub operator: u32,
}

impl EveClient {
    fetch!(fetch_attribute, "dogma/attributes", AttributeId, Attribute);
    fetch!(fetch_effect, "dogma/effects", EffectId, Effect);
}
