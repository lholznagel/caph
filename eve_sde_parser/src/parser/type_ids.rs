use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypeIds {
    #[serde(rename = "groupID")]
    pub group_id: u32,
    pub name: HashMap<String, String>,
    #[serde(rename = "portionSize")]
    pub portion_size: usize,
    pub published: bool,

    #[serde(rename = "basePrice")]
    pub base_price: Option<f64>,
    pub capacity: Option<f64>,
    pub description: Option<HashMap<String, String>>,
    #[serde(rename = "sofFactionName")]
    pub faction_name: Option<String>,
    #[serde(rename = "factionID")]
    pub faction_id: Option<u32>,
    #[serde(rename = "graphicID")]
    pub graphic_id: Option<u32>,
    #[serde(rename = "iconID")]
    pub icon_id: Option<u32>,
    #[serde(rename = "marketGroupID")]
    pub market_group_id: Option<u32>,
    pub mass: Option<f32>,
    pub masteries: Option<HashMap<u16, Vec<u16>>>,
    #[serde(rename = "sofMaterialSetID")]
    pub material_set_id: Option<u32>,
    #[serde(rename = "metaGroupID")]
    pub meta_group_id: Option<u32>,
    #[serde(rename = "raceID")]
    pub race_id: Option<u32>,
    pub radius: Option<f32>,
    #[serde(rename = "soundID")]
    pub sound_id: Option<u32>,
    pub traits: Option<Trait>,
    pub volume: Option<f32>,
    #[serde(rename = "variationParentTypeID")]
    pub variation_parent_type_id: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Trait {
    #[serde(rename = "miscBonuses")]
    pub misc_bonuses: Option<Vec<Bonus>>,
    #[serde(rename = "iconID")]
    pub icon_id: Option<u32>,
    #[serde(rename = "roleBonuses")]
    pub role_bonuses: Option<Vec<Bonus>>,
    pub types: Option<HashMap<usize, Vec<Bonus>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bonus {
    pub bonus: Option<f32>,
    #[serde(rename = "bonusText")]
    pub bonus_text: HashMap<String, String>,
    pub importance: usize,
    #[serde(rename = "unitID")]
    pub unit_id: Option<u32>,
    #[serde(rename = "isPositive")]
    pub is_positive: Option<bool>,
}
