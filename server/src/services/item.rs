use crate::error::EveServerError;
use crate::reprocessing::calc_reprocessing;

use cachem::{ConnectionPool, Protocol};
use caph_db::{BlueprintEntry, FetchBlueprintReq, FetchBlueprintRes, FetchItemMaterialReq, FetchItemMaterialRes, FetchItemReq, FetchItemRes, ItemEntry, Material};
use serde::Serialize;

#[derive(Clone)]
pub struct ItemService(ConnectionPool);

impl ItemService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self(pool)
    }

    pub async fn by_id(&self, id: u32) -> Result<Option<ItemEntry>, EveServerError> {
        let mut conn = self.0.acquire().await?;

        Protocol::request::<_, FetchItemRes>(
            &mut conn,
            FetchItemReq(id)
        )
        .await
        .map(|x| {
            match x {
                FetchItemRes::Ok(x) => Some(x),
                _ => None
            }
        })
        .map_err(Into::into)
    }

    pub async fn bulk(&self, ids: Vec<u32>) -> Result<Vec<ItemEntry>, EveServerError> {
        let mut result = Vec::with_capacity(ids.len());

        for id in ids {
            let item = self.by_id(id).await?;
            if let Some(x) = item {
                result.push(x);
            }
        }

        Ok(result)
    }

    pub async fn blueprint_graph(
        &self,
        id: u32,
    ) -> Result<BlueprintGraph, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let blueprints = Protocol::request::<_, FetchBlueprintRes>(
            &mut conn,
            FetchBlueprintReq::default(),
        )
        .await
        .map(|x| x.0)?;

        let product = blueprints
            .iter()
            .find(|x| x.item_id == id)
            .unwrap();

        let bp_result = product
            .materials
            .iter()
            .find(|x| x.is_product)
            .unwrap();

        let materials = product
            .materials
            .iter()
            .filter(|x| !x.is_product)
            .collect::<Vec<_>>();

        let mut graphs = Vec::new();
        for material in materials {
            let graph = self
                .build_graph(
                                material.material_id,
                                material.quantity,
                                &blueprints
                            );
            graphs.push(graph);
        }

        let root = BlueprintGraph {
            item_id: bp_result.material_id,
            quantity: bp_result.quantity,
            children: graphs
        };

        Ok(root)
    }

    fn build_graph(
        &self,
        material_id: u32,
        quantity: u32,
        blueprints: &Vec<BlueprintEntry>
    ) -> BlueprintGraph {
        let find_product = blueprints
            .iter()
            .map(|x| x.materials.clone())
            .flatten()
            .find(|x| x.is_product && x.material_id == material_id);
        if let Some(x) = find_product {
            // There is an blueprint that produces this item
            let materials = blueprints
                .iter()
                .find(|y| {
                    y.materials
                        .iter()
                        .find(|y| y.is_product && y.material_id == x.material_id)
                        .is_some()
                })
                .map(|x| x.materials.clone())
                .unwrap_or_default();
            let materials = materials
                .iter()
                .filter(|x| !x.is_product)
                .collect::<Vec<_>>();
            BlueprintGraph {
                item_id: material_id,
                quantity,
                // iterate over all materials and build there graph
                children: materials
                            .iter()
                            .map(|x| self
                                        .build_graph(
                                            x.material_id,
                                            x.quantity,
                                            &blueprints
                                        )
                            )
                            .collect::<Vec<_>>()
            }
        } else {
            // Root item, no blueprint found for this item
            BlueprintGraph {
                item_id: material_id,
                quantity,
                children: Vec::new(),
            }
        }
    }

    pub async fn reprocessing(
        &self,
        id: u32,
    ) -> Result<Vec<ItemReprocessingResult>, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let ret = Protocol::request::<_, FetchItemMaterialRes>(
            &mut conn,
            FetchItemMaterialReq(id)
        )
        .await
        .map(|x| {
            if let FetchItemMaterialRes::Ok(x) = x {
                x
            } else {
                Vec::new()
            }
        })?
        .iter()
        .map(|x| {
            let modifier = calc_reprocessing(50, 0, 0, 0);
            ItemReprocessingResult {
                id: x.item_id,
                material_id: x.material_id,
                quantity: x.quantity,
                reprocessed: x.quantity as f32 * (modifier / 100f32),
            }
        })
        .collect::<Vec<_>>();
        Ok(ret)
    }

    pub async fn blueprint_product(
        &self,
        bid: u32,
    ) -> Result<u32, EveServerError> {
        let mut conn = self.0.acquire().await?;

        let blueprints = Protocol::request::<_, FetchBlueprintRes>(
            &mut conn,
            FetchBlueprintReq::default(),
        )
        .await
        .map(|x| x.0)?;

        let product = blueprints
            .iter()
            .find(|x| x.item_id == bid)
            .unwrap();

        let bp_result = product
            .materials
            .iter()
            .find(|x| x.is_product)
            .ok_or(EveServerError::NotFound)?;

        Ok(bp_result.material_id)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemReprocessingResult {
    pub id:          u32,
    pub material_id: u32,
    pub quantity:    u32,
    pub reprocessed: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct BlueprintGraph {
    pub item_id:  u32,
    pub quantity: u32,
    pub children: Vec<BlueprintGraph>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BlueprintResult {
    pub item_id:   u32,
    pub time:      u32,
    pub materials: Vec<BlueprintMaterialResult>,
}

impl From<BlueprintEntry> for BlueprintResult {
    fn from(x: BlueprintEntry) -> Self {
        Self {
            item_id: x.item_id,
            time: x.time,
            materials: x.materials
                        .into_iter()
                        .map(BlueprintMaterialResult::from)
                        .collect::<Vec<_>>()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BlueprintMaterialResult {
    pub material_id: u32,
    pub quantity:    u32,
    pub is_product:  bool,
}

impl From<Material> for BlueprintMaterialResult {
    fn from(x: Material) -> Self {
        Self {
            material_id: x.material_id,
            quantity: x.quantity,
            is_product: x.is_product,
        }
    }
}
