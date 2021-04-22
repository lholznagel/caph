use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DogmaAttributeEntry {
    #[serde(rename = "attributeID")]
    pub attribute_id:     AttributeId,
    #[serde(rename = "dataType")]
    pub data_type:        u32,
    #[serde(rename = "defaultValue")]
    pub default_value:    f32,
    #[serde(rename = "description")]
    #[serde(default)]
    pub description:      String,
    #[serde(rename = "displayNameID")]
    #[serde(default)]
    pub display_name:     HashMap<String, String>,
    #[serde(rename = "highIsGood")]
    pub high_is_good:     bool,
    #[serde(rename = "name")]
    pub name:             String,
    #[serde(rename = "published")]
    pub published:        bool,
    #[serde(rename = "stackable")]
    pub stackable:        bool,
    #[serde(rename = "tooltipDescriptionID")]
    #[serde(default)]
    pub tooltip:          HashMap<String, String>,
    #[serde(rename = "tooltipTitleID")]
    #[serde(default)]
    pub tooltip_title:    HashMap<String, String>,

    #[serde(rename = "categoryID")]
    pub category_id:      Option<CategoryId>,
    #[serde(rename = "chargeRechargeTimeID")]
    pub recharge_time_id: Option<u32>,
    #[serde(rename = "iconID")]
    pub icon_id:          Option<IconId>,
    #[serde(rename = "maxAttributeID")]
    pub max_attribute_id: Option<AttributeId>,
    #[serde(rename = "unitID")]
    pub unit_id:          Option<UnitId>,
}
