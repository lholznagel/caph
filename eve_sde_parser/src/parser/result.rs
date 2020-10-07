use crate::parser::*;

use eve_online_api::{Type, TypeId};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub struct ParserResult {
    pub(crate) materials: HashMap<TypeId, TypeMaterial>,
    pub(crate) type_data: HashMap<TypeId, TypeIdData>,
}

impl ParserResult {
    pub fn items(&self) -> Vec<Type> {
        let mut type_ids = BTreeSet::new();
        let mut items = Vec::with_capacity(self.type_data.len());

        for (k, v) in self.materials.clone() {
            type_ids.insert(k);

            for x in v.materials {
                type_ids.insert(x.material_type_id);
            }
        }

        for (k, v) in self.type_data.clone() {
            if type_ids.contains(&k) {
                items.push(Type {
                    description: v
                        .description
                        .map(|x| x.get("en".into()).unwrap_or(&String::new()).clone())
                        .unwrap_or_default()
                        .clone(),
                    group_id: v.group_id,
                    name: v.name.get("en".into()).unwrap().clone(),
                    published: v.published,
                    type_id: k,
                    mass: v.mass,
                    volume: v.volume,
                    ..Default::default()
                })
            }
        }

        items
    }
}
