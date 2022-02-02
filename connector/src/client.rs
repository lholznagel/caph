use crate::CharacterId;

use async_trait::*;
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use url::Url;

use crate::error::ConnectError;

/// Trait for implementing new clients that interact with an API
///
#[async_trait]
pub trait RequestClient {
    /// Makes a single request to the given path and returns parses the result
    /// the given struct.
    ///
    /// # Params
    ///
    /// * `T`    -> Model that represents the resulting json
    /// * `path` -> Path of the request
    ///
    /// # Errors
    ///
    /// Returns an error if eiher the request failed or the parsing failed
    ///
    /// # Returns
    ///
    /// Parsed json data
    ///
    async fn fetch<T>(
        &self,
        path: &str,
    ) -> Result<T, ConnectError>
    where T: DeserializeOwned;

    /// Makes requests as long as there are pages to fetch.
    ///
    /// # Params
    ///
    /// * `T`    -> Model that represents the resulting json
    /// * `path` -> Path of the request
    ///
    /// # Errors
    ///
    /// Returns an error if eiher the request failed or the parsing failed.
    /// The error is returned the first time an error is encountered.
    ///
    /// # Returns
    ///
    /// Vector of parsed json
    ///
    async fn fetch_page<T>(
        &self,
        path: &str,
    ) -> Result<Vec<T>, ConnectError>
    where T: DeserializeOwned + Send;

    /// Makes a post request to the given path and returns parses the result
    /// the given struct.
    ///
    /// # Params
    ///
    /// * `T`    -> Model that represents the resulting json
    /// * `data` -> Request model
    /// * `path` -> Path of the request
    ///
    /// # Errors
    ///
    /// Returns an error if eiher the request failed or the parsing failed
    ///
    /// # Returns
    ///
    /// Parsed json data
    ///
    async fn post<R, T>(
        &self,
        data: R,
        path: &str,
    ) -> Result<T, ConnectError>
    where
        R: Debug + Serialize + Send + Sync,
        T: DeserializeOwned;
}

/// Client for communicating with the EVE-API.
///
/// # Required ENV
///
/// If not all required ENVs are set, an error will be returned
///
/// * `EVE_USER_AGENT` -> Name of the user agent that is send with every request
///
#[derive(Clone)]
pub struct EveClient(Client);

impl EveClient {
    /// URL to the EVE-API
    const EVE_API_URL:    &'static str = "https://esi.evetech.net";
    /// Name of the ENV of the user agent
    const ENV_USER_AGENT: &'static str = "EVE_USER_AGENT";

    /// Consutructs a new [EveClient].
    ///
    /// # Requirements
    ///
    /// The ENV `EVE_USER_AGENT` must be set.
    ///
    /// # Errors
    ///
    /// The function will return an error if the ENV `EVE_USER_AGENT` is not set.
    /// Besides that it will return an error if the client could not be
    /// constructed.
    /// See the returned error for more details.
    ///
    /// # Returns
    ///
    /// New instance of the [EveClient]
    ///
    pub fn new() -> Result<Self, ConnectError> {
        let user_agent = std::env::var(Self::ENV_USER_AGENT)
            .map_err(|_| ConnectError::env_user_agent())?;

        let client = Client::builder()
            .user_agent(user_agent)
            .build()
            .map_err(ConnectError::CouldNotConstructClient)?;

        Ok(Self(client))
    }

    /// Deconstructs the struct and returns the underlying [reqwest::Client].
    ///
    /// # Returns
    ///
    /// Underlying [reqwest::Client]
    ///
    pub fn into_inner(self) -> Client {
        self.0
    }

    /// Sends a request to the given path.
    ///
    /// If a request failes with a non successfull status, it will retry the
    /// request again, up to 3 times.
    ///
    /// # Params
    ///
    /// * `path` -> Path for the request
    ///
    /// # Errors
    ///
    /// The function errors if too many request failed, or if there is a general
    /// error with the requesting library.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    #[tracing::instrument(level = "debug")]
    async fn send(&self, path: &str) -> Result<Response, ConnectError> {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let mut retry_counter = 0usize;

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", &path);
                return Err(ConnectError::TooManyRetries(path));
            }

            let response = self.0
                .get(&path)
                .send()
                .await
                .map_err(ConnectError::ReqwestError)?;

            if !response.status().is_success() {
                retry_counter += 1;
                tracing::warn!(
                    r#"Fetch resulted in non successful status code.
                       Statuscode was {}. Retrying {}."#,
                    response.status(),
                    retry_counter
                );
                continue;
            }

