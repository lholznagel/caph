use crate::error::*;
use crate::*;

#[derive(Clone, Debug)]
pub struct BlueprintRessCalcResult {
    pub name: String,
    pub count: u32,
}

pub struct BlueprintResourceCalc {
    database: Database,
}

impl BlueprintResourceCalc {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn collect(
        &self,
        blueprint_name: String,
        count: u16,
    ) -> Result<Vec<BlueprintRessCalcResult>> {
        let type_data = self
            .database
            .type_data
            .clone()
            .into_iter()
            .find(|x| x.name == blueprint_name);

        if let Some(x) = type_data {
            let blueprint = self
                .database
                .blueprints
                .clone()
                .into_iter()
                .find(|y| y.type_id == x.type_id)
                .unwrap();

            let mut blueprint_materials = Vec::new();
            for material in blueprint.materials {
                let name = self
                    .database
                    .type_data
                    .clone()
                    .into_iter()
                    .find(|x| x.type_id == material.type_id)
                    .map(|x| x.name)
                    .unwrap();

                blueprint_materials.push(BlueprintRessCalcResult {
                    name,
                    count: material.quantity * count as u32,
                })
            }

            Ok(blueprint_materials)
        } else {
            Ok(Vec::new())
        }
    }
}
