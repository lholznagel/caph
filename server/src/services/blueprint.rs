use crate::error::EveServerError;

use cachem::{ConnectionPool, Protocol};
use caph_db::{BlueprintEntry, FetchBlueprintReq, FetchBlueprintRes, PlanetSchematicEntry};
use serde::Serialize;

#[derive(Clone)]
pub struct BlueprintService(ConnectionPool);

impl BlueprintService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self(pool)
    }

    pub async fn blueprint(
        &self,
        bid: u32,
    ) -> Result<Option<BlueprintEntry>, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let res = Protocol::request::<_, FetchBlueprintRes>(
            &mut conn,
            FetchBlueprintReq::default(),
        )
        .await?;

        Ok(
            res
                .blueprints
                .into_iter()
                .find(|x| x.bid == bid)
        )
    }

    pub async fn blueprint_graph(
        &self,
        id: u32,
    ) -> Result<BlueprintGraph, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let res = Protocol::request::<_, FetchBlueprintRes>(
            &mut conn,
            FetchBlueprintReq::default(),
        )
        .await?;

        let blueprint = res
            .blueprints
            .iter()
            .find(|x| x.bid == id)
            .ok_or(EveServerError::NotFound)?;

        let mut id = 0;
        let mut graphs = Vec::new();
        for material in blueprint.materials.iter() {
            let graph = self
                .build_graph(
                    &mut id,
                    material.material_id,
                    material.quantity,
                    &res.blueprints,
                    &res.schematics,
                );
            graphs.push(graph);
        }

        let root = BlueprintGraph {
            id:       0,
            item_id:  blueprint.product.material_id,
            quantity: blueprint.product.quantity,
            children: graphs
        };

        Ok(root)
    }

    fn build_graph(
        &self,
        id:          &mut u32,
        material_id: u32,
        quantity:    u32,
        blueprints:  &Vec<BlueprintEntry>,
        schematics:  &Vec<PlanetSchematicEntry>,
    ) -> BlueprintGraph {
        *id += 1;
        // search for a blueprint that produces the given material
        let bp = blueprints
            .iter()
            .find(|x| x.product.material_id == material_id);

        if let Some(x) = bp {
            BlueprintGraph {
                id:       *id,
                item_id:  material_id,
                quantity,
                // iterate over all materials and build there graph
                children: x.materials
                            .iter()
                            .map(|x| self
                                        .build_graph(
                                            id,
                                            x.material_id,
                                            x.quantity,
                                            &blueprints,
                                            &schematics,
                                        )
                            )
                            .collect::<Vec<_>>()
            }
        } else {
            // search if there is a schematic that produces the material_id
            let ps = schematics
                .iter()
                .find(|x| x.output.material_id == material_id);

            if let Some(x) = ps {
                BlueprintGraph {
                    id:       *id,
                    item_id:  material_id,
                    quantity,
                    // iterate over all materials and build there graph
                    children: x.inputs
                                .iter()
                                .map(|x| self
                                            .build_graph(
                                                id,
                                                x.material_id,
                                                x.quantity,
                                                &blueprints,
                                                &schematics,
                                            )
                                )
                                .collect::<Vec<_>>()
                }
            } else {
                // Root item, no blueprint or schematic found for this item
                BlueprintGraph {
                    id:       *id,
                    item_id:  material_id,
                    quantity,
                    children: Vec::new(),
                }
            }
        }
    }

    pub async fn blueprint_product(
        &self,
        bid: u32,
    ) -> Result<Option<u32>, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let res = Protocol::request::<_, FetchBlueprintRes>(
            &mut conn,
            FetchBlueprintReq::default(),
        )
        .await?;

        let bp = res
            .blueprints
            .iter()
            .find(|x| x.bid == bid);
        if let Some(x) = bp {
            Ok(Some(x.product.material_id))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BlueprintGraph {
    pub id:       u32,
    pub item_id:  u32,
    pub quantity: u32,
    pub children: Vec<BlueprintGraph>,
}

