use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::error::EveServerError;

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Resolve {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct ResolveService(Pool<Postgres>);

impl ResolveService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    pub async fn resolve_to_name(&self, id: u32) -> Result<Option<Resolve>, EveServerError> {
        if let Some(x) = self.find_in_items_by_id(id).await {
            return Ok(Some(x));
        } else if let Some(x) = self.find_in_names_by_id(id).await {
            return Ok(Some(x));
        } else {
            Ok(None)
        }
    }

    pub async fn bulk_resolve_to_name(&self, ids: Vec<u32>) -> Result<Vec<Resolve>, EveServerError> {
        let mut results = Vec::with_capacity(ids.len());

        for id in ids {
            if let Ok(Some(x)) = self.resolve_to_name(id).await {
                results.push(x);
            }
        }

        Ok(results)
    }

    async fn find_in_items_by_id(&self, id: u32) -> Option<Resolve> {
        let mut conn = self.0.acquire().await.unwrap();
        let query = sqlx::query_as::<_, Resolve>("SELECT id, name FROM items WHERE id = $1")
            .bind(id)
            .fetch_one(&mut conn)
            .await;

        match query {
            Ok(x) => Some(x),
            _ => None,
        }
    }

    async fn find_in_names_by_id(&self, id: u32) -> Option<Resolve> {
        let mut conn = self.0.acquire().await.unwrap();
        let query = sqlx::query_as::<_, Resolve>("SELECT id, name FROM names WHERE id = $1")
            .bind(id)
            .fetch_one(&mut conn)
            .await;

        match query {
            Ok(x) => Some(x),
            _ => None,
        }
    }
}
