use crate::{error::EveServerError, eve::EveAuthService};

use caph_eve_data_wrapper::{EveDataWrapper, IndustryJob, TypeId};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct IndustryService {
    eve_auth: EveAuthService,
    eve_data: EveDataWrapper,
}

impl IndustryService {
    pub fn new(
        eve_auth: EveAuthService,
        eve_data: EveDataWrapper,
    ) -> Self {
        Self {
            eve_auth,
            eve_data,
        }
    }

    pub async fn jobs(
        &self,
        token: String,
    ) -> Result<Vec<IndustryJob>, EveServerError> {
        let user = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;

        let mut jobs = Vec::new();
        for alt in user.aliase.iter() {
            let token = self
                .eve_auth
                .refresh_token_alt(&token, alt.user_id)
                .await?
                .access_token;
            let job = self
                .eve_data
                .industry()
                .await?
                .jobs(&token, alt.user_id)
                .await?;
            jobs.extend(job);
        }

        let token = self
            .eve_auth
            .refresh_token(&token)
            .await?
            .access_token;
        let job = self
            .eve_data
            .industry()
            .await?
            .jobs(&token, user.user_id)
            .await?;
        // Get all corp jobs and filter them for the user id
        let jobs_corp = self
            .eve_data
            .industry()
            .await?
            .corp_jobs(&token, user.corp_id)
            .await?
            .into_iter()
            .filter(|x| x.installer_id == *user.user_id)
            .collect::<Vec<_>>();
        jobs.extend(job);
        jobs.extend(jobs_corp);

        Ok(jobs)
    }

    pub fn stations(&self) -> Result<Vec<Facility>, EveServerError> {
        serde_json::from_str(
            &include_str!("../assets/stations.json")
        )
        .map_err(Into::into)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Facility {
    pub id:          u32,
    pub name:        String,
    pub engineering: EngineeringInfo,
    pub refinery:    RefineryInfo
}

#[derive(Deserialize, Serialize)]
pub struct EngineeringInfo {
    pub manufacturing:       f32,
    pub time_efficiency:     Option<f32>,
    pub material_efficiency: Option<f32>,
    pub copy:                Option<f32>,
    pub invention:           Option<f32>,
    pub type_id:             TypeId,
}

#[derive(Deserialize, Serialize)]
pub struct RefineryInfo {
    pub composit:    f32,
    pub biochemical: Option<f32>,
    pub hybrid:      Option<f32>,
    pub type_id:     TypeId,
}
