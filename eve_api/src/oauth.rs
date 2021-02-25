use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

const EVE_LOGIN_URL:  &'static str = "https://login.eveonline.com/v2/oauth/authorize";
const EVE_TOKEN_URL:  &'static str = "https://login.eveonline.com/v2/oauth/token";
const ENV_REDIRECT:   &'static str = "EVE_REDIRECT_URL";
const ENV_CLIENT_ID:  &'static str = "EVE_CLIENT_ID";
const ENV_SECRET_KEY: &'static str = "EVE_SECRET_KEY";

pub fn eve_auth_uri() -> Result<Url, Box<dyn std::error::Error>> {
    let mut url = Url::parse(EVE_LOGIN_URL).unwrap();

    let redirect_uri = std::env::var(ENV_REDIRECT)?;
    let client_id = std::env::var(ENV_CLIENT_ID)?;

    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("client_id", &client_id)
        .append_pair("scope", &scope())
        .append_pair("state", "TODO:secure");
    Ok(url)
}

// https://docs.esi.evetech.net/docs/sso/web_based_sso_flow.html
pub async fn retrieve_authorization_token(code: &str) -> Result<EveOAuthToken, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("grant_type", "authorization_code");
    map.insert("code", code);

    send(map).await
}

// https://docs.esi.evetech.net/docs/sso/refreshing_access_tokens.html
pub async fn retrieve_refresh_token(refresh_token: &str) -> Result<EveOAuthToken, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("grant_type", "refresh_token");
    map.insert("refresh_token", refresh_token);

    send(map).await
}

async fn send<T: Serialize>(form: T) -> Result<EveOAuthToken, Box<dyn std::error::Error>> {
    let client_id = std::env::var(ENV_CLIENT_ID)?;
    let secret_key = std::env::var(ENV_SECRET_KEY)?;

    Client::new()
        .post(EVE_TOKEN_URL)
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

fn scope() -> String {
    vec![
        "publicData",
        "esi-assets.read_assets.v1",
        "esi-characters.read_agents_research.v1",
        "esi-characters.read_blueprints.v1",
        "esi-characterstats.read.v1",
        "esi-corporations.read_structures.v1",
        "esi-fittings.read_fittings.v1",
        "esi-fittings.write_fittings.v1",
        "esi-industry.read_character_jobs.v1",
        "esi-industry.read_character_mining.v1",
        "esi-location.read_location.v1",
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
    pub access_token: serde_json::Value,
    pub expires_in: u32,
    pub token_type: String,
    pub refresh_token: String
}

impl EveOAuthToken {
    pub fn payload(&self) -> Result<EveOAuthPayload, Box<dyn std::error::Error>> {
        let payload = self.access_token.to_string();
        let payload = payload.split('.').collect::<Vec<_>>();
        let payload = payload.get(1).map(|x| *x).unwrap_or_default();
        let decoded = base64::decode(payload)?;
        serde_json::from_slice(&decoded).map_err(Into::into)
    }
}

#[derive(Serialize)]
struct FormBody {
    grant_type: String,
    code: String,
}
