use eve_online_api::GroupId;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct TypeIdData {
    pub name: HashMap<String, String>,
    pub published: bool,
    #[serde(rename = "groupID")]
    pub group_id: GroupId,
    #[serde(rename = "portionSize")]
    pub portion_size: usize,

    pub description: Option<HashMap<String, String>>,
    pub graphic_id: Option<usize>,
    pub mass: Option<f32>,
    pub radius: Option<f32>,
    pub volume: Option<f32>,
    #[serde(rename = "soundID")]
    pub sound_id: Option<usize>,
}