            return Ok(response)
        }
    }

    /// Extract the page header from the give [reqwest::Response].
    ///
    /// # Params
    ///
    /// * `response` -> Respone to get the header from
    ///
    /// # Returns
    ///
    /// - If the header is not present a 0 is returned
    /// - If the header exists, it will try to parse it, if that fails a 0 is
    ///   is returned
    ///
    #[tracing::instrument(level = "debug")]
    fn page_count(&self, response: &Response) -> u16 {
        let headers = response.headers();
        if let Some(x) = headers.get("x-pages") {
            x.to_str()
                .unwrap_or_default()
                .parse::<u16>()
                .unwrap_or_default()
        } else {
            0u16
        }
    }
}

#[async_trait]
impl RequestClient for EveClient {
    #[tracing::instrument(level = "debug")]
    async fn fetch<T>(
        &self,
        path: &str,
    ) -> Result<T, ConnectError>
    where T: DeserializeOwned {
        let json = self.send(path)
            .await?
            .json::<T>()
            .await
            .map_err(ConnectError::ReqwestError)?;
        Ok(json)
    }

    #[tracing::instrument(level = "debug")]
    async fn fetch_page<T>(
        &self,
        path: &str,
    ) -> Result<Vec<T>, ConnectError>
    where T: DeserializeOwned + Send {
        let response = self
            .send(path)
            .await?;

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        let data = response
            .json::<Vec<T>>()
            .await
            .map_err(ConnectError::ReqwestError)?;
        fetched_data.extend(data);

        for page in 2..=pages {
            let next_page = self
                .send(&format!(
                    "{}?page={}",
                    path,
                    page
                ))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(ConnectError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(fetched_data)
    }

    async fn post<R, T>(
        &self,
        _data: R,
        _path: &str,
    ) -> Result<T, ConnectError>
    where
        R: Debug + Serialize + Send + Sync,
        T: DeserializeOwned {

        unimplemented!()
    }
}

impl std::fmt::Debug for EveClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EveClient")
         .finish()
    }
}

/// Client for communicating with the EVE-API using an authenticated character.
///
/// After constructing it provides helper functions for performing a
/// character authentication against the EVE-API.
///
/// # Usage
///
/// Every application should only create a single instance of this struct.
///
/// The client takes the `refresh_token` provided by the EVE-API after login
/// and takes care that there is a valid `access_token`.
///
/// # Required ENV
///
/// If not all required ENVs are set, an error will be returned.
/// All values for the ENV can be found
/// [here](https://developers.eveonline.com/applications)
///
/// * `EVE_USER_AGENT` -> Name of the user agent that is send with every request
/// * `EVE_CALLBACK`   -> Url to callback after authentication
/// * `ÈVE_CLIENT_ID`  -> Client ID of the application
/// * `EVE_SECRET_KEY` -> Secret key of the application
///
#[derive(Clone)]
pub struct EveAuthClient {
    /// Client for communicating with EVE
    client:        Client,
    /// Token to get a new `access_token`
    refresh_token: String,
    /// Token needed to get data that is behind auth
    access_token:  Arc<Mutex<Option<String>>>,
}

impl EveAuthClient {
    /// URL to the EVE-API
    const EVE_API_URL:    &'static str = "https://esi.evetech.net";
    /// URL to the EVE-API oauth login page
    const EVE_LOGIN_URL:  &'static str = "https://login.eveonline.com/v2/oauth/authorize";
    /// URL to the EVE-API oauth token
    const EVE_TOKEN_URL:  &'static str = "https://login.eveonline.com/v2/oauth/token";
    /// Name of the ENV of the application callback
    const ENV_CALLBACK:   &'static str = "EVE_CALLBACK";
    /// Name of the ENV of the application client id
    const ENV_CLIENT_ID:  &'static str = "EVE_CLIENT_ID";
    /// Name of the ENV of the application secret key
    const ENV_SECRET_KEY: &'static str = "EVE_SECRET_KEY";
    /// Name of the ENV of the user agent
    const ENV_USER_AGENT: &'static str = "EVE_USER_AGENT";
    /// Default scope that is used
    const DEFAULT_SCOPE:  &'static str = "publicData";

    /// Gets the initial access token,
    ///
    /// [More information](https://docs.esi.evetech.net/docs/sso/web_based_sso_flow.html)
    ///
    /// # Params
    ///
    /// * `code` -> Code send by the EVE-API as query parameter
    ///
    /// # Panics
    ///
    /// Panics if the [Mutex] is not exclusive.
    ///
    /// # Errors
    ///
    /// If the retrieving of an `access_token` fails the function will return
    /// an error
    ///
    pub async fn access_token(
        code: &str
    ) -> Result<EveOAuthToken, ConnectError> {
        let mut map = HashMap::new();
        map.insert("grant_type", "authorization_code");
        map.insert("code", code);

        let token = Self::get_token(map).await?;
        Ok(token)
    }

