use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, Serialize)]
struct MaterialNeeded {
    id: u32,
    quantity: u64,
    needed: u64,
}

#[derive(Clone)]
pub struct BlueprintService {
    db: Pool<Postgres>,
}

impl BlueprintService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn fetch_blueprints(&self) -> HashMap<u32, Vec<BlueprintResource>> {
        let mut conn = self.db.acquire().await.unwrap();

        let mut blueprints = HashMap::new();
        sqlx::query_as::<_, Blueprint>("SELECT blueprint_id FROM blueprints")
            .fetch_all(&mut conn)
            .await
            .unwrap()
            .into_iter()
            .for_each(|x| { blueprints.insert(x.blueprint_id as u32, Vec::new()); });

        sqlx::query_as::<_, BlueprintResource>("SELECT blueprint_id, material_id, quantity, is_product FROM blueprint_resources")
            .fetch_all(&mut conn)
            .await
            .unwrap()
            .into_iter()
            .for_each(|x| {
                blueprints
                    .entry(x.blueprint_id as u32)
                    .and_modify(|y| y.push(x));
            });

        blueprints
    }

    pub async fn fetch_schematics(&self) -> HashMap<u32, Vec<SchematicResource>> {
        let mut conn = self.db.acquire().await.unwrap();

        let mut schematics = HashMap::new();
        sqlx::query_as::<_, Schematic>("SELECT schematic_id FROM schematics")
            .fetch_all(&mut conn)
            .await
            .unwrap()
            .into_iter()
            .for_each(|x| { schematics.insert(x.schematic_id as u32, Vec::new()); });

        sqlx::query_as::<_, SchematicResource>("SELECT schematic_id, material_id, quantity, is_input FROM schematic_resources")
            .fetch_all(&mut conn)
            .await
            .unwrap()
            .into_iter()
            .for_each(|x| {
                schematics
                    .entry(x.schematic_id as u32)
                    .and_modify(|y| y.push(x));
            });

        schematics
    }

    pub async fn calc_bp_cost(&self, id: u32) -> HashMap<u32, u64> {
        let bps = self.fetch_blueprints().await;
        let schematics = self.fetch_schematics().await;

        let mut all_materials = HashMap::new();
        let mut materials = VecDeque::new();
        materials.push_front(MaterialNeeded {
            id,
            quantity: 1,
            needed: 1,
        });

        while !materials.is_empty() {
            let material = materials.pop_front().unwrap();

            if let Some(x) = self.production_materials(&bps, &schematics, material.clone()) {
                for x in x {
                    let mut x = x.clone();
                    x.needed *= material.needed;
                    materials.push_back(x.clone());
                }
            } else {
                all_materials
                    .entry(material.id)
                    .and_modify(|x| *x += material.quantity * material.needed)
                    .or_insert(material.quantity * material.needed);
            }
        }

        //all_materials
        HashMap::new()
    }

    // If there is a blueprint or schematic that produces the given id, the materials needed are returned
    fn production_materials(
        &self,
        bps: &HashMap<u32, Vec<BlueprintResource>>,
        schematics: &HashMap<u32, Vec<SchematicResource>>,
        requested_material: MaterialNeeded,
    ) -> Option<Vec<MaterialNeeded>> {
        for (id, resources) in schematics {
            if resources.iter().find(|x| !x.is_input && x.material_id as u32 == requested_material.id).is_some() {
                let mut needed_materials = Vec::new();
                for r in resources.iter().filter(|x| x.is_input) {
                    needed_materials.push(MaterialNeeded {
                        id: *id,
                        quantity: r.quantity as u64,
                        needed: requested_material.quantity,
                    });
                }

                return Some(needed_materials);
            }
        }

        for (id, resources) in bps {
            if resources.iter().find(|x| x.is_product && x.material_id as u32 == requested_material.id).is_some() {
                let mut needed_materials = Vec::new();
                for r in resources.iter().filter(|x| !x.is_product) {
                    needed_materials.push(MaterialNeeded {
                        id: *id,
                        quantity: r.quantity as u64,
                        needed: requested_material.quantity,
                    });
                }

                return Some(needed_materials);
            }
        }

        None
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Blueprint {
    pub blueprint_id: i32,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct BlueprintResource {
    pub blueprint_id: i32,
    pub material_id: i32,
    pub quantity: i32,
    pub is_product: bool
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Schematic {
    pub schematic_id: i32,
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct SchematicResource {
    pub schematic_id: i32,
    pub material_id: i32,
    pub quantity: i32,
    pub is_input: bool,
}

// FIXME:
/*#[cfg(test)]
mod blueprint_tests {
    use super::*;

    use caph_eve_sde_parser::{BlueprintAdditional, Material};
    use async_std::sync::Mutex;

    fn cache_service(blueprints: Vec<BlueprintCacheEntry>) -> Arc<CacheService> {
        Arc::new(CacheService {
            blueprints: Mutex::new(blueprints),
            items: Mutex::new(Vec::new()),
            markets: Mutex::new(Vec::new()),
            names: Mutex::new(Vec::new()),
            regions: Mutex::new(Vec::new()),
            schematics: Mutex::new(Vec::new()),
            solarsystems: Mutex::new(Vec::new()),
            sde_checksum: Mutex::new(Vec::new()),
        })
    }

    #[async_std::test]
    async fn resolve_001() {
        let blueprint = BlueprintCacheEntry {
            id: 99,
            manufacturing: Some(
                BlueprintAdditional {
                    materials: Some(vec![
                        Material {
                            quantity: 10,
                            type_id: 1,
                            probability: None
                        }
                    ]),
                    products: Some(vec![
                        Material {
                            quantity: 1,
                            type_id: 0,
                            probability: None
                        }
                    ]),
                    time: 1,
                    skills: None,
                }
            ),
            copying: None,
            invention: None,
            reaction: None,
            research_material: None,
            research_time: None
        };

        let blueprints = vec![blueprint];
        let cache = cache_service(blueprints);
        let market_service = MarketService::new(cache.clone(), crate::services::ItemService::new(cache.clone()));
        let instance = BlueprintService { cache, market_service };
        let result = instance.calc_bp_cost(0).await;

        let mut expected = HashMap::new();
        expected.insert(1u32, 10u64);

        assert_eq!(result, expected);
    }

    // The inner needs 10 times the item 1, which in return needs 10 times item 2 -> Item 2 is needed 100 times
    #[async_std::test]
    async fn resolve_002() {
        let blueprint_outer = BlueprintCacheEntry {
            id: 99,
            manufacturing: Some(
                BlueprintAdditional {
                    materials: Some(vec![
                        Material {
                            quantity: 10,
                            type_id: 2,
                            probability: None
                        }
                    ]),
                    products: Some(vec![
                        Material {
                            quantity: 1,
                            type_id: 1,
                            probability: None
                        }
                    ]),
                    time: 1,
                    skills: None,
                }
            ),
            copying: None,
            invention: None,
            reaction: None,
            research_material: None,
            research_time: None
        };

        let blueprint_inner = BlueprintCacheEntry {
            id: 100,
            manufacturing: Some(
                BlueprintAdditional {
                    materials: Some(vec![
                        Material {
                            quantity: 10,
                            type_id: 1,
                            probability: None
                        }
                    ]),
                    products: Some(vec![
                        Material {
                            quantity: 1,
                            type_id: 0,
                            probability: None
                        }
                    ]),
                    time: 1,
                    skills: None,
                }
            ),
            copying: None,
            invention: None,
            reaction: None,
            research_material: None,
            research_time: None
        };

        let blueprints = vec![blueprint_inner, blueprint_outer];
        let cache = cache_service(blueprints);
        let market_service = MarketService::new(cache.clone(),  crate::services::ItemService::new(cache.clone()));
        let instance = BlueprintService { cache,market_service };
        let result = instance.calc_bp_cost(0).await;

        let mut expected = HashMap::new();
        expected.insert(2u32, 100u64);

        assert_eq!(result, expected);
    }

    // Item 1 is needed 1 time, wich needs Item 2 10 times, which needs Item 3 10 times -> Item 3 = 1000 times needed
    #[async_std::test]
    async fn resolve_003() {
        let blueprint_0 = BlueprintCacheEntry {
            id: 99,
            manufacturing: Some(
                BlueprintAdditional {
                    materials: Some(vec![
                        Material {
                            quantity: 10,
                            type_id: 3,
                            probability: None
                        }
                    ]),
                    products: Some(vec![
                        Material {
                            quantity: 1,
                            type_id: 2,
                            probability: None
                        }
                    ]),
                    time: 1,
                    skills: None,
                }
            ),
            copying: None,
            invention: None,
            reaction: None,
            research_material: None,
            research_time: None
        };

        let blueprint_1 = BlueprintCacheEntry {
            id: 100,
            manufacturing: Some(
                BlueprintAdditional {
                    materials: Some(vec![
                        Material {
                            quantity: 10,
                            type_id: 2,
                            probability: None
                        }
                    ]),
                    products: Some(vec![
                        Material {
                            quantity: 1,
                            type_id: 1,
                            probability: None
                        }
                    ]),
                    time: 1,
                    skills: None,
                }
            ),
            copying: None,
            invention: None,
            reaction: None,
            research_material: None,
            research_time: None
        };

        let blueprint_2 = BlueprintCacheEntry {
            id: 101,
            manufacturing: Some(
                BlueprintAdditional {
                    materials: Some(vec![
                        Material {
                            quantity: 10,
                            type_id: 1,
                            probability: None
                        }
                    ]),
                    products: Some(vec![
                        Material {
                            quantity: 1,
                            type_id: 0,
                            probability: None
                        }
                    ]),
                    time: 1,
                    skills: None,
                }
            ),
            copying: None,
            invention: None,
            reaction: None,
            research_material: None,
            research_time: None
        };

        let blueprints = vec![blueprint_0, blueprint_1, blueprint_2];
        let cache = cache_service(blueprints);
        let market_service = MarketService::new(cache.clone(), crate::services::ItemService::new(cache.clone()));
        let instance = BlueprintService { cache, market_service };
        let result = instance.calc_bp_cost(0).await;

        let mut expected = HashMap::new();
        expected.insert(3u32, 1000u64);

        assert_eq!(result, expected);
    }
}*/
