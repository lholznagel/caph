use crate::error::EveServerError;

use cachem::v2::ConnectionPool;
use caph_db_v2::{BlueprintEntry, CacheName};
use caph_eve_data_wrapper::TypeId;

#[derive(Clone)]
pub struct BlueprintService(ConnectionPool);

impl BlueprintService {
    pub fn new(pool: ConnectionPool) -> Self {
        Self(pool)
    }

    pub async fn blueprint(
        &self,
        bid: TypeId,
    ) -> Result<Option<BlueprintEntry>, EveServerError> {
        self.0
            .acquire()
            .await?
            .get::<_, _, BlueprintEntry>(CacheName::Blueprint, bid)
            .await
            .map_err(Into::into)
    }
}

