use caph_connector::{CharacterId, EveAuthClient, AllianceId, CorporationId};
use sqlx::PgPool;
use std::convert::Infallible;
use warp::{Filter, Rejection};
use crate::{Error, AuthError, ProjectId};

#[derive(Clone, Debug)]
pub struct AuthCharacter {
    pool:  PgPool,
    token: String,
}

impl AuthCharacter {
    /// Creates a new [Authorization] instance.
    /// 
    /// # Params
    /// 
    /// * pool  > Open connection to a postgres database
    /// * token > Token token that is send with the request
    /// 
    pub fn new(
        pool:   PgPool,
        token: String,
    ) -> Self {
        Self {
            pool,
            token,
        }
    }

    /// Searches for all [CharacterId]s that have the given ESI-Scope. Only
    /// characters that are connected to the currently logged in character
    /// are returned.
    /// 
    /// # Params
    /// 
    /// * scope > ESI-Scope that the character should have
    /// 
    /// # Errors
    /// 
    /// If the database is not available
    /// 
    /// # Returns
    /// 
    /// List of all [CharacterId]s that have the scope
    /// 
    pub async fn with_scope(
        &self,
        scope: &str
    ) -> Result<Vec<AuthCharacterInfo>, AuthError> {
        let cid = self.character_id().await?;
        let entries = sqlx::query!("
                    SELECT
                        alliance_id,
                        character_id,
                        corporation_id
                    FROM characters
                    WHERE (character_id = $1 OR character_main = $1)
                    AND esi_tokens @> ARRAY[$2]::VARCHAR[]
                ",
                *cid,
                scope
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|x| {
                AuthCharacterInfo {
                    alliance_id:    x.alliance_id.map(|x| x.into()),
                    character_id:   x.character_id.into(),
                    corporation_id: x.corporation_id.into()
                }
            })
            .collect::<Vec<_>>();
        Ok(entries)
    }

    /// Creates a new [EveAuthClient] for the given [CharacterId]
    /// 
    /// # Params
    /// 
    /// * cid > [CharacterId] to generate the refresh token for
    /// 
    /// # Errors
    /// 
    /// - If the database is not available
    /// - If the EVE-API is not available
    /// 
    /// # Returns
    /// 
    /// A authorized [EveAuthClient]
    /// 
    pub async fn eve_auth_client(
        &self,
        cid: &CharacterId
    ) -> Result<EveAuthClient, AuthError> {
        let refresh_token = self
            .refresh_token(cid)
            .await?;
        let client = EveAuthClient::new(refresh_token)
            .map_err(AuthError::CreateAuthClient)?;
        Ok(client)
    }

    /// Gets the [CharacterId] of the logged in character.
    /// 
    /// # Errors
    /// 
    /// - If the database is not available
    /// - If there is no character matching the token
    /// 
    /// # Returns
    /// 
    /// [CharacterId] of the logged in character
    /// 
    pub async fn character_id(
        &self,
    ) -> Result<CharacterId, AuthError> {
        sqlx::query!(r#"
                    SELECT character_id AS "character_id!"
                    FROM logins
                    WHERE token = $1
                "#,
                self.token,
            )
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AuthError::InvalidToken)
            .map(|x| x.character_id.into())
    }

    /// Checks if the current user has access to the given [ProjectId]
    /// 
    /// # Errors
    /// 
    /// - If the database is not avialable
    /// 
    /// # Returns
    /// 
    /// - `true`  -> if the user has access
    /// - `false` -> if the user does not have access
    /// 
    pub async fn has_project_access(
        &self,
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
                self.token
            )

            .fetch_optional(&self.pool)
            .await?;
        Ok(result.is_some())
    }

    /// Gets a fresh refresh token for the EVE-API.
    /// 
    /// # Params
    /// 
    /// * cid > [CharacterId] to generate the refresh token for
    /// 
    /// # Errors
    /// 
    /// - If the database is not available
    /// - If the EVE-API is not available
    /// 
    /// # Returns
    /// 
    /// New refresh token
    /// 
    async fn refresh_token(
        &self,
        cid: &CharacterId,
    ) -> Result<String, AuthError> {
        let refresh_token = sqlx::query!("
                SELECT refresh_token
                FROM logins
                WHERE character_id = $1
            ",
                **cid
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AuthError::InvalidUser)?
            .refresh_token
            .ok_or(AuthError::InvalidUser)?;
        Ok(refresh_token)
    }
}

/// Filter for the API.
/// 
/// # Params
/// 
/// * `pool` > Open connection to postgres
/// 
/// # Errors
/// 
/// - If the cookie key value is not set
/// - If there is a HMAC error from converting the cookie
/// 
/// # Returns
/// 
/// Initialized instance of [Authorization]
/// 
pub fn with_authorization(
    pool:   PgPool,
)  -> impl Filter<Extract = (AuthCharacter,), Error = Rejection> + Clone {
    warp::any()
        .map(move || pool.clone())
        .and(warp::cookie("token"))
        .and_then(|pool: PgPool, token: String| async move {
            if let Ok(token) = crate::utils::recreate_secure_token(token) {
                Ok(AuthCharacter::new(pool, token))
            } else {
                Err(warp::reject::custom(Error::BadRequest))
            }
        })
}

/// Information about a requested character
#[derive(Debug)]
pub struct AuthCharacterInfo {
    pub alliance_id:    Option<AllianceId>,
    pub character_id:   CharacterId,
    pub corporation_id: CorporationId,
}
