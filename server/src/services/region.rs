use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct RegionService(Pool<Postgres>);

impl RegionService {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self(db)
    }

    pub async fn all(&self) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut conn = self.0.acquire().await?;
        sqlx::query_as::<_, RegionId>("SELECT DISTINCT region_id FROM stations")
            .fetch_all(&mut conn)
            .await
            .map(|x| x.into_iter().map(|y| y.region_id as u32).collect::<Vec<u32>>())
            .map_err(|x| x.into())
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct RegionId {
    pub region_id: i32,
}
