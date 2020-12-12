use caph_eve_online_api::{EveClient, RouteFlag};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::error::EveServerError;

#[derive(Clone)]
pub struct RegionService(Pool<Postgres>);

impl RegionService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    pub async fn all(&self) -> Result<Vec<u32>, EveServerError> {
        let mut conn = self.0.acquire().await?;
        sqlx::query_as::<_, RegionId>("SELECT DISTINCT region_id FROM stations")
            .fetch_all(&mut conn)
            .await
            .map(|x| {
                x.into_iter()
                    .map(|y| y.region_id as u32)
                    .collect::<Vec<u32>>()
            })
            .map_err(|x| x.into())
    }

    pub async fn route(
        &self,
        origin: u32,
        destination: u32,
    ) -> Result<Vec<Route>, EveServerError> {
        let mut conn = self.0.acquire().await?;
        let routes = sqlx::query_as::<_, Route>(
            r#"
            SELECT origin, destination, systems, flag
            FROM routes
            WHERE origin = $1
            AND destination = $2"#,
        )
        .bind(origin as i32)
        .bind(destination as i32)
        .fetch_all(&mut conn)
        .await?;

        if routes.is_empty() {
            let mut routes = Vec::new();
            routes.push(self.fetch_and_insert_route(origin, destination, RouteFlag::Shortest).await?);
            routes.push(self.fetch_and_insert_route(origin, destination, RouteFlag::Secure).await?);
            routes.push(self.fetch_and_insert_route(origin, destination, RouteFlag::Insecure).await?);
            Ok(routes)
        } else {
            Ok(routes)
        }
    }

    async fn fetch_and_insert_route(&self, origin: u32, destination: u32, flag: RouteFlag) -> Result<Route, EveServerError> {
        let client = EveClient::default();
        let route = client
            .fetch_route(
                origin,
                destination,
                Some(flag.clone()),
            )
            .await
            .unwrap()
            .into_iter()
            .map(|x| x as i32)
            .collect::<Vec<i32>>();

        let mut conn = self.0.acquire().await?;
        let route = sqlx::query_as::<_, Route>(
            r#"
            INSERT INTO routes (origin, destination, systems, flag)
            VALUES ($1, $2, $3, $4)
            RETURNING origin, destination, systems, flag"#,
        )
        .bind(origin as i32)
        .bind(destination as i32)
        .bind(route)
        .bind(&flag.as_string())
        .fetch_one(&mut conn)
        .await?;
        Ok(route)
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct RegionId {
    pub region_id: i32,
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Route {
    pub origin: i32,
    pub destination: i32,
    pub systems: Vec<i32>,
    pub flag: String,
}
