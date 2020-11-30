use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ItemService(Pool<Postgres>);

impl ItemService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    pub async fn all(&self) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;
        sqlx::query_as::<_, Item>("SELECT id, name FROM items")
            .fetch_all(&mut conn)
            .await
            .map_err(|x| x.into())
    }

    pub async fn by_id(&self, id: u32) -> Result<Item, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;
        sqlx::query_as::<_, Item>("SELECT id, name FROM items WHERE id = $1")
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .map_err(|x| x.into())
    }

    /// If a id does not exist, it will silently by ignored
    pub async fn bulk_item_by_id(
        &self,
        ids: Vec<u32>,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;
        sqlx::query_as::<_, Item>(&format!(
            "SELECT id, name FROM items WHERE id = ANY(ARRAY {:?})",
            ids
        ))
        .fetch_all(&mut conn)
        .await
        .map_err(|x| x.into())
    }

    pub async fn search(
        &self,
        exact: bool,
        name: &str,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;

        if exact {
            sqlx::query_as::<_, Item>("SELECT id, name FROM items WHERE name = $1")
                .bind(name)
                .fetch_all(&mut conn)
                .await
                .map_err(|x| x.into())
        } else {
            sqlx::query_as::<_, Item>(&format!(
                "SELECT id, name FROM items WHERE name ILIKE '%{}%'",
                name
            ))
            .fetch_all(&mut conn)
            .await
            .map_err(|x| x.into())
        }
    }

    pub async fn bulk_search(
        &self,
        exact: bool,
        names: Vec<String>,
    ) -> Result<HashMap<String, Vec<Item>>, Box<dyn std::error::Error>> {
        let mut results = HashMap::new();
        for name in names {
            results.insert(name.clone(), self.search(exact, &name).await?);
        }
        Ok(results)
    }

    pub async fn reprocessing(
        &self,
        id: u32,
    ) -> Result<Vec<ItemReprocessing>, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;

        sqlx::query_as::<_, ItemReprocessing>(
            "SELECT id, material_id, quantity FROM item_materials WHERE id = $1",
        )
        .bind(id)
        .fetch_all(&mut conn)
        .await
        .map_err(|x| x.into())
    }

    pub async fn bulk_reprocessing(
        &self,
        ids: Vec<u32>,
    ) -> Result<HashMap<u32, Vec<ItemReprocessing>>, Box<dyn std::error::Error>> {
        let mut results = HashMap::new();
        for id in ids {
            results.insert(id, self.reprocessing(id).await?);
        }
        Ok(results)
    }
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct ItemReprocessing {
    pub id: i32,
    pub material_id: i32,
    pub quantity: i32,
}
