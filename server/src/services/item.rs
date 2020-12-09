use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;

use crate::reprocessing::calc_reprocessing;

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
    ) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        for name in names {
            results.extend(self.search(exact, &name).await?);
        }
        Ok(results)
    }

    pub async fn reprocessing(
        &self,
        id: u32,
    ) -> Result<Vec<ItemReprocessingResult>, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;

        let result = sqlx::query_as::<_, ItemReprocessing>(
            "SELECT id, material_id, quantity FROM item_materials WHERE id = $1",
        )
        .bind(id)
        .fetch_all(&mut conn)
        .await
        .unwrap()
        .iter()
        .map(|x| {
            let modifier = calc_reprocessing(50, 0, 0, 0);
            ItemReprocessingResult {
                id: x.id,
                material_id: x.material_id,
                quantity: x.quantity,
                reprocessed: x.quantity as f32 * (modifier / 100f32)
            }
        })
        .collect::<Vec<ItemReprocessingResult>>();
        Ok(result)
    }

    pub async fn bulk_reprocessing(
        &self,
        ids: Vec<u32>,
    ) -> Result<HashMap<u32, Vec<ItemReprocessingResult>>, Box<dyn std::error::Error>> {
        let mut results = HashMap::new();
        for id in ids {
            results.insert(id, self.reprocessing(id).await?);
        }
        Ok(results)
    }

    pub async fn fetch_my_items(&self) -> Result<Vec<MyItem>, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;
        sqlx::query_as::<_, MyItem>(
            "SELECT id, quantity FROM user_items",
        )
        .fetch_all(&mut conn)
        .await
        .map_err(|x| x.into())
    }

    pub async fn fetch_my_item(&self, id: u32) -> Result<MyItem, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;
        sqlx::query_as::<_, MyItem>(
            "SELECT id, quantity FROM user_items WHERE id = $1",
        )
        .bind(id as i32)
        .fetch_one(&mut conn)
        .await
        .map_err(|x| x.into())
    }

    pub async fn push_my_items(&self, items: HashMap<u32, u64>) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;

        let ids = items
            .clone()
            .into_iter()
            .map(|(x, _)| x as i32)
            .collect::<Vec<i32>>();
        let quantities = items
            .into_iter()
            .map(|(_, x)| x as i64)
            .collect::<Vec<i64>>();

        sqlx::query(&format!("DELETE FROM user_items WHERE id = ANY(ARRAY {:?})", ids))
            .execute(&mut conn)
            .await?;

        sqlx::query(
            r#"INSERT INTO user_items (id, quantity)
            SELECT * FROM UNNEST($1, $2)
            RETURNING id, quantity"#,
        )
        .bind(&ids)
        .bind(&quantities)
        .execute(&mut *conn)
        .await?;

        Ok(())
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

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct ItemReprocessingResult {
    pub id: i32,
    pub material_id: i32,
    pub quantity: i32,
    pub reprocessed: f32,
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct MyItem {
    pub id: i32,
    pub quantity: i64,
}