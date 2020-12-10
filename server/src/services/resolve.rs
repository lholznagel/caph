use serde::Serialize;
use sqlx::{Pool, Postgres};

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

    pub async fn resolve(&self, id: u32) -> Result<Option<Resolve>, Box<dyn std::error::Error>> {
        if let Some(x) = self.find_in_items(id).await {
            return Ok(Some(x));
        } else if let Some(x) = self.find_in_names(id).await {
            return Ok(Some(x));
        } else {
            Ok(None)
        }
    }

    async fn find_in_items(&self, id: u32) -> Option<Resolve> {
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

    async fn find_in_names(&self, id: u32) -> Option<Resolve> {
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
