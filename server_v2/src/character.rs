use crate::error::ServerError;

use caph_eve_data_wrapper::{AllianceId, CharacterId, CorporationId, EveDataWrapper};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone)]
pub struct CharacterService {
    pool: PgPool
}

impl CharacterService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    /// Gets a list of alts for the given [CharacterId]
    ///
    /// # Params
    ///
    /// * `cid` -> [CharacterId] of the requesting character
    ///
    /// # Returns
    ///
    /// List of alt characters
    ///
    pub async fn alts(
        &self,
        cid: CharacterId
    ) -> Result<Vec<Character>, ServerError> {
        let alts = sqlx::query!("
                SELECT DISTINCT character_id
                FROM login
                WHERE character_main = $1 AND character_id IS NOT NULL
            ", *cid as i32)
            .fetch_all(&self.pool)
            .await?;

        let mut characters = Vec::new();
        for alt in alts {
            let character_id = alt.character_id;

            if let Some(x) = character_id {
                let character_id = (x as u32).into();
                let character = self.info(character_id, Some(cid)).await?;
                characters.push(character);
            }
        }

        Ok(characters)
    }

    /// Fetches information either from the database or the eve servers
    ///
    /// # Params
    ///
    /// `cid`  -> Character id of the character to fetch
    /// `main` -> Optional character id of the main account
    ///
    /// # Returns
    ///
    /// Alliance, character and corporation information
    ///
    pub async fn info(
        &self,
        cid:  CharacterId,
        main: Option<CharacterId>
    ) -> Result<Character, ServerError> {
        let character = sqlx::query!("
            SELECT
                c.alliance_id,
                c.alliance_name,
                c.character_id,
                c.character_name,
                c.corporation_id,
                c.corporation_name
            FROM login l
            JOIN character c
            ON l.character_id = c.character_id
            WHERE c.character_id = $1;
        ", *cid as i32)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(x) = character {
            let character = Character::new(
                x.alliance_name,
                (x.alliance_id as u32).into(),
                x.character_name,
                (x.character_id as u32).into(),
                x.corporation_name,
                (x.corporation_id as u32).into(),
            );
            Ok(character)
        } else {
            let character = self.eve_character_info(cid).await?;
            self.save(&character, main).await?;
            Ok(character)
        }
    }

    /// Saves the character information in the database
    ///
    /// # Params
    ///
    /// * `character` -> All information about the character
    /// * `main`      -> Optional main character id
    ///
    async fn save(
        &self,
        character: &Character,
        main:      Option<CharacterId>
    ) -> Result<(), ServerError> {
        sqlx::query!("
                INSERT INTO character
                (
                    alliance_id, alliance_name,
                    character_id, character_name,
                    corporation_id, corporation_name,
                    character_main
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
            *character.alliance_id as i32,
            character.alliance,
            *character.character_id as i32,
            character.character,
            *character.corporation_id as i32,
            character.corporation,
            main.map(|x| *x as i32)
        )
        .execute(&self.pool)
        .await
        .map(drop)
        .map_err(Into::into)
    }

    /// Fetches all character infos from eve
    ///
    /// # Params
    ///
    /// * `cid` -> Character id to fetch
    ///
    /// # Returns
    ///
    /// Alliance, character and corporation information
    ///
    async fn eve_character_info(
        &self,
        cid: CharacterId
    ) -> Result<Character, ServerError> {
        let character_service = EveDataWrapper::new()
            .await?
            .character()
            .await?;
        let character = character_service
            .character(cid)
            .await?;

        let alliance_id = character.alliance_id.unwrap();
        let alliance = character_service
            .alliance_name(alliance_id)
            .await?;

        let corporation_id = character.corporation_id;
        let corporation = character_service
            .corporation_name(corporation_id.into())
            .await?;

        Ok(Character::new(
            alliance,
            alliance_id,
            character.name,
            cid,
            corporation,
            corporation_id
        ))
    }
}

/// Represents a character with all its information
#[derive(Debug, Serialize)]
pub struct Character {
    alliance:         String,
    alliance_icon:    String,
    alliance_id:      AllianceId,
    character:        String,
    character_id:     CharacterId,
    character_icon:   String,
    corporation:      String,
    corporation_icon: String,
    corporation_id:   CorporationId,
}

impl Character {
    pub fn new(
        alliance: String,
        alliance_id: AllianceId,
        character: String,
        character_id: CharacterId,
        corporation: String,
        corporation_id: CorporationId
    ) -> Self {
        Self {
            alliance,
            alliance_id: alliance_id.into(),
            alliance_icon: format!(
                "https://images.evetech.net/alliances/{}/logo?size=1024",
                alliance_id
            ),
            character,
            character_id: character_id.into(),
            character_icon: format!(
                "https://images.evetech.net/characters/{}/portrait?size=1024",
                character_id
            ),
            corporation,
            corporation_id: corporation_id.into(),
            corporation_icon: format!(
                "https://images.evetech.net/corporations/{}/logo?size=1024",
                corporation_id
            ),
        }
    }
}

