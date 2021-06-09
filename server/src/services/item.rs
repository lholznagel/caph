use crate::error::EveServerError;
use crate::reprocessing::calc_reprocessing;

use cachem::v2::ConnectionPool;
use caph_db_v2::CacheName;
use caph_db_v2::ItemEntry;
use caph_db_v2::ReprocessEntry;
use caph_eve_data_wrapper::TypeId;
use serde::Serialize;

#[derive(Clone)]
pub struct ItemService(ConnectionPool);

impl ItemService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self(pool)
    }

    pub async fn by_id(&self, iid: TypeId) -> Result<Option<ItemEntry>, EveServerError> {
        self.0
            .acquire()
            .await?
            .get::<_, _, ItemEntry>(CacheName::Item, *iid)
            .await
            .map_err(Into::into)
    }

    pub async fn bulk(&self, ids: Vec<TypeId>) -> Result<Vec<ItemEntry>, EveServerError> {
        let res = self.0
            .acquire()
            .await?
            .mget::<_, _, ItemEntry>(CacheName::Item, ids)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        Ok(res)
    }

    pub async fn resolve_id(&self, tid: TypeId) -> Result<Option<String>, EveServerError> {
        self.0
            .acquire()
            .await?
            .get::<_, _, String>(CacheName::Name, tid)
            .await
            .map_err(Into::into)
    }

    pub async fn resolve_bulk(&self, ids: Vec<TypeId>) -> Result<Vec<String>, EveServerError> {
        let res = self.0
            .acquire()
            .await?
            .mget::<_, _, String>(CacheName::Name, ids)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        Ok(res)
    }

    pub async fn reprocessing(
        &self,
        tid: TypeId,
    ) -> Result<Vec<ItemReprocessingResult>, EveServerError> {
        let ret = self.0
            .acquire()
            .await?
            .get::<_, _, Vec<ReprocessEntry>>(CacheName::Reprocess, tid)
            .await?
            .unwrap_or_default()
            .iter()
            .map(|x| {
                let modifier = calc_reprocessing(50, 0, 0, 0);
                ItemReprocessingResult {
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
    pub material_id: TypeId,
    pub quantity:    u32,
    pub reprocessed: f32,
}

