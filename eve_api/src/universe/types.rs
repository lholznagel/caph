use crate::eve_client::*;
use crate::fetch;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Type {
    pub description: String,
    pub group_id: GroupId,
    pub name: String,
    pub published: bool,
    pub type_id: TypeId,

    pub capacity: Option<f32>,
    pub dogma_attributes: Option<Vec<DogmaAttributes>>,
    pub dogma_effects: Option<Vec<DogmaEffects>>,
    pub graphic_id: Option<GraphicId>,
    pub icon_id: Option<u32>,
    pub market_group_id: Option<u32>,
    pub mass: Option<f32>,
    pub package_volume: Option<f32>,
    pub portion_size: Option<u32>,
    pub radius: Option<f32>,
    pub volume: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DogmaAttributes {
    pub attribute_id: AttributeId,
    pub value: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DogmaEffects {
    pub effect_id: u32,
    pub is_default: bool,
}

impl EveClient {
    fetch!(fetch_type, "universe/types", TypeId, Type);
}