    /// Makes a request to the token interface and sets necessary headers to
    /// retrieve a new `access_token`.
    ///
    /// # Params
    ///
    /// * `form` -> Form containing `grant_type` and `code` or `refres_token`.
    ///             See the EVE SSO-Flow documentation for more information
    ///
    /// # Errors
    ///
    /// If the request fails
    ///
    /// # Returns
    ///
    /// New token object
    ///
    async fn get_token(
        form: HashMap<&str, &str>
    ) -> Result<EveOAuthToken, ConnectError> {
        let client_id = std::env::var(Self::ENV_CLIENT_ID)
            .map_err(|_| ConnectError::env_client_id())?;
        let secret_key = std::env::var(Self::ENV_SECRET_KEY)
            .map_err(|_| ConnectError::env_secret_key())?;

        Client::new()
            .post(Self::EVE_TOKEN_URL)
            .basic_auth(client_id, Some(secret_key))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Host", "login.eveonline.com")
            .form(&form)
            .send()
            .await
            .map_err(ConnectError::ReqwestError)?
            .json::<EveOAuthToken>()
            .await
            .map_err(ConnectError::ReqwestError)
    }

    /// Consutructs a new [EveAuthClient].
    ///
    /// # Requirements
    ///
    /// The ENV `EVE_USER_AGENT` must be set.
    ///
    /// # Params
    ///
    /// * `refresh_token` -> Refresh token from the EVE-API
    ///
    /// # Errors
    ///
    /// The function will return an error if the ENV `EVE_USER_AGENT` is not set.
    /// Besides that it will return an error if the client could not be
    /// constructed.
    /// See the returned error for more details.
    ///
    /// # Returns
    ///
    /// New instance of the [EveAuthClient]
    ///
    pub fn new(
        refresh_token: String
    ) -> Result<Self, ConnectError> {
        let user_agent = std::env::var(Self::ENV_USER_AGENT)
            .map_err(|_| ConnectError::env_user_agent())?;

        let client = Client::builder()
            .user_agent(user_agent)
            .pool_idle_timeout(None)
            .build()
            .map_err(ConnectError::CouldNotConstructClient)?;

        Ok(Self {
            client:        client,
            refresh_token: refresh_token,
            access_token:  Arc::new(Mutex::new(None))
        })
    }

    /// Consutructs a new [EveAuthClient] with an existing `access_token`.
    ///
    /// # Requirements
    ///
    /// The ENV `EVE_USER_AGENT` must be set.
    ///
    /// # Params
    ///
    /// * `access_token`  -> Access token fromt he EVE-API
    /// * `refresh_token` -> Refresh token from the EVE-API
    ///
    /// # Panics
    ///
    /// Panics if the [Mutex] is not exclusive.
    ///
    /// # Errors
    ///
    /// The function will return an error if the ENV `EVE_USER_AGENT` is not set.
    /// Besides that it will return an error if the client could not be
    /// constructed.
    /// See the returned error for more details.
    ///
    /// # Returns
    ///
    /// New instance of the [EveAuthClient]
    ///
    #[allow(clippy::unwrap_in_result)]
    pub fn with_access_token(
        access_token:  String,
        refresh_token: String
    ) -> Result<Self, ConnectError> {
        let s = Self::new(refresh_token)?;
        #[allow(clippy::unwrap_used)]
        {
            *s.access_token.lock().unwrap() = Some(access_token);
        }
        Ok(s)
    }

