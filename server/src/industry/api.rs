use caph_connector::CharacterId;
use reqwest::StatusCode;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;

use crate::{AuthService, AuthCharacter, with_authorization, ESI_READ_INDUSTRY_JOBS, ESI_READ_CORPORATION_INDUSTRY_JOBS, ESI_READ_ASSETS, ESI_READ_CORPORATION_ASSETS};
use super::service::{IndustryService, with_industry_service};

#[derive(Clone, Debug)]
pub struct IndustryApi;

impl IndustryApi {
    /// Filters that build up the api for this part of the application
    pub fn api(
        pool:      PgPool,
        base_path: BoxedFilter<()>,
    ) -> BoxedFilter<(impl Reply,)> {
        let base_path = base_path
            .clone()
            .and(warp::path!("industry" / ..))
            .and(with_authorization(pool.clone()))
            .and(with_industry_service(pool.clone()))
            .boxed();

        let character_jobs = base_path
            .clone()
            .and(warp::path!("jobs"))
            .and(warp::get())
            .and_then(Self::indy_job_character)
            .boxed();
        let corporation_jobs = base_path
            .clone()
            .and(warp::path!("jobs" / "corporation"))
            .and(warp::get())
            .and_then(Self::indy_job_corporation)
            .boxed();

        let get_stockpile = base_path
            .clone()
            .and(warp::path!("stockpile"))
            .and(warp::get())
            .and_then(Self::stockpile)
            .boxed();

        character_jobs
            .or(corporation_jobs)
            .or(get_stockpile)
            .boxed()
    }

    /// Gets all industry jobs for all characters that the logged in character
    /// has registered.
    /// 
    /// # Errors
    /// 
    /// - If the database is not available
    /// - If the EVE API is not available
    /// 
    /// # Returns
    /// 
    /// List of all industry jobs for all characters
    /// 
    async fn indy_job_character(
        auth:    AuthCharacter,
        service: IndustryService,
    ) -> Result<impl Reply, Rejection> {
        let mut cid_client = Vec::new();

        let characters = auth.with_scope(ESI_READ_INDUSTRY_JOBS).await?;
        for c in characters {
            let client = auth.eve_auth_client(&c.character_id).await?;
            cid_client.push((c, client));
        }

        service
            .character_jobs(cid_client)
            .await
            .map_err(Into::into)
            .map(|x| warp::reply::json(&x))
    }

    /// Gets all industry jobs for the corporation of the given user id.
    /// 
    /// # Errors
    /// 
    /// - If the scope is not given for that character
    /// - If the given character is not the main character or an alt
    /// - If the character does not have the corporation permission
    /// 
    /// # Returns
    /// 
    /// List of all industry jobs for that character
    /// 
    async fn indy_job_corporation(
        auth:    AuthCharacter,
        service: IndustryService,
    ) -> Result<impl Reply, Rejection> {
        let mut cid_client = Vec::new();

        let characters = auth
            .with_scope(ESI_READ_CORPORATION_INDUSTRY_JOBS)
            .await?;
        for c in characters {
            let client = auth.eve_auth_client(&c.character_id).await?;
            cid_client.push((c, client));
        }

        service
            .corporation_jobs(cid_client)
            .await
            .map_err(Into::into)
            .map(|x| warp::reply::json(&x))
    }

    async fn stockpile(
        auth:    AuthCharacter,
        service: IndustryService,
    ) -> Result<impl Reply, Rejection> {
        let mut cid_client = Vec::new();

        let characters = auth
            .with_scope(ESI_READ_ASSETS)
            .await?;
        for c in characters {
            let client = auth.eve_auth_client(&c.character_id).await?;
            cid_client.push((c, client));
        }

        service
            .character_assets(cid_client)
            .await;

        /*cid_client = Vec::new();
        let characters = auth
            .with_scope(ESI_READ_CORPORATION_ASSETS)
            .await?;
        for c in characters {
            let client = auth.eve_auth_client(&c.character_id).await?;
            cid_client.push((c, client));
        }
        service
            .corporation_assets(cid_client)
            .await
            .map_err(Into::into)?;*/

        Ok(warp::reply::with_status(warp::reply::json(&()), StatusCode::OK))
    }
}
