use crate::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TypeIdService(pub HashMap<TypeId, TypeIdEntry>);

impl TypeIdService {
    const PATH: &'static str = "sde/fsd/typeIDs.yaml";

    service_gen!();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeIdEntry {
    #[serde(rename = "groupID")]
    pub group_id:                 GroupId,
    #[serde(rename = "description")]
    #[serde(default)]
    pub description:              HashMap<String, String>,
    #[serde(rename = "name")]
    pub name:                     HashMap<String, String>,
    #[serde(rename = "masteries")]
    #[serde(default)]
    pub masteries:                HashMap<u16, Vec<u16>>,
    #[serde(rename = "portionSize")]
    pub portion_size:             usize,
    #[serde(rename = "published")]
    pub published:                bool,

    #[serde(rename = "basePrice")]
    pub base_price:               Option<f64>,
    #[serde(rename = "capacity")]
    pub capacity:                 Option<f64>,
    #[serde(rename = "sofFactionName")]
    pub faction_name:             Option<String>,
    #[serde(rename = "factionID")]
    pub faction_id:               Option<FactionId>,
    #[serde(rename = "graphicID")]
    pub graphic_id:               Option<GraphicId>,
    #[serde(rename = "iconID")]
    pub icon_id:                  Option<IconId>,
    #[serde(rename = "marketGroupID")]
    pub market_group_id:          Option<MarketGroupId>,
    #[serde(rename = "mass")]
    pub mass:                     Option<f32>,
    #[serde(rename = "sofMaterialSetID")]
    pub material_set_id:          Option<u32>,
    #[serde(rename = "metaGroupID")]
    pub meta_group_id:            Option<MetaGroupId>,
    #[serde(rename = "raceID")]
    pub race_id:                  Option<RaceId>,
    #[serde(rename = "radius")]
    pub radius:                   Option<f32>,
    #[serde(rename = "soundID")]
    pub sound_id:                 Option<SoundId>,
    #[serde(rename = "traits")]
    pub traits:                   Option<Trait>,
    #[serde(rename = "volume")]
    pub volume:                   Option<f32>,
    #[serde(rename = "variationParentTypeID")]
    pub variation_parent_type_id: Option<u32>,
}

impl TypeIdEntry {
    pub fn description(&self) -> Option<String> {
        self
            .description
            .get("en")
            .map(|x| x.clone())
    }

    pub fn name(&self) -> Option<String> {
        self
            .name
            .get("en")
            .map(|x| x.clone())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Trait {
    #[serde(rename = "miscBonuses")]
    pub misc_bonuses: Option<Vec<Bonus>>,
    #[serde(rename = "iconID")]
    pub icon_id :     Option<IconId>,
    #[serde(rename = "roleBonuses")]
    pub role_bonuses: Option<Vec<Bonus>>,
    #[serde(rename = "types")]
    #[serde(default)]
    pub types:        HashMap<usize, Vec<Bonus>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bonus {
    #[serde(rename = "bonus")]
    pub bonus:       Option<f32>,
    #[serde(rename = "bonusText")]
    pub bonus_text:  HashMap<String, String>,
    #[serde(rename = "importance")]
    pub importance:  usize,
    #[serde(rename = "unitID")]
    pub unit_id:     Option<UnitId>,
    #[serde(rename = "isPositive")]
    pub is_positive: Option<bool>,
}
