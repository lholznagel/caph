use crate::error::EveServerError;
use crate::reprocessing::calc_reprocessing;

use cachem::{ConnectionPool, Protocol};
use caph_db::{FetchIdNameReq, FetchIdNameRes, FetchItemMaterialReq, FetchItemMaterialRes, FetchItemReq, FetchItemRes, IdNameEntry, ItemEntry};
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

    pub async fn resolve_id(&self, id: u32) -> Result<Option<IdNameEntry>, EveServerError> {
        let mut conn = self.0.acquire().await?;

        Protocol::request::<_, FetchIdNameRes>(
            &mut conn,
            FetchIdNameReq(id)
        )
        .await
        .map(|x| {
            match x {
                FetchIdNameRes::Ok(x) => Some(x),
                _ => None,
            }
        })
        .map_err(Into::into)
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
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemReprocessingResult {
    pub id:          u32,
    pub material_id: u32,
    pub quantity:    u32,
    pub reprocessed: f32,
}
