use crate::error::*;
use crate::*;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Blueprint {
    pub type_id: TypeId,
    pub materials: Vec<Material>,
    pub produces: Option<Material>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Material {
    pub quantity: u32,
    #[serde(alias = "typeID")]
    pub type_id: TypeId
}

impl Blueprint {
    pub fn parse() -> Result<Vec<Blueprint>> {
        Ok(BlueprintYamlParser::parse().unwrap())
    }
}

#[derive(Debug, Deserialize)]
pub struct BlueprintYamlParser {
    activities: Vec<serde_yaml::Value>,
    #[serde(alias = "blueprintTypeID")]
    blueprint_type_id: TypeId,
    #[serde(alias = "maxProductionLimit")]
    max_production_limit: u32
}

#[derive(Debug, Deserialize)]
pub struct TimeModel {
    time: u32
}

#[derive(Debug, Deserialize)]
pub struct ActivityManufactoring {
    #[serde(default)]
    materials: Vec<Material>,
    #[serde(default)]
    products: Vec<Material>,
    #[serde(default)]
    time: u32
}

impl BlueprintYamlParser {
    pub fn parse() -> Result<Vec<Blueprint>> {
        let mut blueprints = File::open("./database/blueprints.yaml").unwrap();
        let mut data = Vec::new();
        blueprints.read_to_end(&mut data).unwrap();

        let parsed: HashMap<TypeId, serde_yaml::Value> = serde_yaml::from_slice(&data).unwrap();

        let mut entries = Vec::new();

        for (type_id, value) in parsed {
            let map = value.as_mapping().unwrap();
            for (key, value) in map {
                let mut blueprint = Blueprint { type_id: TypeId(0), materials: Vec::new(), produces: None };
                blueprint.type_id = type_id;

                if key.as_str().unwrap_or_default() == "activities" {
                    let map = value.as_mapping().unwrap();

                    for (key, value) in map {
                        if key.as_str().unwrap() == "manufacturing" {
                            let manufactoring: ActivityManufactoring = serde_yaml::from_value(value.clone()).unwrap();
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

        Ok(entries)
    }
}