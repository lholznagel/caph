use crate::error::ServerError;

use async_trait::*;
use axum::{extract::{FromRequest, Extension, RequestParts}, http::Uri};
use caph_connector::{CharacterId, EveAuthClient, EveOAuthToken};
use sqlx::PgPool;
use uuid::Uuid;

/// Scope for a normal character without any features actived
/*const EVE_DEFAULT_SCOPE: &[&'static str] = &[
    "publicData",
];*/

#[deprecated]
const EVE_SCOPE: &[&'static str] = &[
    "publicData",
    "esi-assets.read_assets.v1",
    //"esi-characters.read_agents_research.v1",
    "esi-characters.read_blueprints.v1",
    "esi-characterstats.read.v1",
    "esi-industry.read_character_jobs.v1",
    "esi-industry.read_corporation_jobs.v1",
    //"esi-industry.read_character_mining.v1",
    //"esi-markets.read_character_orders.v1",
    //"esi-markets.structure_markets.v1",
    //"esi-planets.manage_planets.v1",
    //"esi-search.search_structures.v1",
    //"esi-skills.read_skillqueue.v1",
    //"esi-skills.read_skills.v1",
    //"esi-universe.read_structures.v1",
    //"esi-wallet.read_character_wallet.v1",
];

/// Handles authentication and authorisation.
///
/// The authentication is connected with the official EVE-API.
/// Authorisation is configured and handled by this application.
#[derive(Clone)]
pub struct AuthService {
    pool: PgPool
}

impl AuthService {
    /// Creates a new service instance.
    ///
    /// # Params
    ///
    /// * `pool` -> Connection pool to postgres
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
    pub async fn auth(
        &self,
        code:  &str,
        token: Uuid
    ) -> Result<Option<Uuid>, ServerError> {
        let character = EveAuthClient::access_token(&code).await?;
        self.save_login(&token, character).await?;

        if self.is_alt(&token).await? {
            Ok(None)
        } else {
            Ok(Some(token))
        }
    }

    /// Creates a new login process for the EVE-API.
    /// This function is for login in a user without any activated features, it
    /// therefore only requires the most basic permissions.
    /// 
    /// For extending a users permissions the function [AuthService::xyz] FIXME:
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
    pub async fn login(&self) -> Result<Uri, ServerError> {
        let token = sqlx::query!(
            "INSERT INTO login DEFAULT VALUES RETURNING token"
        )
        .fetch_one(&self.pool)
        .await?
        .token;

        EveAuthClient::auth_uri(
                &token.to_string(),
                Some(&EVE_SCOPE.join(" "))
            )?
            .to_string()
            .parse::<Uri>()
            .map_err(|x| ServerError::GenericError(x.to_string()))
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
    pub async fn login_alt(
        &self,
        cid: CharacterId
    ) -> Result<Uri, ServerError> {
        let token = sqlx::query!("
                INSERT INTO login (character_main)
                VALUES ($1)
                RETURNING token
            ", *cid)
            .fetch_one(&self.pool)
            .await?
            .token;

        EveAuthClient::auth_uri(
                &token.to_string(),
                Some(&EVE_SCOPE.join(" "))
            )?
            .to_string()
            .parse::<Uri>()
            .map_err(|x| ServerError::GenericError(x.to_string()))
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
    pub async fn alts(
        &self,
        cid: CharacterId
    ) -> Result<Vec<CharacterId>, ServerError> {
        let mut alts = sqlx::query!("
                SELECT DISTINCT character_id
                FROM login
                WHERE character_main = $1 AND character_id IS NOT NULL
            ", *cid as i32)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| x.character_id)
            .map(|x| x.unwrap().into())
            .collect::<Vec<CharacterId>>();
        alts.push(cid);
        Ok(alts)
    }

    pub async fn refresh_token(
        &self,
        cid:   &CharacterId,
        token: &Uuid,
    ) -> Result<String, ServerError> {
        let refresh_token = sqlx::query!("
                SELECT refresh_token
                FROM login
                WHERE character_id = $1
                  AND token        = $2
            ", **cid, token)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ServerError::InvalidUser)?
            .refresh_token
            .ok_or(ServerError::InvalidUser)?;
        Ok(refresh_token)
    }

    pub async fn is_admin(
        &self,
        cid: CharacterId
    ) -> Result<bool, ServerError> {
        if let Some(x) = sqlx::query!("
                SELECT admin
                FROM   character
                WHERE  character_id = $1
            ",
                *cid
            )
            .fetch_optional(&self.pool)
            .await? {

            Ok(x.admin)
        } else {
            Ok(false)
        }
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
    pub async fn character_id(
        &self,
        token: &Uuid
    ) -> Result<CharacterId, ServerError> {
        let character = sqlx::query!("
                SELECT character_id
                FROM login
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
                Err(ServerError::InvalidUser)
            }
        } else {
            Err(ServerError::InvalidUser)
        }
    }

    /// Saves the main character in the database
    ///
    /// # Params
    ///
    /// `token`     -> Token for identifying the character
    /// `character` -> Character with access_token and refresh_token
    ///
    async fn save_login(
        &self,
        token:     &Uuid,
        character: EveOAuthToken
    ) -> Result<(), ServerError> {
        let character_id = character.character_id()?;

        sqlx::query!("
            DELETE FROM login WHERE character_id = $1
        ", *character_id)
        .execute(&self.pool)
        .await?;

        sqlx::query!("
                UPDATE login
                SET
                    character_id = $1,
                    access_token = $2,
                    refresh_token = $3,
                    expire_date = NOW() + interval '1199' second
                WHERE token = $4
            ",
            *character_id,
            character.access_token,
            character.refresh_token,
            token
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ServerError::InvalidUser)?;

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
    async fn is_alt(
        &self,
        token: &Uuid
    ) -> Result<bool, ServerError> {
        let is_alt = sqlx::query!("
                SELECT character_main
                FROM login
                WHERE token = $1
            ", token)
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
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Extension::<AuthService>::from_request(req)
            .await
            .map(|Extension(x)| x)
            .map_err(ServerError::FromReqestError)
    }
}
