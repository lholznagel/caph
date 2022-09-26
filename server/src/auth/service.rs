use crate::{error::Error, ProjectId};

use async_trait::*;
use axum::extract::{FromRequest, Extension, RequestParts};
use caph_connector::{CharacterId, EveAuthClient, EveOAuthToken};
use sqlx::PgPool;
use tracing::instrument;
use serde::Deserialize;
use serde_json::json;

const ESI_PUBLIC_DATA:                    &str = "publicData";
const ESI_UNIVERSE_STRUCTURES :           &str = "esi-universe.read_structures.v1";

const ESI_READ_BLUEPRINTS:                &str = "esi-characters.read_blueprints.v1";
const ESI_READ_CORPORATION_BLUEPRINTS:    &str = "esi-corporations.read_blueprints.v1";

pub const ESI_READ_ASSETS:                    &str = "esi-assets.read_assets.v1";
pub const ESI_READ_CORPORATION_ASSETS:        &str = "esi-assets.read_corporation_assets.v1";

pub const ESI_READ_INDUSTRY_JOBS:             &str = "esi-industry.read_character_jobs.v1";
pub const ESI_READ_CORPORATION_INDUSTRY_JOBS: &str = "esi-corporations.read_blueprints.v1";

pub const ESI_DEFAULT_SCOPE: &[&str] = &[
    ESI_PUBLIC_DATA
];

const ESI_ASSET_SCOPE: &[&str] = &[
    ESI_READ_ASSETS,
    ESI_READ_BLUEPRINTS,
    ESI_UNIVERSE_STRUCTURES,
];

const ESI_CORPORATION_ASSET_SCOPE: &[&str] = &[
    ESI_READ_CORPORATION_ASSETS,
    ESI_READ_CORPORATION_BLUEPRINTS,
];

const ESI_ALL: &[&str] = &[
    ESI_PUBLIC_DATA,
    ESI_UNIVERSE_STRUCTURES,
    ESI_READ_BLUEPRINTS,
    ESI_READ_CORPORATION_BLUEPRINTS,
    ESI_READ_ASSETS,
    ESI_READ_CORPORATION_ASSETS,
    ESI_READ_INDUSTRY_JOBS,
    ESI_READ_CORPORATION_INDUSTRY_JOBS,
];

/// Handles authentication and authorisation.
///
/// The authentication is connected with the official EVE-API.
/// Authorisation is configured and handled by this service.
#[derive(Clone, Debug)]
pub struct AuthService {
    pool: PgPool
}

impl AuthService {
    /// Creates a new service instance.
    ///
    /// # Params
    ///
    /// * `pool` -> Connection pool to postgres&str
    ///
    /// # Returns
    ///
    /// New instance of the [AuthService].
    ///
    pub fn new(
        pool: PgPool
    ) -> Self {
        Self {
            pool
        }
    }

    /// Performs the last eve auth step
    ///
    /// # Params
    ///
    /// `code`  -> Code that was send when starting the auth process
    /// `token` -> Our unique identifier
    ///
    /// # Errors
    ///
    /// When no access token can be received from the EVE-API.
    ///
    /// # Returns
    ///
    /// Returns a token that should be used as a cookie and should be send
    /// with every request as value in the header with the key `token`.
    ///
    #[instrument(err)]
    pub async fn auth(
        &self,
        code:  &str,
        state: String
    ) -> Result<Option<String>, Error> {
        let (token, hash) = crate::utils::generate_secure_token()?;

        let character = EveAuthClient::access_token(code).await?;
        self.save_login(&hash, &state, character).await?;

        if self.is_alt(&hash).await? {
            Ok(None)
        } else {
            Ok(Some(token))
        }
    }

    /// Creates a new login process for the EVE-API.
    /// This function is for login in a user without any activated features, it
    /// therefore only requires the most basic permissions.
    /// 
    /// For extending a users permissions the function [AuthService::add_scope]
    /// must be used.
    /// 
    /// # Errors
    /// 
    /// When the the uri cannot be parsed.
    /// 
    /// # Returns
    /// 
    /// Redirect URI to the EVE-Login page.
    /// 
    #[instrument(err)]
    pub async fn login(
        &self,
        scope: Scope
    ) -> Result<String, Error> {
        let (_, hash) = crate::utils::generate_secure_token()?;

        sqlx::query!("
                INSERT INTO logins (token)
                VALUES ($1)
            ",
                &hash,
        )
        .execute(&self.pool)
        .await?;

        self.login_uri(&hash, scope)
    }

