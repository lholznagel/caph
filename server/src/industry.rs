use crate::ServerError;

use caph_connector::{CharacterId, JobId, StationId, TypeId};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct IndustryService {
    pool:  PgPool,
}

impl IndustryService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn jobs(
        &self,
        cid: CharacterId,
    ) -> Result<Vec<IndustryJob>, ServerError> {
        let entry = sqlx::query!("
                SELECT ij.*, i.name
                FROM industry_job ij
                JOIN item i
                  ON i.type_id = ij.type_id
                WHERE character_id = ANY(
                    SELECT character_id
                    FROM character
                    WHERE character_id = $1
                       OR character_main = $1
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
                    name:         x.name
                }
            })
            .collect::<Vec<_>>();
        Ok(entry)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct IndustryJob {
    /// Activity of the job
    #[serde(rename = "activity_id")]
    pub activity:    i32,
    /// TypeId of the blueprint that is used
    #[serde(rename = "blueprint_type_id")]
    pub type_id:      TypeId,
    /// Date the character started the job
    #[serde(rename = "start_date")]
    pub start_date:   String,
    /// Date when the job is done
    #[serde(rename = "end_date")]
    pub end_date:     String,
    /// Character id of the character that installed the job
    #[serde(rename = "installer_id")]
    pub character_id: CharacterId,
    /// Id of the station the job was started
    #[serde(rename = "station_id")]
    pub station_id:   StationId,
    /// Unique id of the job
    #[serde(rename = "job_id")]
    pub job_id:       JobId,
    /// Name of the item that is produced
    #[serde(rename = "name")]
    pub name:         String
}