    /// Generates a url for authenticationg a character against the EVE-API.
    ///
    /// # Params
    ///
    /// * `state` -> Unique key, used for extra security
    /// * `scope` -> Required scope, musst be a lost of space seperated entries
    ///
    /// # Errors
    ///
    /// The function will return an error if either the ENV `EVE_CALLBACK`,
    /// the ENV `EVE_CLIENT_ID` or ENV `EVE_CALLBACK` are not set.
    ///
    /// # Usage
    ///
    /// ``` rust
    /// use caph_connector::*;
    /// # std::env::set_var("EVE_CALLBACK", "");
    /// # std::env::set_var("EVE_CLIENT_ID", "");
    /// # std::env::set_var("EVE_SECRET_KEY", "");
    ///
    /// let state = "my_unique_state";
    /// let url = EveAuthClient::auth_uri(state, None).unwrap();
    ///
    /// // redirect user to the returned url
    /// ```
    ///
    #[tracing::instrument(level = "debug")]
    pub fn auth_uri(state: &str, scope: Option<&str>) -> Result<Url, ConnectError> {
        let scope = if let Some(x) = scope {
            x
        } else {
            Self::DEFAULT_SCOPE
        };

        let mut url = Url::parse(Self::EVE_LOGIN_URL)
            .map_err(|_| ConnectError::UrlParseError)?;

        let callback = std::env::var(Self::ENV_CALLBACK)
            .map_err(|_| ConnectError::env_callback())?;
        let client_id = std::env::var(Self::ENV_CLIENT_ID)
            .map_err(|_| ConnectError::env_client_id())?;
        let _ = std::env::var(Self::ENV_SECRET_KEY)
            .map_err(|_| ConnectError::env_secret_key())?;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &callback)
            .append_pair("client_id", &client_id)
            .append_pair("scope", scope)
            .append_pair("state", state);
        Ok(url)
    }

    /// Gets a new `access_token` using the `refresh_token`.
    ///
    /// [More information](https://docs.esi.evetech.net/docs/sso/refreshing_access_tokens.html)
    ///
    /// # Errors
    ///
    /// If the retrieving of an `access_token` fails the function will return
    /// an error
    ///
    async fn refresh_token(&self) -> Result<(), ConnectError> {
        let mut map = HashMap::new();
        map.insert("grant_type", "refresh_token");
        map.insert("refresh_token", &self.refresh_token);

        let token = Self::get_token(map).await?;

        #[allow(clippy::unwrap_used)]
        {
            *self.access_token.lock().unwrap() = Some(token.access_token);
        }

        Ok(())
    }

    /// Sends a GET request to the given path setting the current `access_token`
    /// as `bearer_auth`.
    ///
    /// If a request failes with a non successfull status, it will retry the
    /// request again, up to 3 times.
    ///
    /// # Params
    ///
    /// * `path` -> Path for the request
    ///
    /// # Errors
    ///
    /// The function errors if too many request failed, or if there is a general
    /// error with the requesting library.
    ///
    /// If the EVE-API returns [StatusCode::UNAUTHORIZED] it will attempt to
    /// retriev a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    #[tracing::instrument(level = "debug")]
    async fn send(&self, path: &str) -> Result<Response, ConnectError> {
        let access_token = {
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        };
        let access_token = if access_token.is_none() {
            self.refresh_token().await?;
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        } else {
            access_token
        };

        let mut retry_counter = 0usize;

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", path);
                return Err(ConnectError::TooManyRetries(path.into()));
            }

            let token = access_token
                .as_ref()
                .expect("We check but somehow the access_token is still None");
            let response = self
                .client
                .get(path)
                .bearer_auth(token)
                .send()
                .await
                .map_err(ConnectError::ReqwestError)?;

            if response.status() == StatusCode::UNAUTHORIZED {
                self.refresh_token().await?;
                continue;
            }

            if !response.status().is_success() {
                retry_counter += 1;
                tracing::error!(
                    { retry = retry_counter, status = response.status().as_u16() },
                    "Fetch resulted in non successful status code.",
                );
                continue;
            }

            return Ok(response)
        }
    }

    /// Sends a POST request to the given path setting the current
    /// `access_token` as `bearer_auth`.
    ///
    /// If a request failes with a non successfull status, it will retry the
    /// request again, up to 3 times.
    ///
    /// # Params
    ///
    /// * `data` -> Data to send in the body
    /// * `path` -> Path for the request
    ///
    /// # Errors
    ///
    /// The function errors if too many request failed, or if there is a general
    /// error with the requesting library.
    ///
    /// If the EVE-API returns [StatusCode::UNAUTHORIZED] it will attempt to
    /// retriev a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    #[tracing::instrument(level = "debug")]
    async fn send_post<R>(
        &self,
        data: R,
        path: &str
    ) -> Result<Response, ConnectError>
    where
        R: Debug + Serialize + Send + Sync {

        let access_token = {
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        };
        let access_token = if access_token.is_none() {
            self.refresh_token().await?;
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        } else {
            access_token
        };

        let mut retry_counter = 0usize;

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", path);
                return Err(ConnectError::TooManyRetries(path.into()));
            }

            let token = access_token
                .as_ref()
                .expect("We check but somehow the access_token is still None");
            let response = self
                .client
                .post(path)
                .json(&data)
                .bearer_auth(token)
                .send()
                .await
                .map_err(ConnectError::ReqwestError)?;

            if response.status() == StatusCode::UNAUTHORIZED {
                self.refresh_token().await?;
                continue;
            }

            if !response.status().is_success() {
                retry_counter += 1;
                tracing::error!(
                    { retry = retry_counter, status = response.status().as_u16() },
                    "Fetch resulted in non successful status code.",
                );
                continue;
            }

            return Ok(response)
        }
    }

    /// Extract the page header from the give [reqwest::Response].
    ///
    /// # Params
    ///
    /// * `response` -> Respone to get the header from
    ///
    /// # Returns
    ///
    /// - If the header is not present a 0 is returned
    /// - If the header exists, it will try to parse it, if that fails a 0 is
    ///   is returned
    ///
    #[tracing::instrument(level = "debug")]
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