    /// Creates a new unqiue code for logging in an alt character
    ///
    /// # Params
    ///
    /// `token` -> Unique token provided by the cookie
    ///
    /// # Returns
    ///
    /// Uri to the eve auth server
    ///
    #[instrument(err)]
    pub async fn login_alt(
        &self,
        cid:   CharacterId,
        scope: Scope
    ) -> Result<String, Error> {
        let (_, hash) = crate::utils::generate_secure_token()?;
        sqlx::query!("
                INSERT INTO logins (character_main, token)
                VALUES ($1, $2)
            ",
                *cid,
                &hash,
            )
            .execute(&self.pool)
            .await?;

        self.login_uri(&hash, scope)
    }

    fn login_uri(
        &self,
        hash:  &str,
        scope: Scope
    ) -> Result<String, Error> {
        let scope = &ESI_ALL.join(" ");

        Ok(EveAuthClient::auth_uri(
            &hash,
            Some(scope)
        )?
        .to_string())
    }

    #[instrument(err)]
    pub async fn add_scope(
        &self,
        cid:   CharacterId,
        scope: String,
    ) -> Result<String, Error> {
        let scope = Scope::from(scope);
        let (_, hash) = crate::utils::generate_secure_token()?;

        sqlx::query!("
                UPDATE logins
                SET
                    access_token         = NULL,
                    refresh_token        = NULL,
                    token                = $2
                WHERE character_id = $1
            ",
            *cid,
            &hash,
        )
        .execute(&self.pool)
        .await?;

        self.login_uri(&hash, scope)
    }

