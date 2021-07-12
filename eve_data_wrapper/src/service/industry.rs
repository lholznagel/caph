use crate::*;

#[derive(Clone, Debug)]
pub struct IndustryService {
    eve_client: EveClient,
}

impl IndustryService {
    pub fn new(
        eve_client: EveClient,
        _: SdeZipArchive
    ) -> Result<Self, EveConnectError> {
        Ok(Self {
            eve_client
        })
    }

    pub async fn jobs(
        &self,
        token: &str,
        character_id: CharacterId,
    ) -> Result<Vec<IndustryJob>, EveConnectError> {
        let path = format!("characters/{}/industry/jobs", *character_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json::<Vec<IndustryJob>>()
            .await
            .map_err(Into::into)
    }

    pub async fn corp_jobs(
        &self,
        token: &str,
        corporation_id: CorporationId,
    ) -> Result<Vec<IndustryJob>, EveConnectError> {
        let path = format!("corporations/{}/industry/jobs", *corporation_id);
        self
            .eve_client
            .fetch_oauth(&token, &path)
            .await?
            .json::<Vec<IndustryJob>>()
            .await
            .map_err(Into::into)
    }

    pub async fn systems(&self) -> Result<Vec<IndustrySystem>, EveConnectError> {
        self
            .eve_client
            .fetch("industry/systems")
            .await?
            .json()
            .await
            .map_err(Into::into)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndustryJob {
    pub activity_id:            u32,
    pub blueprint_id:           u64,
    pub blueprint_location_id:  u64,
    pub blueprint_type_id:      u32,
    pub duration:               u32,
    pub end_date:               String,
    pub facility_id:            u64,
    pub installer_id:           u32,
    pub job_id:                 u32,
    pub output_location_id:     u64,
    pub runs:                   u32,
    pub start_date:             String,
    pub status:                 String,

    pub completed_character_id: Option<u32>,
    pub completed_date:         Option<String>,
    pub cost:                   Option<f32>,
    pub licensed_runs:          Option<u32>,
    pub paused_date:            Option<String>,
    pub probability:            Option<f32>,
    pub successful_runs:        Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IndustrySystem {
    pub cost_indices:    Vec<CostIndex>,
    pub solar_system_id: SolarSystemId
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CostIndex {
    pub activity:   String,
    pub cost_index: f32,
}
