use std::str::FromStr;

use crate::error::{ServerError, internal_error};

use async_trait::async_trait;
use axum::extract::{Extension, FromRequest, RequestParts, TypedHeader};
use axum::http::{StatusCode, Uri};
use caph_eve_data_wrapper::{CharacterId, EveClient, EveOAuthUser};
use headers::Cookie;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

const EVE_SCOPE: &[&'static str] = &[
    "publicData",
    "esi-assets.read_assets.v1",
    "esi-characters.read_agents_research.v1",
    "esi-characters.read_blueprints.v1",
    "esi-characterstats.read.v1",
    "esi-fittings.read_fittings.v1",
    "esi-fittings.write_fittings.v1",
    "esi-industry.read_character_jobs.v1",
    "esi-industry.read_corporation_jobs.v1",
    "esi-industry.read_character_mining.v1",
    "esi-markets.read_character_orders.v1",
    "esi-markets.structure_markets.v1",
    "esi-planets.manage_planets.v1",
    "esi-search.search_structures.v1",
    "esi-skills.read_skillqueue.v1",
    "esi-skills.read_skills.v1",
    "esi-universe.read_structures.v1",
    "esi-wallet.read_character_wallet.v1",
];

#[derive(Clone)]
pub struct EveService {
    pool: PgPool
}

impl EveService {
    pub fn new(pool: PgPool) -> Self {
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
        let character = EveClient::retrieve_authorization_token(&code).await;
        self.save_login(&token, character?).await?;

        if self.is_alt(&token).await? {
            Ok(None)
        } else {
            Ok(Some(token))
        }
    }

    /// Creates a new unique code and returns a eve login auth uri
    /// This function is only for main accounts
    ///
    /// # Returns
    ///
    /// Uri to the eve auth server
    ///
    pub async fn login(&self) -> Result<Uri, ServerError> {
        let token = sqlx::query!(
            "INSERT INTO login DEFAULT VALUES RETURNING token"
        )
        .fetch_one(&self.pool)
        .await?
        .token;

        let url = EveClient::eve_auth_uri(
            &token.to_string(),
        )?;
        let uri = url
            .to_string()
            .parse::<Uri>()
            .unwrap();
        Ok(uri)
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
        token: Uuid
    ) -> Result<Uri, ServerError> {
        let character = sqlx::query!("
                SELECT character_id
                FROM login
                WHERE token = $1
            ",
            token
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(ServerError::InvalidUser)?
        .character_id;

        let token = sqlx::query!("
            INSERT INTO login (character_main)
            VALUES ($1)
            RETURNING token
        ", character)
        .fetch_one(&self.pool)
        .await?
        .token;

        EveClient::eve_auth_uri(&token.to_string()).map_err(Into::into)
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
    async fn character_id(
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
                Ok((id as u32).into())
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
        character: EveOAuthUser
    ) -> Result<(), ServerError> {
        sqlx::query!("
            DELETE FROM login WHERE character_id = $1
        ", *character.user_id as i32)
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
            *character.user_id as i32,
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

/// Login query that is send by the eve auth servers
#[derive(Debug, Deserialize)]
pub struct EveAuthQuery {
    pub code:  String,
    pub state: Uuid,
}

pub struct LoggedInCharacter {
    token:         Uuid,
    eve_service:   EveService,
}

impl LoggedInCharacter {
    pub fn new(token: Uuid, eve_service: EveService) -> Self {
        Self {
            token,
            eve_service
        }
    }

    /// Gets a valid `access_token` for the eve api
    ///
    /// # Returns
    ///
    /// Valid `access_token` that can be used for requests to the eve api
    ///
    pub async fn get_token(&self) -> Result<String, ServerError> {
        Ok(String::new())
    }

    /// Gets the character id of requesting character
    ///
    /// # Returns
    ///
    /// Character id of the logged in character
    ///
    pub async fn character_id(&self) -> Result<CharacterId, ServerError> {
        self.eve_service.character_id(&self.token).await
    }

    /// Gets all logged in alts for a character
    ///
    /// # Returns
    ///
    /// Character id of the logged in character
    ///
    pub async fn character_alts(&self) -> Result<Vec<CharacterId>, ServerError> {
        let character_id = self.character_id().await?;
        self.eve_service.alts(character_id).await
    }
}

#[async_trait]
impl<B> FromRequest<B> for LoggedInCharacter
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token = TypedHeader::<Cookie>::from_request(req)
            .await
            .map_err(internal_error)?
            .get("token")
            .map(Uuid::from_str)
            .ok_or((StatusCode::BAD_REQUEST, "".into()))?
            .map_err(|_| (StatusCode::BAD_REQUEST, "".into()))?;
        let Extension(eve_service) = Extension::<EveService>::from_request(req)
            .await
            .map_err(internal_error)?;

        Ok(Self::new(token, eve_service))
    }
}