    /// Gets a list of alts for the given [CharacterId]
    ///
    /// # Params
    ///
    /// * `cid` -> [CharacterId] of the requesting character
    ///
    /// # Returns
    ///
    /// List of [CharacterId] inculding the main and all alts
    ///
    #[instrument(err)]
    pub async fn alts(
        &self,
        cid: CharacterId
    ) -> Result<Vec<CharacterId>, Error> {
        let mut alts = sqlx::query!(r#"
                SELECT DISTINCT character_id AS "character_id!"
                FROM logins
                WHERE character_main = $1 AND character_id IS NOT NULL
            "#, *cid as i32)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| x.character_id)
            .map(|x| x.into())
            .collect::<Vec<CharacterId>>();
        alts.push(cid);
        Ok(alts)
    }

    #[instrument(err)]
    pub async fn refresh_token(
        &self,
        cid: &CharacterId,
    ) -> Result<String, Error> {
        let refresh_token = sqlx::query!("
                SELECT refresh_token
                FROM logins
                WHERE character_id = $1
            ",
                **cid
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::InvalidUser)?
            .refresh_token
            .ok_or(Error::InvalidUser)?;
        Ok(refresh_token)
    }

    /// Gets the character id from the database
    ///
    /// # Parameters
    ///
    /// * `token` -> Token provided by the cookie
    ///
    /// # Returns
    ///
    /// Character id of the logged in character
    ///
    #[instrument(err)]
    pub async fn character_id(
        &self,
        token: &str
    ) -> Result<CharacterId, Error> {
        let character = sqlx::query!("
                SELECT character_id
                FROM logins
                WHERE token = $1
            ",
            token
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(x) = character {
            if let Some(id) = x.character_id {
                Ok(id.into())
            } else {
                Err(Error::InvalidUser)
            }
        } else {
            Err(Error::InvalidUser)
        }
    }

    #[instrument(err)]
    #[deprecated]
    pub async fn has_project_access(
        &self,
        token: &str,
        pid:   ProjectId,
    ) -> Result<bool, Error> {
        let result = sqlx::query!("
                SELECT owner
                FROM projects
                WHERE project = $1
                  AND owner = (
                      SELECT character_id
                      FROM logins
                      WHERE token = $2
                )
            ",
                pid,
                token
            )
            .fetch_optional(&self.pool)
            .await?;
        Ok(result.is_some())
    }

    pub async fn is_permitted(
        &self,
        token: String
    ) -> Result<bool, Error> {
        let result = sqlx::query!("
                SELECT character_id
                FROM logins
                WHERE token = $1
            ",
                token
            )
            .fetch_optional(&self.pool)
            .await?;
        if result.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Gets a list of all scopes that are avaialble withing the application
    pub fn available_scopes(
        &self
    ) -> serde_json::Value {
        json!([
            {
                "key":    "public",
                "name":   "Public data",
                "reason": "Need for general information about the character",
                "scopes": Scope::Public.scopes()
            },
            {
                "key":    "character_assets",
                "name":   "Read assets and blueprints",
                "reason": "Needed for generting a warehouse",
                "scopes": Scope::CharacterAssets.scopes()
            },
            {
                "key":    "corporation_assets",
                "name":   "Read corporation assets and blueprints",
                "reason": "Needed for generting a warehouse",
                "scopes": Scope::CorporationAssets.scopes()
            },
            {
                "key":    "character_industry_jobs",
                "name":   "Read industry jobs",
                "reason": "Required to show a list of active industry jobs",
                "scopes": Scope::CharacterIndustryJobs.scopes()
            },
            {
                "key":    "corporation_industry_jobs",
                "name":   "Read corporation industry jobs",
                "reason": "Required to show a list of active industry jobs",
                "scopes": Scope::CorporationIndustryJobs.scopes()
            }
        ])
    }

    /// Saves the main character in the database
    ///
    /// # Params
    ///
    /// `token`     -> Token for identifying the character
    /// `character` -> Character with access_token and refresh_token
    ///
    #[instrument(err)]
    async fn save_login(
        &self,
        token:     &str,
        state:     &str,
        character: EveOAuthToken
    ) -> Result<(), Error> {
        let character_id = character.character_id()?;
        let scp = character.payload()?.scp.into_vec();

        sqlx::query!("
                UPDATE logins
                SET
                    character_id = $1,
                    refresh_token = $2,
                    access_token = $3,
                    expire_date = NOW() + interval '1199' second,
                    token = $4
                WHERE token = $5
            ",
            *character_id,
            &character.refresh_token,
            &character.access_token,
            token,
            state
        )
        .execute(&self.pool)
        .await
        .map_err(|_| Error::InvalidUser)?;

        sqlx::query!("
                UPDATE characters
                SET esi_tokens = (
                    SELECT array_agg(c) FROM (
                        SELECT DISTINCT UNNEST(esi_tokens || $2::VARCHAR[])
                        FROM characters
                        WHERE character_id = $1
                ) AS dt(c))
                WHERE character_id = $1
            ",
            *character_id,
            &scp,
        )
        .execute(&self.pool)
        .await
        .map_err(|x| {dbg!(x); Error::InvalidUser})?;

        Ok(())
    }

    /// Checks if a new login is an alt or not
    ///
    /// # Params
    ///
    /// `token` -> Unique token to identify the character
    ///
    /// # Returns
    ///
    /// * `true`  -> The character is an alt
    /// * `false` -> The character is not an alt
    ///
    #[instrument(err)]
    async fn is_alt(
        &self,
        token: &str
    ) -> Result<bool, Error> {
        let is_alt = sqlx::query!("
                SELECT character_main
                FROM logins
                WHERE token = $1
            ",
                token
            )
            .fetch_one(&self.pool)
            .await?
            .character_main
            .is_some();
        Ok(is_alt)
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthService
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Extension::<AuthService>::from_request(req)
            .await
            .map(|Extension(x)| x)
            .map_err(Error::FromReqestError)
    }
}

#[derive(Debug, Deserialize)]
pub enum Scope {
    /// Default required scope
    #[serde(rename = "public")]
    Public,
    /// Grants access to characters assets and blueprints
    #[serde(rename = "character_assets")]
    CharacterAssets,
    /// Grants access to corporation assets including blueprints
    #[serde(rename = "corporation_assets")]
    CorporationAssets,
    /// Grants access to corporation industry jobs
    #[serde(rename = "character_industry_jobs")]
    CharacterIndustryJobs,
    /// Grants access to corporation industry jobs
    #[serde(rename = "CorporationIndustryJobs")]
    CorporationIndustryJobs,
}

impl Scope {
    pub fn scopes(&self) -> &[&str] {
        match self {
            Self::Public                  => &[
                "publicData"
            ],
            Self::CharacterAssets         => &[
                "esi-assets.read_assets.v1",
                "esi-characters.read_blueprints.v1"
            ],
            Self::CorporationAssets       => &[
                "esi-assets.read_corporation_assets.v1",
                "esi-corporations.read_blueprints.v1"
            ],
            Self::CharacterIndustryJobs   => &[
                "esi-industry.read_character_jobs.v1"
            ],
            Self::CorporationIndustryJobs => &[
                "esi-industry.read_corporation_jobs.v1"
            ],
            _                             => Self::Public.scopes(),
        }
    }
}

impl From<String> for Scope {
    fn from(x: String) -> Self {
        match x.as_ref() {
            "public"                    => Self::CharacterAssets,
            "character_assets"          => Self::CorporationAssets,
            "corporation_assets"        => Self::CorporationAssets,
            "character_industry_jobs"   => Self::CharacterIndustryJobs,
            "corporation_industry_jobs" => Self::CorporationIndustryJobs,
            _                           => Self::Public
        }
    }
}
