use crate::error::EveServerError;
use crate::eve::EveAuthService;

use cachem::v2::ConnectionPool;
use caph_db_v2::{CacheName, CharacterAssetEntry, CharacterBlueprintEntry};
use caph_eve_data_wrapper::{CharacterId, CorporationId, ItemId};
use caph_eve_data_wrapper::EveDataWrapper;
use caph_eve_data_wrapper::ItemLocation;
use serde::Serialize;

/// Service for all character related interfaces
#[derive(Clone)]
pub struct CharacterService {
    pool:     ConnectionPool,
    eve_auth: EveAuthService,
    eve_data: EveDataWrapper,
}

impl CharacterService {
    /// Creates a new instance
    pub fn new(
        pool: ConnectionPool,
        eve_auth: EveAuthService,
        eve_data: EveDataWrapper,
    ) -> Self {
        Self {
            pool,
            eve_auth,
            eve_data,
        }
    }

    pub async fn assets(
        &self,
        token: &str
    ) -> Result<Vec<CharacterAssetEntry>, EveServerError> {
        let mut con = self.pool.acquire().await?;

        // Check if the user exists
        let _ = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;

        let keys = con
            .keys::<_, ItemId>(CacheName::CharacterAsset)
            .await?;
        let assets = con
            .mget::<_, _, CharacterAssetEntry>(CacheName::CharacterAsset, keys)
            .await?
            .into_iter()
            .flatten()
            .collect::<Vec<CharacterAssetEntry>>();
        Ok(assets)
    }

    /// Resolves all blueprints for a character and its alts
    ///
    /// # Params
    ///
    /// `token` -> Cookie from the requesting main
    ///
    /// # Returns
    ///
    /// List of all blueprints of all characters
    ///
    pub async fn blueprints(
        &self,
        token: String
    ) -> Result<Vec<CharacterBlueprintEntry>, EveServerError> {
        let mut con = self.pool.acquire().await?;

        // Check if the user exists
        let _ = self
            .eve_auth
            .lookup(&token)
            .await?
            .ok_or(EveServerError::InvalidUser)?;

        let keys = con
            .keys::<_, ItemId>(CacheName::CharacterBlueprint)
            .await?;
        let bps = con
            .mget::<_, _, CharacterBlueprintEntry>(CacheName::CharacterBlueprint, keys)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<CharacterBlueprintEntry>>();
        Ok(bps)
    }

    /// Gets a blueprint by its [ItemId]
    ///
    /// # Params
    ///
    /// * `bpid` -> [ItemId] of the requested blueprint
    ///
    /// # Result
    ///
    /// Either a `Some(CharacterBlueprintEntry)` or `None` if a blueprint with
    /// that id does not exist
    ///
    pub async fn blueprint_by_item_id(
        &self,
        bpid: ItemId,
    ) -> Result<Option<CharacterBlueprintEntry>, EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .get::<_, _, CharacterBlueprintEntry>(CacheName::CharacterBlueprint, bpid)
            .await
            .map_err(Into::into)
    }

    /// Tries to resolve an items location
    ///
    /// # Params
    ///
    /// `id` -> LocationId of the item
    ///
    /// # Returns
    ///
    /// Location information about the item
    ///
    pub async fn item_location(
        &self,
        token: String,
        id:    u64
    ) -> Result<Option<ItemLocation>, EveServerError> {
        let charater_service = self.eve_data.character().await?;
        let user = self.eve_auth.refresh_token(&token).await?;

        charater_service
            .item_location(&user.access_token, id)
            .await
            .map_err(|_| EveServerError::InvalidUser)
    }

    /// Attempts to figure out who the requesting user is
    ///
    /// # Params
    ///
    /// `token` -> Cookie from the user
    ///
    /// # Returns
    ///
    /// Struct containing the name, protrait, corp icon and alliance icon
    ///
    pub async fn whoami(&self, token: String) -> Result<WhoAmI, EveServerError> {
        let charater_service = self.eve_data.character().await?;
        let user = self.eve_auth.refresh_token(&token).await?;

        let whoami = charater_service
            .character(&user.access_token, user.user_id)
            .await;
        if let Ok(x) = whoami {
            Ok(WhoAmI::new(user.user_id, x))
        } else {
            Err(EveServerError::InvalidUser)
        }
    }

    /// Gets all information about an character
    ///
    /// # Params
    ///
    /// `token` -> Cookie of the requesting character
    ///
    /// # Returns
    ///
    /// Character information with all information about him and its alts
    ///
    pub async fn info(&self, token: String) -> Result<Character, EveServerError> {
        let user = self.eve_auth.lookup(&token).await?;

        let user = if let Some(user) = user {
            user
        } else {
            return Err(EveServerError::InvalidUser);
        };

        let mut aliase = Vec::new();
        for alias in user.aliase {
            let info = self
                .character_info(alias.access_token, alias.user_id)
                .await?;
            aliase.push(info);
        }

        let mut character = self.character_info(
            user.access_token,
            user.user_id
        ).await?;
        character.aliase = aliase;

        Ok(character)
    }

    /// Builds the character information together
    async fn character_info(
        &self,
        access_token: String,
        uid: CharacterId
    ) -> Result<Character, EveServerError> {
        let character_service = self.eve_data.character().await?;
        let character = character_service
            .character(&access_token, uid)
            .await?;

        let alliance_name = if let Some(x) = character.alliance_id {
            Some(character_service.alliance_name(x).await?)
        } else {
            None
        };
        let corp_name = character_service
            .corporation_name(character.corporation_id.into())
            .await?;

        let character = Character::new(
            uid,
            character,
            corp_name,
            alliance_name,
            Vec::new()
        );
        Ok(character)
    }
}