impl std::fmt::Debug for EveAuthClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EveAuthClient")
         .finish()
    }
}

#[async_trait]
impl RequestClient for EveAuthClient {
    #[tracing::instrument(level = "debug")]
    async fn fetch<T>(
        &self,
        path: &str,
    ) -> Result<T, ConnectError>
    where T: DeserializeOwned {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let json = self.send(&path)
            .await?
            .json::<T>()
            .await
            .map_err(ConnectError::ReqwestError)?;
        Ok(json)
    }

    #[tracing::instrument(level = "debug")]
    async fn fetch_page<T>(
        &self,
        path: &str,
    ) -> Result<Vec<T>, ConnectError>
    where T: DeserializeOwned + Send {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let response = self
            .send(&path)
            .await?;

        let pages = self.page_count(&response);

        let mut fetched_data: Vec<T> = Vec::new();
        let data = response
            .json::<Vec<T>>()
            .await
            .map_err(ConnectError::ReqwestError)?;
        fetched_data.extend(data);

        for page in 2..=pages {
            let next_page = self
                .send(&format!(
                    "{}?page={}",
                    path,
                    page
                ))
                .await?
                .json::<Vec<T>>()
                .await
                .map_err(ConnectError::ReqwestError)?;

            fetched_data.extend(next_page);
        }

        Ok(fetched_data)
    }

    #[tracing::instrument(level = "debug")]
    async fn post<R, T>(
        &self,
        data: R,
        path: &str,
    ) -> Result<T, ConnectError>
    where
        R: Debug + Serialize + Send + Sync,
        T: DeserializeOwned {

        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let json = self.send_post(data, &path)
            .await?
            .json::<T>()
            .await
            .map_err(ConnectError::ReqwestError)?;
        Ok(json)
    }
}

/// Decoded access token
#[derive(Debug, Deserialize)]
pub struct EveOAuthPayload {
    /// List of all permissions that where granted
    pub scp: Scp,
    /// User identification
    pub sub: String,
}

/// Parses the scp field in the payload
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Scp {
    /// The permission is a single String
    Str(String),
    /// The permission is a list of strings
    StrArray(Vec<String>)
}

impl Scp {
    /// Gets the inner string or vec of string and returns it as vec
    pub fn into_vec(self) -> Vec<String> {
        match self {
            Self::Str(x)      => vec![x],
            Self::StrArray(x) => x
        }
    }
}

/// Parsed version of the response from EVE after a successfull login.
///
#[derive(Debug, Deserialize)]
pub struct EveOAuthToken {
    /// Access token required for every request
    pub access_token:  String,
    /// Type of the token
    pub token_type:    String,
    /// Lifetime of the token, typically 1199 seconds
    pub expires_in:    i32,
    /// Token to get a new access token
    pub refresh_token: String,
}

impl EveOAuthToken {
    /// Extracts the payload from the access token
    ///
    /// # Errors
    ///
    /// Fails when the payload could not be decoded or parsed
    ///
    /// # Returns
    ///
    /// Payload of the access token
    ///
    pub fn payload(&self) -> Result<EveOAuthPayload, ConnectError> {
        let payload = self.access_token.to_string();
        let payload = payload.split('.').collect::<Vec<_>>();
        let payload = payload.get(1).copied().unwrap_or_default();
        let decoded = base64::decode(payload)
            .map_err(ConnectError::OAuthPayloadDecode)?;
        serde_json::from_slice(&decoded)
            .map_err(ConnectError::OAuthParseError)
    }

    /// Gets the character id from the token
    ///
    /// # Errors
    ///
    /// Fails when either getting the payload fails or the user identification
    /// could not be parsed
    ///
    /// # Returns
    ///
    /// The character id
    ///
    pub fn character_id(&self) -> Result<CharacterId, ConnectError> {
        let character_id = self.payload()?
            .sub
            .replace("CHARACTER:EVE:", "")
            .parse::<i32>()
            .map_err(ConnectError::OAuthParseCharacterId)?;
        Ok(character_id.into())
    }
}
