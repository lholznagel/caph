use crate::EveConnectError;

use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use url::Url;

/// This struct contains all functions for communicating with the Eve Online
/// REST API.
#[derive(Clone, Debug)]
pub struct EveClient(Client);

impl EveClient {
    const EVE_API_URL:    &'static str = "https://esi.evetech.net/latest";
    const EVE_LOGIN_URL:  &'static str = "https://login.eveonline.com/v2/oauth/authorize";
    const EVE_TOKEN_URL:  &'static str = "https://login.eveonline.com/v2/oauth/token";
    const ENV_REDIRECT:   &'static str = "EVE_REDIRECT_URL";
    const ENV_CLIENT_ID:  &'static str = "EVE_CLIENT_ID";
    const ENV_SECRET_KEY: &'static str = "EVE_SECRET_KEY";

    pub fn new() -> Result<Self, EveConnectError> {
        let client = Client::builder()
            .user_agent("github.com/lholznagel")
            .build()?;

        Ok(Self(client))
    }

    pub fn eve_auth_uri(state: &str) -> Result<Url, EveConnectError> {
        let mut url = Url::parse(Self::EVE_LOGIN_URL).unwrap();

        let client_id    = std::env::var(Self::ENV_CLIENT_ID)
            .map_err(|_| EveConnectError::EnvError(format!("ENV var {} not present", Self::ENV_CLIENT_ID)))?;
        let redirect_uri = std::env::var(Self::ENV_REDIRECT)
            .map_err(|_| EveConnectError::EnvError(format!("ENV var {} not present", Self::ENV_REDIRECT)))?;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("client_id", &client_id)
            .append_pair("scope", &scope())
            .append_pair("state", state);
        Ok(url)
    }

    // https://docs.esi.evetech.net/docs/sso/web_based_sso_flow.html
    pub async fn retrieve_authorization_token(code: &str) -> Result<EveOAuthUser, EveConnectError> {
        let mut map = HashMap::new();
        map.insert("grant_type", "authorization_code");
        map.insert("code", code);

        let result = Self::send(map).await?;
        Ok(EveOAuthUser::from(result))
    }

    // https://docs.esi.evetech.net/docs/sso/refreshing_access_tokens.htmlEveOAuthUser
    pub async fn retrieve_refresh_token(refresh_token: &str) -> Result<EveOAuthUser, EveConnectError> {
        let mut map = HashMap::new();
        map.insert("grant_type", "refresh_token");
        map.insert("refresh_token", refresh_token);

        let result = Self::send(map).await?;
        Ok(EveOAuthUser::from(result))
    }

    async fn send<T: Serialize>(form: T) -> Result<EveOAuthToken, EveConnectError> {
        let client_id = std::env::var(Self::ENV_CLIENT_ID)
            .map_err(|_| EveConnectError::EnvError(format!("ENV var {} not present", Self::ENV_CLIENT_ID)))?;
        let secret_key = std::env::var(Self::ENV_SECRET_KEY)
            .map_err(|_| EveConnectError::EnvError(format!("ENV var {} not present", Self::ENV_SECRET_KEY)))?;

        Client::new()
            .post(Self::EVE_TOKEN_URL)
            .basic_auth(client_id, Some(secret_key))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Host", "login.eveonline.com")
            .form(&form)
            .send()
            .await?
            .json::<EveOAuthToken>()
            .await
            .map_err(Into::into)
    }

    /// Wraps reqwest´s client
    /// When requesting the eve online API often the server returns 502 or 503
    /// this results in a broken payload. If that happens, we just retry the request.
    /// The function will try 3 times, after that it will return an error.
    pub(crate) async fn fetch(&self, path: &str) -> Result<Response, EveConnectError> {
        let mut retry_counter = 0usize;

        loop {
            let url = format!("{}/{}", Self::EVE_API_URL, path);
            if retry_counter == 3 {
                log::error!("Too many retries requesting {}.", url);
                return Err(EveConnectError::TooManyRetries(url));
            }

            let response = self.0
                .get(&url)
                .send()
                .await;
            let response = response.map_err(EveConnectError::ReqwestError)?;

            // status 200 and 404 are ok
            if response.status() != StatusCode::OK &&
               response.status() != StatusCode::NOT_FOUND {
                retry_counter += 1;
                log::error!(
                    "Fetch resulted in non 200 or 404 status code. Statuscode was {}. Retrying.",
                    response.status()
                );
                continue;
            }

            return Ok(response);
        }
    }