#[derive(Debug, Serialize)]
pub struct WhoAmI {
    /// Name of the user
    name:             String,
    /// https://images.evetech.net/characters/2117441999/portrait?size=1024
    portrait:         String,
    /// https://images.evetech.net/corporations/692480993/logo?size=1024
    corporation_icon: String,
    /// https://images.evetech.net/alliances/99003214/logo?size=1024
    alliance_icon:    Option<String>,
    /// Id of the user
    user_id:          CharacterId,
    /// Id of the users corporation
    corp_id:          CorporationId,
}

impl WhoAmI {
    pub fn new(
        user_id:   CharacterId,
        character: caph_eve_data_wrapper::Character
    ) -> Self {
        let alliance = if let Some(x) = character.alliance_id {
            Some(format!( "https://images.evetech.net/alliances/{}/logo?size=1024", x))
        } else { None };

        WhoAmI {
            name: character.name,
            portrait: format!(
                "https://images.evetech.net/characters/{}/portrait?size=1024",
                user_id
            ),
            corporation_icon: format!(
                "https://images.evetech.net/corporations/{}/logo?size=1024",
                character.corporation_id
            ),
            alliance_icon: alliance,
            user_id,
            corp_id: character.corporation_id.into()
        }
    }
}

/// Represents a character with all its information
#[derive(Debug, Serialize)]
pub struct Character {
    name:          String,
    portrait:      String,
    corp:          String,
    corp_icon:     String,
    alliance:      Option<String>,
    alliance_icon: Option<String>,
    aliase:        Vec<Character>,
    user_id:       CharacterId,
    corp_id:       CorporationId,
}

impl Character {
    /// Creates a new character
    pub fn new(
        user_id: CharacterId,
        character: caph_eve_data_wrapper::Character,
        corp: String,
        alliance: Option<String>,
        aliase: Vec<Character>
    ) -> Self {
        let alliance_icon = if let Some(x) = character.alliance_id {
            Some(format!( "https://images.evetech.net/alliances/{}/logo?size=1024", x))
        } else { None };

        Self {
            name: character.name,
            portrait: format!(
                "https://images.evetech.net/characters/{}/portrait?size=1024",
                user_id
            ),
            corp,
            corp_icon: format!(
                "https://images.evetech.net/corporations/{}/logo?size=1024",
                character.corporation_id
            ),
            alliance,
            alliance_icon,
            aliase,
            user_id,
            corp_id: character.corporation_id.into()
        }
    }
}

