use crate::error::ServerError;

use async_trait::*;
use axum::{extract::{FromRequest, Extension, RequestParts}, http::Uri};
use caph_connector::{CharacterId, EveAuthClient, EveOAuthToken};
use caph_core::ProjectId;
use sqlx::PgPool;
use tracing::instrument;

const EVE_DEFAULT_SCOPE: &[&'static str] = &[
    "publicData",
];

/// Handles authentication and authorisation.
///
/// The authentication is connected with the official EVE-API.
/// Authorisation is configured and handled by this application.
#[derive(Clone, Debug)]
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
    #[instrument(err)]
    pub async fn auth(
        &self,
        code:  &str,
        state: String
    ) -> Result<Option<String>, ServerError> {
        let (token, hash) = crate::utils::generate_secure_token()?;

        let character = EveAuthClient::access_token(&code).await?;
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
    #[instrument(err)]
    pub async fn login(&self) -> Result<Uri, ServerError> {
        let (_, hash) = crate::utils::generate_secure_token()?;

        sqlx::query!(
            "INSERT INTO logins (token) VALUES ($1)",
            &hash
        )
        .execute(&self.pool)
        .await?;

        EveAuthClient::auth_uri(
                &hash,
                Some(&EVE_DEFAULT_SCOPE.join(" "))
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
    #[instrument(err)]
    pub async fn login_alt(
        &self,
        cid: CharacterId
    ) -> Result<Uri, ServerError> {
        let (_, hash) = crate::utils::generate_secure_token()?;
        sqlx::query!("
                INSERT INTO logins (character_main, token)
                VALUES ($1, $2)
            ",
                *cid,
                &hash
            )
            .execute(&self.pool)
            .await?;

        EveAuthClient::auth_uri(
                &hash,
                Some(&EVE_DEFAULT_SCOPE.join(" "))
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
    #[instrument(err)]
    pub async fn alts(
        &self,
        cid: CharacterId
    ) -> Result<Vec<CharacterId>, ServerError> {
        let mut alts = sqlx::query!("
                SELECT DISTINCT character_id
                FROM logins
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

    #[instrument(err)]
    pub async fn refresh_token(
        &self,
        cid: &CharacterId,
    ) -> Result<String, ServerError> {
        let refresh_token = sqlx::query!("
                SELECT refresh_token
                FROM logins
                WHERE character_id = $1
            ",
                **cid
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ServerError::InvalidUser)?
            .refresh_token
            .ok_or(ServerError::InvalidUser)?;
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
    ) -> Result<CharacterId, ServerError> {
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
                Err(ServerError::InvalidUser)
            }
        } else {
            Err(ServerError::InvalidUser)
        }
    }

    #[instrument(err)]
    pub async fn has_project_access(
        &self,
        token: &str,
        pid:   ProjectId,
    ) -> Result<bool, ServerError> {
        let result = sqlx::query!("
                SELECT character_id
                FROM project_members
                WHERE project = $1
                  AND character_id = (
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
    ) -> Result<bool, ServerError> {
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
    ) -> Result<(), ServerError> {
        let character_id = character.character_id()?;

        sqlx::query!("
            DELETE FROM logins WHERE character_id = $1
        ", *character_id)
        .execute(&self.pool)
        .await?;

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
    #[instrument(err)]
    async fn is_alt(
        &self,
        token: &str
    ) -> Result<bool, ServerError> {
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
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Extension::<AuthService>::from_request(req)
            .await
            .map(|Extension(x)| x)
            .map_err(ServerError::FromReqestError)
    }
}
