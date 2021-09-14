use crate::error::EveServerError;

use cachem::ConnectionPool;
use caph_db::{CacheName, UserEntry};
use caph_eve_data_wrapper::{AllianceId, CharacterId, CorporationId, EveDataWrapper, EveOAuthUser};
use caph_eve_data_wrapper::{EveClient, Url};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Describes different type of session logins
#[derive(Debug, PartialEq)]
enum SessionType {
    /// Login process with the main account
    Main,
    /// Login process with an alt
    /// Contains the user id of the main
    Alt(CharacterId),
    /// Logged in user
    /// Contains the user id of the main
    Logged(CharacterId)
}

#[derive(Clone)]
pub struct EveAuthService {
    pool:      ConnectionPool,
    eve_data:  EveDataWrapper,
    sessions:  Arc<Mutex<HashMap<Uuid, SessionType>>>,
}

impl EveAuthService {
    /// Creates a new instance
    pub fn new(
        pool:      ConnectionPool,
        eve_data:  EveDataWrapper,
    ) -> Self {
        Self {
            pool,
            eve_data,
            sessions: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    /// Performs the last eve auth step
    ///
    /// # Params
    ///
    /// `code`  -> Code that was send when starting the auth process
    /// `state` -> Our unique identifier
    ///
    /// # Returns
    ///
    /// Returns an optional ChaCha20 64 chars generated token which should be
    /// used as cookie.
    /// This token is only returned when the logged in character is a main.
    ///
    pub async fn auth(
        &self,
        code:  String,
        state: Uuid
    ) -> Result<Option<Uuid>, EveServerError> {
        let session_entry = {
            // Make sure that the code is valid
            // If the code is valid, remove it from the map
            //
            // Also keep the session as short as possible
            let mut session = self.sessions
                .lock()
                .await;
            if session.contains_key(&state) {
                // Unwrap is save here, because we made sure the entry exists
                session.remove(&state).unwrap()
            } else {
                return Err(EveServerError::InvalidUser);
            }
        };

        let user = EveClient::retrieve_authorization_token(&code).await?;

        if session_entry == SessionType::Main {
            let user_token = self.generate_key();
            self.sessions
                .lock()
                .await
                .insert(user_token.clone(), SessionType::Logged(user.user_id));

            self.save_login(&user_token, user).await?;
            Ok(Some(user_token))
        } else if let SessionType::Alt(uid) = session_entry {
            let main = self
                .pool
                .acquire()
                .await?
                .get::<_, _, UserEntry>(CacheName::User, uid)
                .await?;

            if let Some(main) = main {
                self.add_alt(main, user).await?;
                Ok(None)
            } else {
                Err(EveServerError::InvalidUser)
            }
        } else {
            Err(EveServerError::InvalidUser)
        }
    }

    /// Creates a new unique code and returns a eve login auth uri
    /// This function is only for main accounts
    ///
    /// # Returns
    ///
    /// Uri to the eve auth server
    ///
    pub async fn login(&self) -> Result<Url, EveServerError> {
        let key = self.generate_key();
        self.sessions.lock().await.insert(key.clone(), SessionType::Main);

        EveClient::eve_auth_url(&key.to_string())
            .map_err(Into::into)
    }

    /// Creates a new unique code and returns a eve login auth uri
    /// This function is only for alt accounts
    ///
    /// # Params
    ///
    /// `token` -> Token of the cookie from the main user
    ///
    /// # Returns
    ///
    /// Uri to the eve auth server
    ///
    pub async fn login_alt(&self, token: &Uuid) -> Result<Url, EveServerError> {
        let user = self.lookup(token).await?;

        if let Some(x) = user {
            let key = self.generate_key();
            self.sessions.lock().await.insert(key.clone(), SessionType::Alt(x.user_id));

            EveClient::eve_auth_url(&key.to_string())
                .map_err(Into::into)
        } else {
            Err(EveServerError::InvalidUser)
        }
    }

    /// Looksup a user by its id
    ///
    /// # Params
    ///
    /// `token` -> Token of the user to lookup
    ///
    pub async fn lookup(
        &self,
        token: &Uuid,
    ) -> Result<Option<UserEntry>, EveServerError> {
        let uid = self
            .sessions
            .lock()
            .await;
        let uid = uid.get(token);

        if let Some(SessionType::Logged(x)) = uid {
            self
                .pool
                .acquire()
                .await?
                .get::<_, _, UserEntry>(CacheName::User, *x)
                .await
                .map_err(Into::into)
        } else {
            Ok(None)
        }
    }

    /// Requests a new refresh token from the eve auth server
    ///
    /// # Param
    ///
    /// `token` -> Token of the user
    ///
    /// # Returns
    ///
    /// New oauth user
    ///
    pub async fn refresh_token(&self, token: &Uuid) -> Result<EveOAuthUser, EveServerError> {
        let oauth = self
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;
        let oauth = EveClient::retrieve_refresh_token(&oauth.refresh_token)
            .await
            .map_err(EveServerError::from)?;

        self.save_login(token, oauth.clone()).await?;

        Ok(oauth)
    }

    /// Requests a new refresh token for an alt from the eve auth server
    ///
    /// # Param
    ///
    /// `token` -> Token of the user
    /// `uid`   -> Userid of the alt
    ///
    /// # Returns
    ///
    /// New oauth user
    ///
    pub async fn refresh_token_alt(
        &self,
        token: &Uuid,
        uid:   CharacterId,
    ) -> Result<EveOAuthUser, EveServerError> {
        let oauth = self
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;
        let oauth = oauth
            .aliase
            .iter()
            .find(|x| x.user_id == uid)
            .ok_or(EveServerError::InvalidUser)?;
        let oauth = EveClient::retrieve_refresh_token(&oauth.refresh_token)
            .await
            .map_err(EveServerError::from)?;

        self.save_login_alt(token, oauth.clone()).await?;

        Ok(oauth)
    }

    /// Saves the main character in the database
    ///
    /// # Params
    ///
    /// `character` -> Character with access_token and refresh_token
    ///
    async fn save_login(
        &self,
        token:     &Uuid,
        character: EveOAuthUser
    ) -> Result<(), EveServerError> {
        if let Some(x) = self.lookup(&token).await? {
            let user = UserEntry {
                access_token: character.access_token,
                refresh_token: character.refresh_token,
                ..x
            };
            self.save_user(user).await?;
        } else {
            let character_service = self.eve_data.character().await?;
            let character_info = character_service
                .character(character.user_id)
                .await?;
            let alliance_name = self.alliance_name(character.alliance_id).await?;
            let corp_name = self.corp_name(character.corp_id).await?;

            let user = UserEntry {
                access_token: character.access_token,
                refresh_token: character.refresh_token,
                alliance_id: character.alliance_id,
                alliance_name,
                user_id: character.user_id,
                user_name: character_info.name,
                corp_id: character.corp_id,
                corp_name,
                aliase: Vec::new(),
            };
            self.save_user(user).await?;
        }

        Ok(())
    }

    /// Updates an alt of a main char
    ///
    /// # Params
    ///
    /// `character` -> Character with access_token and refresh_token
    ///
    async fn save_login_alt(
        &self,
        token:     &Uuid,
        character: EveOAuthUser
    ) -> Result<(), EveServerError> {
        let character_service = self.eve_data.character().await?;
        let mut main = self
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;

        for alias in main
                        .aliase
                        .iter_mut()
                        .find(|x| x.user_id == character.user_id) {

            let character_info = character_service
                .character(character.user_id)
                .await?;
            let alliance_name = self.alliance_name(character.alliance_id).await?;
            let corp_name = self.corp_name(character.corp_id).await?;

            *alias = UserEntry {
                access_token: character.access_token.clone(),
                refresh_token: character.refresh_token.clone(),
                alliance_id: character.alliance_id,
                alliance_name,
                user_id: character.user_id,
                user_name: character_info.name,
                corp_id: character.corp_id,
                corp_name,
                aliase: Vec::new(),
            };
        }

        self.save_user(main).await?;
        Ok(())
    }

    /// Saves the given user entry in the database
    ///
    /// # Params
    ///
    /// `user` -> User to save
    ///
    async fn save_user(
        &self,
        user: UserEntry
    ) -> Result<(), EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .set(CacheName::User, user.user_id, user)
            .await
            .map_err(Into::into)
    }

    /// Adds a new alt to a main
    ///
    /// # Params
    ///
    /// `main` -> User entry of the main account
    /// `alt`  -> User entry of the alt account
    ///
    async fn add_alt(
        &self,
        main: UserEntry,
        alt:  EveOAuthUser,
    ) -> Result<(), EveServerError> {
        let character_service = self.eve_data.character().await?;
        let character_info = character_service
            .character(alt.user_id)
            .await?;
        let alliance_name = self.alliance_name(alt.alliance_id).await?;
        let corp_name = self.corp_name(alt.corp_id).await?;
        let alt = UserEntry {
            access_token: alt.access_token,
            refresh_token: alt.refresh_token,
            alliance_id: alt.alliance_id,
            alliance_name,
            user_id: alt.user_id,
            user_name: character_info.name,
            corp_id: alt.corp_id,
            corp_name,
            aliase: Vec::new(),
        };

        let mut main = main;
        main.aliase.push(alt);
        self.save_user(main).await
    }

    async fn alliance_name(
        &self,
        aid: AllianceId
    ) -> Result<String, EveServerError> {
        let character_service = self.eve_data.character().await?;
        Ok(character_service.alliance_name(aid).await?)
    }

    async fn corp_name(
        &self,
        cid: CorporationId
    ) -> Result<String, EveServerError> {
        let character_service = self.eve_data.character().await?;
        Ok(character_service.corporation_name(cid).await?)
    }

    /// Could be safer, but its good enough for out use case
    fn generate_key(&self) -> Uuid {
        let val: u128 = ChaCha20Rng::from_entropy().gen();
        Uuid::from_u128(val)
    }
}

