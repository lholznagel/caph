use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DogmaEffectEntry {
    #[serde(rename = "descriptionID")]
    #[serde(default)]
    pub description:                   HashMap<String, String>,
    #[serde(rename = "disallowAutoRepeat")]
    pub disallow_auto_repeat:          bool,
    #[serde(rename = "effectCategory")]
    pub effect_category:               u32,
    #[serde(rename = "displayID")]
    #[serde(default)]
    pub display:                       HashMap<String, String>,
    #[serde(rename = "displayNameID")]
    #[serde(default)]
    pub name:                          HashMap<String, String>,
    #[serde(rename = "effectID")]
    pub effect_id:                     EffectId,
    #[serde(rename = "effectName")]
    pub effect_name:                   String,
    #[serde(rename = "electronicChance")]
    pub electronic_chance:             bool,
    #[serde(rename = "isAssistance")]
    pub is_assistance:                 bool,
    #[serde(rename = "isOffensive")]
    pub is_offensive:                  bool,
    #[serde(rename = "isWarpSafe")]
    pub is_warp_safe:                  bool,
    #[serde(rename = "propulsionChance")]
    pub propulsion_chance:             bool,
    #[serde(rename = "published")]
    pub published:                     bool,
    #[serde(rename = "rangeChance")]
    pub range_chance:                  bool,

    #[serde(rename = "dischargeAttributeID")]
    pub discharge_attribute_id:        Option<AttributeId>,
    #[serde(rename = "distribution")]
    pub distribution:                  Option<u32>,
    #[serde(rename = "durationAttributeID")]
    pub duration_attribute_id:         Option<AttributeId>,
    #[serde(rename = "falloffAttributeID")]
    pub falloff_attribute_id:          Option<AttributeId>,
    #[serde(rename = "fittingUsageChanceAttributeID")]
    pub fitting_usage_chance_attr_id:  Option<AttributeId>,
    #[serde(rename = "guid")]
    pub guid:                          Option<String>,
    #[serde(rename = "iconID")]
    pub icon_id:                       Option<IconId>,
    #[serde(rename = "modifierInfo")]
    pub modifier_info:                 Option<Vec<ModifierInfo>>,
    #[serde(rename = "npcUsageChanceAttributeID")]
    pub npc_usage_chance_attr_id:      Option<AttributeId>,
    #[serde(rename = "npcActivationChanceAttributeID")]
    pub npc_activation_chance_attr_id: Option<AttributeId>,
    #[serde(rename = "rangeAttributeID")]
    pub range_attribute_id:            Option<AttributeId>,
    #[serde(rename = "resistanceAttributeID")]
    pub resistance_attr_id:            Option<AttributeId>,
    #[serde(rename = "sfxName")]
    pub sfx_name:                      Option<String>,
    #[serde(rename = "trackingSpeedAttributeID")]
    pub tracking_speed_attribute_id:   Option<AttributeId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModifierInfo {
    #[serde(rename = "domain")]
    pub domain:                 String,
    #[serde(rename = "func")]
    pub func:                   String,

    #[serde(rename = "effectID")]
    pub effect_id:              Option<u32>,
    #[serde(rename = "groupID")]
    pub groupd_id:              Option<GroupId>,
    #[serde(rename = "modifiedAttributeID")]
    pub modified_attribute_id:  Option<AttributeId>,
    #[serde(rename = "modifyingAttributeID")]
    pub modifying_attribute_id: Option<AttributeId>,
    #[serde(rename = "operation")]
    pub operation:              Option<i8>,
    #[serde(rename = "skillTypeID")]
    pub skill_type_id:          Option<TypeId>
}
