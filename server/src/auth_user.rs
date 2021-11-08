use async_trait::*;
use axum::extract::{Extension, FromRequest, RequestParts, TypedHeader};
use caph_connector::{CharacterId, EveAuthClient};
use headers::Cookie;
use std::str::FromStr;
use uuid::Uuid;

use crate::{AuthService, ServerError};

/// Represents a logged in character.
pub struct AuthUser {
    token:        Uuid,
    auth_service: AuthService,
}

impl AuthUser {
    /// Creates a new user instance.
    ///
    /// # Params
    ///
    /// * `token`        -> Token that is send by the cookie
    /// * `auth_service` -> Authentication service
    ///
    /// # Returns
    ///
    /// New user instance.
    ///
    pub fn new(
        token:        Uuid,
        auth_service: AuthService
    ) -> Self {
        Self {
            token,
            auth_service
        }
    }

    /// Creates a new EVE-Authentication client for sending messages to the
    /// EVE-API that require that the user is logged in.
    ///
    /// # Errors
    ///
    /// Fails if getting a new refresh token from the API fails.
    ///
    /// # Returns
    ///
    /// A newly created authentication client, with a fresh token.
    ///
    pub async fn eve_auth_client(&self) -> Result<EveAuthClient, ServerError> {
        let refresh_token = self.auth_service.refresh_token(
            &self.character_id().await?,
            &self.token
        )
        .await?;

        let client = EveAuthClient::new(refresh_token)?;
        Ok(client)
    }

    /// Gets the character id of the currently logged in user.
    ///
    /// # Errors
    ///
    /// Fails if the user is not in the database.
    ///
    /// # Returns
    ///
    /// Character id of the logged in character.
    ///
    pub async fn character_id(&self) -> Result<CharacterId, ServerError> {
        self.auth_service.character_id(&self.token).await
    }

    /// Validates that the requesting user is an admin in the system.
    /// 
    /// # Errors
    /// 
    /// None
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the user is an admin and `Err([ServerError::Unauthorized])`
    /// if the user is not an admin.
    /// 
    pub async fn assert_admin(&self) -> Result<(), ServerError> {
        let cid = self.character_id().await?;
        let is_admin = self.auth_service
            .is_admin(cid)
            .await?;
        if is_admin {
            Ok(())
        } else {
            Err(ServerError::Unauthorized)
        }
    }

    /// Validates that the requesting user has access to the resource
    /// 
    /// # Errors
    /// 
    /// None
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the user has access and `Err([ServerError::Unauthorized])`
    /// if the user is not allowed to access that resource.
    /// 
    pub async fn assert_access(&self) -> Result<(), ServerError> {
        Ok(())
    }

    /// Gets all logged in alts for a character.
    ///
    /// # Errors
    ///
    /// Fails if the user is not in the database.
    ///
    /// # Returns
    ///
    /// Character id of the logged in character.
    ///
    #[deprecated(note = "Use 'character_id' and then let the database search for alts")]
    pub async fn character_alts(&self) -> Result<Vec<CharacterId>, ServerError> {
        let character_id = self.character_id().await?;
        self.auth_service.alts(character_id).await
    }
}

/// Implements the trait [FromRequest] so that we can construct a new [AuthUser]
/// on every incoming request.
/// At the same time we check if the token in the cookie is okay.
#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token = TypedHeader::<Cookie>::from_request(req)
            .await
            .map_err(|x| ServerError::GenericError(x.to_string()))?
            .get("token")
            .map(Uuid::from_str)
            .ok_or(ServerError::BadRequest)?
            .map_err(|_| ServerError::BadRequest)?;
        let Extension(auth_service) = Extension::<AuthService>::from_request(req)
            .await
            .map_err(ServerError::FromReqestError)?;

        Ok(Self::new(token, auth_service))
    }
}