    pub(crate) async fn fetch_oauth(
        &self,
        token: &str,
        path: &str
    ) -> Result<Response, EveConnectError> {
        let mut retry_counter = 0usize;

        loop {
            let url = format!("{}/{}", Self::EVE_API_URL, path);
            if retry_counter == 3 {
                log::error!("Too many retries requesting {}.", url);
                return Err(EveConnectError::TooManyRetries(url));
            }

            let response = self.0
                .get(&url)
                .bearer_auth(token)
                .send()
                .await;
            let response = response.map_err(EveConnectError::ReqwestError)?;

            if response.status() == StatusCode::UNAUTHORIZED ||
               response.status() == StatusCode::FORBIDDEN {
                return Err(EveConnectError::Unauthorized);
            }

            // status 200 and 404 are ok
            if response.status() != StatusCode::OK &&
               response.status() != StatusCode::NOT_FOUND {
                retry_counter += 1;
                log::error!(
                    "Fetch resulted in non 200 or 404 status code. Statuscode was {}. Retrying.",
                    response.status()
                );
                continue;
            }

            return Ok(response);
        }
    }

    pub(crate) async fn fetch_page<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<Vec<T>, EveConnectError> {
        let response = self
            .fetch(path)
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        fetched_data.extend(response.json::<Vec<T>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "{}?page={}",
                    path,
                    page
                ))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(EveConnectError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(fetched_data)
    }

    pub(crate) async fn fetch_page_oauth<T: DeserializeOwned>(
        &self,
        token: &str,
        path: &str,
    ) -> Result<Vec<T>, EveConnectError> {
        let response = self
            .fetch_oauth(token, path)
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        fetched_data.extend(response.json::<Vec<T>>().await?);

        for page in 2..=pages {
            let next_page = self
                .fetch(&format!(
                    "{}?page={}",
                    path,
                    page
                ))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(EveConnectError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(fetched_data)
    }

    pub(crate) async fn post_oauth<T, R>(
        &self,
        token: &str,
        path: &str,
        body: &T
    ) -> Result<R, EveConnectError>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned {

        dbg!(&serde_json::to_string(&body));
        let mut retry_counter = 0usize;

        loop {
            let url = format!("{}/{}", Self::EVE_API_URL, path);
            if retry_counter == 3 {
                log::error!("Too many retries requesting {}.", url);
                return Err(EveConnectError::TooManyRetries(url));
            }

            let response = self.0
                .post(&url)
                .json(body)
                .bearer_auth(token)
                .send()
                .await;
            let response = response.map_err(EveConnectError::ReqwestError)?;

            if response.status() == StatusCode::UNAUTHORIZED ||
               response.status() == StatusCode::FORBIDDEN {
                return Err(EveConnectError::Unauthorized);
            }

            // status 200 and 404 are ok
            if response.status() != StatusCode::OK &&
               response.status() != StatusCode::NOT_FOUND {
                retry_counter += 1;
                log::error!(
                    "Fetch resulted in non 200 or 404 status code. Statuscode was {}. Retrying.",
                    response.status()
                );
                continue;
           } else {
               return response.json().await.map_err(Into::into);
           }
        }
    }

    fn page_count(&self, response: &Response) -> u8 {
        let headers = response.headers();
        if let Some(x) = headers.get("x-pages") {
            x.to_str()
                .unwrap_or_default()
                .parse::<u8>()
                .unwrap_or_default()
        } else {
            0u8
        }
    }
}

fn scope() -> String {
    vec![
        "publicData",
        "esi-assets.read_assets.v1",
        "esi-characters.read_agents_research.v1",
        "esi-characters.read_blueprints.v1",
        "esi-characterstats.read.v1",
        "esi-fittings.read_fittings.v1",
        "esi-fittings.write_fittings.v1",
        "esi-industry.read_character_jobs.v1",
        "esi-industry.read_character_mining.v1",
        "esi-markets.read_character_orders.v1",
        "esi-markets.structure_markets.v1",
        "esi-planets.manage_planets.v1",
        "esi-search.search_structures.v1",
        "esi-skills.read_skillqueue.v1",
        "esi-skills.read_skills.v1",
        "esi-universe.read_structures.v1",
        "esi-wallet.read_character_wallet.v1",
    ]
    .join(" ")
}

#[derive(Debug, Deserialize)]
pub struct EveOAuthPayload {
    pub sub: String,
    pub name: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct EveOAuthToken {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
    pub refresh_token: String
}

impl EveOAuthToken {
    pub fn payload(&self) -> Result<EveOAuthPayload, EveConnectError> {
        let payload = self.access_token.to_string();
        let payload = payload.split('.').collect::<Vec<_>>();
        let payload = payload.get(1).copied().unwrap_or_default();
        let decoded = base64::decode(payload)
            .map_err(|_| EveConnectError::OAuthPayload("Failed to decode base64".into()))?;
        serde_json::from_slice(&decoded).map_err(Into::into)
    }
}

#[derive(Clone, Debug)]
pub struct EveOAuthUser {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: u32,
}

impl From<EveOAuthToken> for EveOAuthUser {
    fn from(x: EveOAuthToken) -> Self {
        Self {
            access_token: x.access_token.clone(),
            refresh_token: x.refresh_token.clone(),
            user_id: x.payload().unwrap().sub.replace("CHARACTER:EVE:", "").parse().unwrap_or_default(),
        }
    }
}

#[derive(Serialize)]
struct FormBody {
    grant_type: String,
    code: String,
}
