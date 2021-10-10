use crate::ServerError;

use caph_connector::{CharacterId, IndustryJob};
use sqlx::PgPool;

#[derive(Clone)]
pub struct IndustryService {
    pool:  PgPool,
}

impl IndustryService {
    pub async fn products(
        &self,
        cid: CharacterId,
    ) -> Result<Vec<IndustryJob>, ServerError> {
        let entry = sqlx::query!("
                SELECT *
                FROM industry_job
                WHERE character_id = ANY(
                    SELECT character_id
                    FROM character
                    WHERE character_id = $1
                )
            ",
                *cid,
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                IndustryJob {
                    activity:     x.activity,
                    type_id:      x.type_id.into(),
                    start_date:   x.start_date,
                    end_date:     x.end_date,
                    character_id: x.character_id.into(),
                    station_id:   x.station_id.into(),
                    job_id:       x.job_id.into(),
                }
            })
            .collect::<Vec<_>>();
        Ok(entry)
    }
}
