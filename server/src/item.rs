use crate::ServerError;

use caph_connector::TypeId;
use sqlx::PgPool;

#[derive(Clone)]
pub struct ItemService {
    pool: PgPool
}

impl ItemService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn name(
        &self,
        tid: TypeId
    ) -> Result<String, ServerError> {
        let entry = sqlx::query!("
                SELECT name
                FROM items
                WHERE type_id = $1
            ",
                *tid
            )
            .fetch_optional(&self.pool)
            .await?;

        if let Some(e) = entry {
            Ok(e.name)
        } else {
            Err(ServerError::NotFound)
        }
    }
}
