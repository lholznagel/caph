use eve_online_api::TypeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Blueprint {
    pub type_id: TypeId,
    pub materials: Vec<Material>,
    pub produces: Option<Material>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Material {
    pub quantity: u32,
    #[serde(alias = "typeID")]
    pub type_id: TypeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlueprintYamlParser {
    activities: Vec<serde_yaml::Value>,
    #[serde(alias = "blueprintTypeID")]
    blueprint_type_id: TypeId,
    #[serde(alias = "maxProductionLimit")]
    max_production_limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeModel {
    time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityManufactoring {
    #[serde(default)]
    materials: Vec<Material>,
    #[serde(default)]
    products: Vec<Material>,
    #[serde(default)]
    time: u32,
}

impl BlueprintYamlParser {
    pub fn parse(data: Vec<u8>) -> Vec<Blueprint> {
        let parsed: HashMap<TypeId, serde_yaml::Value> = serde_yaml::from_slice(&data).unwrap();

        let mut entries = Vec::new();

        for (type_id, value) in parsed {
            let map = value.as_mapping().unwrap();
            for (key, value) in map {
                let mut blueprint = Blueprint {
                    type_id: TypeId(0),
                    materials: Vec::new(),
                    produces: None,
                };
                blueprint.type_id = type_id;

                if key.as_str().unwrap_or_default() == "activities" {
                    let map = value.as_mapping().unwrap();

                    for (key, value) in map {
                        if key.as_str().unwrap() == "manufacturing" {
                            let manufactoring: ActivityManufactoring =
                                serde_yaml::from_value(value.clone()).unwrap();
                            blueprint.materials = manufactoring.materials;

                            if let Some(x) = manufactoring.products.get(0) {
                                blueprint.produces = Some(x.clone());
                            } else {
                                blueprint.produces = None;
                            }
                        }
                    }
                }

                entries.push(blueprint);
            }
        }

        entries
    }
}
